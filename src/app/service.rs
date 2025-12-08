use rand::Rng;
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock, mpsc::Receiver},
    thread,
    time::{Duration, Instant},
};

use crate::{GameCommand, app::Unit};

#[derive(Default, Debug)]
pub struct Army {
    pub enemy_units: Vec<VecDeque<Unit>>,
    pub friendly_units: Vec<VecDeque<Unit>>,
}

// 游戏服务
#[derive(Debug)]
pub struct GameService {
    // 处理资源
    // 处理事件
    // 处理战斗
    // 可选：是否运行标志
    cmd_rx: Receiver<GameCommand>, // 接收消息
    pub running: bool,
    pub battle_running: bool, // 只处理战斗
    pub army: Arc<RwLock<Army>>,
}

impl GameService {
    pub fn new(cmd_rx: Receiver<GameCommand>, army: Arc<RwLock<Army>>) -> Self {
        Self {
            cmd_rx,
            running: false,
            battle_running: false,
            army,
        }
    }

    pub fn army_message(&mut self, enemy_num: usize, friendly_num: usize) {
        // 先生成数据（不持有锁！）
        let enemy_units = Unit::test(enemy_num);
        let friendly_units = Unit::test(friendly_num);
        // 再获取写锁，一次性写入
        if let Ok(mut army) = self.army.write() {
            army.enemy_units = enemy_units;
            army.friendly_units = friendly_units;
        } else {
            log::error!("无法获取军队的写锁！");
            return;
        }
        log::info!(
            "⚔️ 战斗信息：敌方 {} 人，友方 {} 人",
            enemy_num,
            friendly_num
        );
    }

    fn start_battle(&mut self) {
        // 检查，是否在战斗中，
        if self.battle_running {
            log::error!("战斗已开始！请勿重复启动！");
            return;
        }
        self.battle_running = true
    }

    // 启动服务（消耗 self，确保只能启动一次）
    pub fn start(mut self) {
        self.running = true;
        thread::spawn(move || {
            println!("游戏服务开始 ===>");

            loop {
                // 处理命令
                if let Ok(cmd) = self.cmd_rx.try_recv() {
                    match cmd {
                        GameCommand::Army(num1, num2) => self.army_message(num1, num2),
                        GameCommand::StartBattle => self.start_battle(),
                        GameCommand::StopBattle => self.battle_running = false,
                        GameCommand::StopService => self.running = false,
                    }
                }
                // 检查是否要退出
                if !self.running {
                    break;
                }

                // 执行战斗逻辑（或其他工作）
                Self::run_battle(&mut self);

                // 控制帧率 / 避免忙等待
                thread::sleep(Duration::from_millis(50));
            }

            println!("<=== 游戏服务结束");
        });
    }

    // 停止服务
    pub fn stop(&mut self) {
        self.running = false;
    }

    // 战斗逻辑（需要可变引用 &mut self）
    fn run_battle(&mut self) {
        loop {
            if self.battle_running {
                if !battle_run(self.army.clone()) {
                    break;
                }
            } else {
                break;
            }
        }
        self.battle_running = false;
    }
}

