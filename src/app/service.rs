use rand::seq::IndexedRandom;
use std::{
    collections::VecDeque,
    sync::{Arc, mpsc::Receiver},
    thread,
    time::{Duration, Instant},
};

use crate::{GameCommand, app::Unit, double_buffer::DoubleBuffer};

#[derive(Default, Debug, Clone)]
pub struct Army {
    pub enemy_units: Vec<VecDeque<Unit>>,
    pub friendly_units: Vec<VecDeque<Unit>>,

    pub enemy_num: usize,
    pub friendly_num: usize,
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
    pub army_view: Arc<DoubleBuffer<Army>>,
    pub army: Army,
}

impl GameService {
    pub fn new(cmd_rx: Receiver<GameCommand>, army_view: Arc<DoubleBuffer<Army>>) -> Self {
        Self {
            cmd_rx,
            running: false,
            battle_running: false,
            army_view,
            army: Army::default(),
        }
    }

    pub fn army_message(&mut self, enemy_num: usize, friendly_num: usize) {
        let enemy_units = Unit::test(enemy_num);
        let friendly_units = Unit::test(friendly_num);

        self.army = Army {
            enemy_units,
            friendly_units,
            enemy_num,
            friendly_num,
        };

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
                self.set_army_view();
                if self.battle_running && !self.battle_run() {
                    self.battle_running = false;
                }

                // 控制帧率 / 避免忙等待
                if !self.battle_running {
                    thread::sleep(Duration::from_millis(50));
                }
            }

