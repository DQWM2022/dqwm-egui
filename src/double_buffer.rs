use parking_lot::Mutex;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};

#[derive(Debug)]
pub struct DoubleBuffer<T: Clone> {
    front: AtomicPtr<Arc<T>>, // UI 线程无锁读
    back: Mutex<T>,           // 游戏线程独占写
}

// 手动 Send/Sync 是安全的：Arc 内部已处理，Mutex 也是 Sync
unsafe impl<T: Clone + Send> Send for DoubleBuffer<T> {}
unsafe impl<T: Clone + Sync> Sync for DoubleBuffer<T> {}

impl<T: Clone> DoubleBuffer<T> {
    /// 创建双缓冲，initial 作为第一份前台快照
    pub fn new(initial: T) -> Self {
        let front_arc = Arc::new(initial.clone());
        let front_ptr = Box::into_raw(Box::new(front_arc));
        Self {
            front: AtomicPtr::new(front_ptr),
            back: Mutex::new(initial),
        }
    }

    /// 游戏线程：拿到后台可变引用，随意修改
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut back = self.back.lock();
        f(&mut back);
    }

    /// 游戏线程：把后台提交为新的前台快照（交换）
    pub fn swap(&self) {
        // 1. 克隆后台数据
        let new_data = {
            let back = self.back.lock();
            back.clone()
        };
        // 2. 装进 Arc
        let new_arc = Arc::new(new_data);
        let new_ptr = Box::into_raw(Box::new(new_arc));

        // 3. 原子替换前台指针
        let old_ptr = self.front.swap(new_ptr, Ordering::Release);

        // 4. 释放旧 Arc
        unsafe {
            let _ = Box::from_raw(old_ptr); // Arc 自动 drop
        }
    }

    /// UI 线程：无锁获取当前前台快照
    /// 返回 Arc<T>，想读几次就克隆几次，永不阻塞
    pub fn read(&self) -> Arc<T> {
        let ptr = self.front.load(Ordering::Acquire);
        unsafe { Arc::clone(&*ptr) }
    }
}