pub fn battle_run(army: Arc<RwLock<Army>>) -> bool {
    // 获取写锁
    let mut army = match army.write() {
        Ok(g) => g,
        Err(e) => {
            log::error!("battle_run: 获取 army 写锁失败 - {}", e);
            return false;
        }
    };

    let now = Instant::now();
    let mut rng = rand::rng();

    // ========== 胜负判定 ==========
    let enemy_alive = army
        .enemy_units
        .iter()
        .any(|col| col.iter().any(|u| u.hp > 0));
    let friendly_alive = army
        .friendly_units
        .iter()
        .any(|col| col.iter().any(|u| u.hp > 0));

    if !enemy_alive {
        log::info!("🎉 友方胜利！");
        return false;
    }
    if !friendly_alive {
        log::info!("💀 敌方胜利！");
        return false;
    }

    // ========== 收集友方攻击 ==========
    let mut attacks = Vec::new();
    for (col_idx, col) in army.friendly_units.iter().enumerate() {
        for (row_idx, unit) in col.iter().enumerate().take(3) {
            // 每列最多前3个单位能攻击
            if unit.hp > 0 && now >= unit.next_attack_at {
                // 找出敌方还有活人的列
                let target_cols: Vec<usize> = army
                    .enemy_units
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| c.iter().any(|u| u.hp > 0))
                    .map(|(i, _)| i)
                    .collect();

                if !target_cols.is_empty() {
                    let target_col = target_cols[rng.random_range(0..target_cols.len())];
                    attacks.push((true, col_idx, row_idx, target_col)); // true = 友方攻击
                }
            }
        }
    }

    // ========== 收集敌方攻击 ==========
    for (col_idx, col) in army.enemy_units.iter().enumerate() {
        for (row_idx, unit) in col.iter().enumerate().take(3) {
            if unit.hp > 0 && now >= unit.next_attack_at {
                let target_cols: Vec<usize> = army
                    .friendly_units
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| c.iter().any(|u| u.hp > 0))
                    .map(|(i, _)| i)
                    .collect();

                if !target_cols.is_empty() {
                    let target_col = target_cols[rng.random_range(0..target_cols.len())];
                    attacks.push((false, col_idx, row_idx, target_col)); // false = 敌方攻击
                }
            }
        }
    }

    for (is_friendly, a_col, a_row, d_col) in attacks {
        if is_friendly {
            if a_col < army.friendly_units.len()
                && a_row < army.friendly_units[a_col].len()
                && d_col < army.enemy_units.len()
                && !army.enemy_units[d_col].is_empty()
            {
                // 直接通过索引修改，不持有引用
                if army.friendly_units[a_col][a_row].hp > 0 && army.enemy_units[d_col][0].hp > 0 {
                    let atk = army.friendly_units[a_col][a_row].atk;
                    let def = army.enemy_units[d_col][0].def;
                    let damage = atk.saturating_sub(def).max(1);

                    army.enemy_units[d_col][0].hp =
                        army.enemy_units[d_col][0].hp.saturating_sub(damage);
                    army.friendly_units[a_col][a_row].next_attack_at = now
                        + std::time::Duration::from_millis(army.friendly_units[a_col][a_row].speek);
                }
            }
        } else {
            // 类似处理敌方攻击
            if a_col < army.enemy_units.len()
                && a_row < army.enemy_units[a_col].len()
                && d_col < army.friendly_units.len()
                && !army.friendly_units[d_col].is_empty()
                && army.enemy_units[a_col][a_row].hp > 0
                && army.friendly_units[d_col][0].hp > 0
            {
                let atk = army.enemy_units[a_col][a_row].atk;
                let def = army.friendly_units[d_col][0].def;
                let damage = atk.saturating_sub(def).max(1);

                army.friendly_units[d_col][0].hp =
                    army.friendly_units[d_col][0].hp.saturating_sub(damage);
                army.enemy_units[a_col][a_row].next_attack_at =
                    now + std::time::Duration::from_millis(army.enemy_units[a_col][a_row].speek);
            }
        }
    }

    // ========== 清理死亡单位（每列头部连续死亡） ==========
    for col in army.enemy_units.iter_mut() {
        while !col.is_empty() && col.front().is_some_and(|u| u.hp == 0) {
            col.pop_front();
        }
    }
    for col in army.friendly_units.iter_mut() {
        while !col.is_empty() && col.front().is_some_and(|u| u.hp == 0) {
            col.pop_front();
        }
    }

    // ========== 补位：空列从最满列借一个单位 ==========
    {
        let columns = &mut army.enemy_units;
        if columns.len() > 1 {
            let empty_indices: Vec<usize> = columns
                .iter()
                .enumerate()
                .filter(|(_, col)| col.is_empty())
                .map(|(i, _)| i)
                .collect();

            if !empty_indices.is_empty()
                && let Some((richest_idx, _)) = columns
                    .iter()
                    .enumerate()
                    .filter(|(_, col)| !col.is_empty())
                    .max_by_key(|(_, col)| col.len())
                && columns[richest_idx].len() > 1
                && let Some(unit) = columns[richest_idx].pop_back()
                && let Some(&first_empty) = empty_indices.first()
            {
                columns[first_empty].push_back(unit);
            }
        }
    }
    {
        let columns = &mut army.friendly_units;
        if columns.len() > 1 {
            let empty_indices: Vec<usize> = columns
                .iter()
                .enumerate()
                .filter(|(_, col)| col.is_empty())
                .map(|(i, _)| i)
                .collect();

            if !empty_indices.is_empty()
                && let Some((richest_idx, _)) = columns
                    .iter()
                    .enumerate()
                    .filter(|(_, col)| !col.is_empty())
                    .max_by_key(|(_, col)| col.len())
                && columns[richest_idx].len() > 1
                && let Some(unit) = columns[richest_idx].pop_back()
                && let Some(&first_empty) = empty_indices.first()
            {
                columns[first_empty].push_back(unit);
            }
        }
    }

    // 战斗继续
    true
}