            println!("<=== 游戏服务结束");
        });
    }

    // 停止服务
    pub fn stop(&mut self) {
        self.running = false;
    }

    fn set_army_view(&self) {
        // 需要先检测是否运行并且是否为空
        if !self.running || self.army.enemy_units.is_empty() || self.army.friendly_units.is_empty()
        {
            return;
        }
        self.army_view.write(|view_army| {
            // 只取每列前 10 个单位（或更少）
            view_army.enemy_units = self
                .army
                .enemy_units
                .iter()
                .map(|col| col.iter().take(10).cloned().collect())
                .collect();

            view_army.friendly_units = self
                .army
                .friendly_units
                .iter()
                .map(|col| col.iter().take(10).cloned().collect())
                .collect();
            view_army.enemy_num = self.army.enemy_units.iter().map(VecDeque::len).sum();
            view_army.friendly_num = self.army.friendly_units.iter().map(VecDeque::len).sum();
        });
        self.army_view.swap(); // 提交
    }

    pub fn battle_run(&mut self) -> bool {
        let now = Instant::now();
        let mut rng = rand::rng();

        // ========== 1. 快速胜负判定 + 收集可攻击单位 ==========
        let mut enemy_alive = false;
        let mut friendly_alive = false;
        let mut friendly_attacks = Vec::with_capacity(9); // 最多 3列×3行
        let mut enemy_attacks = Vec::with_capacity(9);

        // 友方：扫描并收集活单位 + 判断存活
        for (col_idx, col) in self.army.friendly_units.iter().enumerate() {
            let mut col_has_live = false;
            for (row_idx, unit) in col.iter().enumerate().take(3) {
                if unit.hp > 0 {
                    col_has_live = true;
                    if now >= unit.next_attack_at {
                        friendly_attacks.push((col_idx, row_idx));
                    }
                }
            }
            if col_has_live {
                friendly_alive = true;
            }
        }

        // 敌方：同理
        for (col_idx, col) in self.army.enemy_units.iter().enumerate() {
            let mut col_has_live = false;
            for (row_idx, unit) in col.iter().enumerate().take(3) {
                if unit.hp > 0 {
                    col_has_live = true;
                    if now >= unit.next_attack_at {
                        enemy_attacks.push((col_idx, row_idx));
                    }
                }
            }
            if col_has_live {
                enemy_alive = true;
            }
        }

        // 胜负判定（提前退出）
        if !enemy_alive {
            log::info!("🎉 友方胜利！");
            return false;
        }
        if !friendly_alive {
            log::info!("💀 敌方胜利！");
            return false;
        }

        // ========== 2. 预计算有效目标列（只做一次） ==========
        let mut enemy_target_cols: Vec<usize> = Vec::new();
        let mut friendly_target_cols: Vec<usize> = Vec::new();

        for (i, col) in self.army.enemy_units.iter().enumerate() {
            if col.iter().any(|u| u.hp > 0) {
                enemy_target_cols.push(i);
            }
        }
        for (i, col) in self.army.friendly_units.iter().enumerate() {
            if col.iter().any(|u| u.hp > 0) {
                friendly_target_cols.push(i);
            }
        }

        // ========== 3. 执行攻击（直接修改，无额外检查） ==========
        for (a_col, a_row) in friendly_attacks {
            if enemy_target_cols.is_empty() {
                continue;
            }
            let d_col = *enemy_target_cols
                .choose(&mut rng)
                .expect("enemy_target_cols 非空，choose() 不应返回 None");

            // 安全前提：调用方保证 a_col/a_row 有效（来自 iter().enumerate()）
            let attacker = &mut self.army.friendly_units[a_col][a_row];
            if attacker.hp == 0 {
                continue;
            } // 可能被前面攻击杀死

            if let Some(defender) = self.army.enemy_units[d_col].front_mut()
                && defender.hp > 0
            {
                let damage = attacker.atk.saturating_sub(defender.def).max(1);
                defender.hp = defender.hp.saturating_sub(damage);
                attacker.next_attack_at = now + std::time::Duration::from_millis(attacker.speek);
            }
        }

        for (a_col, a_row) in enemy_attacks {
            if friendly_target_cols.is_empty() {
                continue;
            }
            let d_col = *friendly_target_cols
                .choose(&mut rng)
                .expect("friendly_target_cols 非空，choose() 不应返回 None");

            let attacker = &mut self.army.enemy_units[a_col][a_row];
            if attacker.hp == 0 {
                continue;
            }

            if let Some(defender) = self.army.friendly_units[d_col].front_mut()
                && defender.hp > 0
            {
                let damage = attacker.atk.saturating_sub(defender.def).max(1);
                defender.hp = defender.hp.saturating_sub(damage);
                attacker.next_attack_at = now + std::time::Duration::from_millis(attacker.speek);
            }
        }

        // ========== 4. 清理 + 补位（复用函数） ==========
        cleanup_columns(&mut self.army.enemy_units);
        cleanup_columns(&mut self.army.friendly_units);
        rebalance_columns(&mut self.army.enemy_units);
        rebalance_columns(&mut self.army.friendly_units);

        true
    }
}
// 清理每列头部死亡单位
fn cleanup_columns(columns: &mut [VecDeque<Unit>]) {
    for col in columns {
        while col.front().is_some_and(|u| u.hp == 0) {
            col.pop_front();
        }
    }
}

// 补位：空列从最满列借一个单位（仅当有富余时）
fn rebalance_columns(columns: &mut [VecDeque<Unit>]) {
    if columns.len() <= 1 {
        return;
    }

    let empty_indices: Vec<usize> = columns
        .iter()
        .enumerate()
        .filter(|(_, col)| col.is_empty())
        .map(|(i, _)| i)
        .collect();

    if empty_indices.is_empty() {
        return;
    }

    // 找到最长的非空列（且长度 > 1）
    if let Some((richest_idx, richest_len)) = columns
        .iter()
        .enumerate()
        .filter(|(_, col)| !col.is_empty())
        .map(|(i, col)| (i, col.len()))
        .max_by_key(|&(_, len)| len)
        && richest_len > 1
        && let Some(unit) = columns[richest_idx].pop_back()
        && let Some(&first_empty) = empty_indices.first()
    {
        columns[first_empty].push_back(unit);
    }
}
