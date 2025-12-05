use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use log;
use rand::Rng;

#[derive(Clone)]
pub struct Unit {
    pub name: String,
    pub hp: u128,
    pub max_hp: u128,
    pub atk: u128,
    pub def: u128,
}

impl Unit {
    pub fn test(num: usize) -> Vec<Vec<Unit>> {
        // 1. 生成 200 个 Unit
        let units: Vec<Unit> = (1..=num)
            .map(|i| Unit {
                name: format!("Unit {}", i),
                hp: 100,
                max_hp: 100,
                atk: i as u128,
                def: i as u128,
            })
            .collect();

        // 2. 根据数量决定列数
        let n_cols = determine_columns(units.len());
        if n_cols == 0 {
            return vec![];
        }

        // 3. 计算每列应有多少行（向上取整）
        let rows = units.len().div_ceil(n_cols);

        // 4. 按"填满第一列再第二列"的方式分组
        let mut columns: Vec<Vec<Unit>> = vec![Vec::new(); n_cols];
        for (index, unit) in units.into_iter().enumerate() {
            let col_idx = index / rows;
            if col_idx < n_cols {
                columns[col_idx].push(unit);
            } else {
                // 安全兜底：极端情况放最后一列
                columns[n_cols - 1].push(unit);
            }
        }

        columns // ← 关键：这是尾表达式，无分号，返回 Vec<Vec<Unit>>
    }
}

// 根据单位数量决定显示的列数
pub fn determine_columns(count: usize) -> usize {
    match count {
        1..=3 => 1,
        4..=9 => 2,
        10..=29 => 3,
        30..=59 => 4,
        60..=119 => 5,
        _ => 6,
    }
}

// 辅助函数：移除每列中 hp <= 0 的单位（从头部开始）
fn remove_dead_front(units: &mut Vec<Vec<Unit>>) {
    for col in units.iter_mut() {
        // 从前往后移除死亡单位（因为只关心"第一行"）
        while !col.is_empty() && col[0].hp == 0 {
            col.remove(0);
        }
    }
    // 清理空列（可选）
    units.retain(|col| !col.is_empty());
}

// 检查是否还有存活单位
fn has_any_alive(units: &[Vec<Unit>]) -> bool {
    units.iter().any(|col| !col.is_empty())
}

#[derive(Default)]
pub struct StartBattle {
    pub enemy_units: Arc<Mutex<Vec<Vec<Unit>>>>,
    pub friendly_units: Arc<Mutex<Vec<Vec<Unit>>>>,
}

impl StartBattle {
    pub fn new(enemy_units: Vec<Vec<Unit>>, friendly_units: Vec<Vec<Unit>>) -> Self {
        Self {
            enemy_units: Arc::new(Mutex::new(enemy_units)),
            friendly_units: Arc::new(Mutex::new(friendly_units)),
        }
    }

    /// 启动战斗（在后台线程运行）
    pub fn run(&self) {
        // 克隆 Arc，这样线程可以独立持有引用
        let enemy = Arc::clone(&self.enemy_units);
        let friendly = Arc::clone(&self.friendly_units);

        thread::spawn(move || {
            let mut rng = rand::rng();

            loop {
                // 使用 enemy 和 friendly（它们是 Arc<Mutex<...>>）
                let mut e_guard = enemy.lock().expect("锁敌方失败");
                let mut f_guard = friendly.lock().expect("锁友方失败");

                if !has_any_alive(&e_guard) {
                    log::info!("🎉 友方胜利！");
                    break;
                }
                if !has_any_alive(&f_guard) {
                    log::info!("💀 敌方胜利！");
                    break;
                }

                // 提取所有前线单位（第一排）
                let mut e_front_indices: Vec<usize> = vec![]; // 敌方前线单位的列索引
                let mut f_front_indices: Vec<usize> = vec![]; // 友方前线单位的列索引

                // 收集敌方的所有第一排单位的索引
                for col_idx in 0..e_guard.len() {
                    if !e_guard[col_idx].is_empty() {
                        e_front_indices.push(col_idx);
                    }
                }

                // 收集友方的所有第一排单位的索引
                for col_idx in 0..f_guard.len() {
                    if !f_guard[col_idx].is_empty() {
                        f_front_indices.push(col_idx);
                    }
                }

                // 友方单位随机攻击敌方单位
                for &f_col_idx in &f_front_indices {
                    if !e_front_indices.is_empty() {
                        let target_idx = rng.random_range(0..e_front_indices.len());
                        let e_col_idx = e_front_indices[target_idx];

                        let f_unit = &mut f_guard[f_col_idx][0];
                        let e_unit = &mut e_guard[e_col_idx][0];

                        // 计算伤害
                        let damage = if f_unit.atk > e_unit.def {
                            f_unit.atk - e_unit.def
                        } else {
                            1 // 保证至少有1点伤害
                        };
                        e_unit.hp = e_unit.hp.saturating_sub(damage);
                    }
                }

                // 敌方单位随机攻击友方单位
                for &e_col_idx in &e_front_indices {
                    if !f_front_indices.is_empty() {
                        let target_idx = rng.random_range(0..f_front_indices.len());
                        let f_col_idx = f_front_indices[target_idx];

                        let e_unit = &mut e_guard[e_col_idx][0];
                        let f_unit = &mut f_guard[f_col_idx][0];

                        // 计算伤害
                        let damage = if e_unit.atk > f_unit.def {
                            e_unit.atk - f_unit.def
                        } else {
                            1 // 保证至少有1点伤害
                        };
                        f_unit.hp = f_unit.hp.saturating_sub(damage);
                    }
                }

                // 清理死亡单位
                remove_dead_front(&mut e_guard);
                remove_dead_front(&mut f_guard);

                drop(e_guard);
                drop(f_guard);

                thread::sleep(Duration::from_millis(500));
            }
        });
    }
}
