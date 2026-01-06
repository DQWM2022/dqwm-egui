use std::{collections::VecDeque, time::Instant};

use flume::Sender;

use crate::model::Unit;
#[derive(Debug, Default)]
struct Army {
    pub enemys: Vec<VecDeque<Unit>>,
    pub allys: Vec<VecDeque<Unit>>,
}

#[derive(Debug)]
pub enum BattleOutput {
    ArmySnapshot(ArmySnapshot),
    BattleEvent(BattleEvent),
    // 可扩展：Log(String), Progress(f32), Error(String), Done 等
}
#[derive(Debug, Default)]
pub struct ArmySnapshot {
    pub enemys: Vec<VecDeque<Unit>>,
    pub allys: Vec<VecDeque<Unit>>,
    pub enemys_num: usize,
    pub allys_num: usize,
}

#[derive(Debug)]
pub enum BattleEvent {
    ATK {
        id: u128,
        timestamp: Instant, // 可选：用于去重或排序
    },
    DEF {
        id: u128,
        amount: u128,
        timestamp: Instant, // 可选：用于去重或排序
    },
}
impl BattleEvent {
    pub fn atk(id: u128) -> BattleEvent {
        BattleEvent::ATK {
            id,
            timestamp: Instant::now(), // 可选：用于去重或排序
        }
    }
    pub fn def(id: u128, amount: u128) -> BattleEvent {
        BattleEvent::DEF {
            id,
            amount,
            timestamp: Instant::now(), // 可选：用于去重或排序
        }
    }
}

#[derive(Debug)]
pub struct BattleContext {
    pub max_enemy_cols: usize,
    pub max_ally_cols: usize,
    pub army: Army,
    pub events: Sender<BattleEvent>,   // 事件发送
    pub army_tx: Sender<ArmySnapshot>, // 帧发送
}
pub struct BattleExecutor;
impl BattleExecutor {
    pub async fn run(&self, ctx: BattleContext) {
        let army = ctx.army;
        // 我方第一行从左到右依次攻击，攻击敌方第一行随机单位
    }
}
