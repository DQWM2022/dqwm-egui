use std::{
    collections::VecDeque,
    sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Faction {
    Ally,
    Enemy,
}

#[derive(Clone, Debug)]

pub struct Unit {
    pub id: usize,
    pub name: &'static str,
    pub hp: u128,
    pub max_hp: u128,
    pub atk: u128,
    pub def: u128,
    pub speek: u64, // 攻击间隔（毫秒）
}

#[derive(Debug, Clone, Default)]
pub struct GlobalArmy {
    pub enemy_units: Vec<VecDeque<Unit>>, // 完整队列
    pub ally_units: Vec<VecDeque<Unit>>,
}

#[derive(Debug, Clone, Default)]
pub struct ViewportArmy {
    pub enemy_units: Vec<VecDeque<Unit>>, // 完整队列
    pub ally_units: Vec<VecDeque<Unit>>,
    pub enemy_num: usize,
    pub ally_num: usize,
}

static NEXT_UNIT_ID: AtomicUsize = AtomicUsize::new(1);
pub fn test(num: usize) -> Vec<VecDeque<Unit>> {
    if num == 0 {
        return vec![];
    }

    let n_cols = match num {
        1..=3 => 1,
        4..=9 => 2,
        10..=29 => 3,
        30..=59 => 4,
        60..=119 => 5,
        _ => 6,
    };
    if n_cols == 0 {
        return vec![];
    }

    // 预生成所有单位（不指定 id，用 new 自动分配）
    let units: Vec<Unit> = (1..=num)
        .map(|i| Unit {
            id: NEXT_UNIT_ID.fetch_add(1, Ordering::Relaxed),
            name: "Unit",
            hp: i as u128,
            max_hp: i as u128,
            atk: 10 + (i % 10) as u128,
            def: 1,
            speek: 100 + ((i % 100) * 2) as u64,
        })
        .collect();

    let mut columns: Vec<VecDeque<Unit>> = vec![VecDeque::new(); n_cols];

    // 按行主序分配：i → (row, col)
    for (i, unit) in units.into_iter().enumerate() {
        let col = i % n_cols; // 关键：列 = i % n_cols
        columns[col].push_back(unit);
    }

    columns
}
