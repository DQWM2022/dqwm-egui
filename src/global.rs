use std::sync::{LazyLock, OnceLock, atomic::AtomicUsize};
use tokio::runtime::Runtime;

pub fn global_tokio_runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("无法创建全局Tokio运行时环境！")
    })
}

pub struct GlobalConfig {
    pub max_enemy_cols: AtomicUsize, // 战斗页面敌方最大列数
    pub max_ally_cols: AtomicUsize,  // 战斗页面我方最大列数
}
pub static CONFIG: LazyLock<GlobalConfig> = LazyLock::new(|| GlobalConfig {
    max_enemy_cols: AtomicUsize::new(10),
    max_ally_cols: AtomicUsize::new(10),
});
