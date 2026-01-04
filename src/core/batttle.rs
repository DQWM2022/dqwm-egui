use std::collections::VecDeque;

use crate::model::Unit;

struct Army {
    enemys: Vec<VecDeque<Unit>>,
    allys: Vec<VecDeque<Unit>>,
}

pub struct ArmyTick {
    pub enemys: Vec<VecDeque<Unit>>,
    pub allys: Vec<VecDeque<Unit>>,
    pub enemys_num: usize,
    pub allys_num: usize,
}

pub enum BattleEvent {
    AtkEvent(u128),       // 攻击者id
    DefEvent(u128, u128), // 被攻击者id，受到的伤害
}
pub struct BattleContext {
    max_enemy_cols: usize,
    max_ally_cols: usize,
    army: Army,
}

struct BattleExecutor;
impl BattleExecutor {
    async fn run(&self, ctx: BattleContext) {}
}
