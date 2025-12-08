use std::{collections::VecDeque, time::Instant};

pub mod service;

#[derive(Clone, Debug)]
pub struct Unit {
    pub name: String,
    pub hp: u128,
    pub max_hp: u128,
    pub atk: u128,
    pub def: u128,
    pub speek: u64,              // 攻击间隔（毫秒）
    pub next_attack_at: Instant, // 下次可攻击时间
}
impl Unit {
    /// 创建新单位
    pub fn new(name: String, hp: u128, atk: u128, def: u128, speek: u64) -> Self {
        Self {
            name,
            hp,
            max_hp: hp,
            atk,
            def,
            speek,
            next_attack_at: Instant::now(),
        }
    }

    pub fn test(num: usize) -> Vec<VecDeque<Unit>> {
        let now = Instant::now();
        let units: Vec<Unit> = (1..=num)
            .map(|i| Unit {
                name: format!("Unit {}", i),
                hp: 100,
                max_hp: 100,
                atk: 10 + (i % 10) as u128,
                def: 1,
                speek: 100 + ((i % 100) * 2) as u64,
                next_attack_at: now, // 初始即可攻击
            })
            .collect();

        let n_cols = determine_columns(units.len());
        if n_cols == 0 {
            return vec![];
        }

        let rows = units.len().div_ceil(n_cols);
        let mut columns: Vec<VecDeque<Unit>> = vec![VecDeque::new(); n_cols]; // 👈 用 VecDeque 初始化

        for (index, unit) in units.into_iter().enumerate() {
            let col_idx = index / rows;
            if col_idx < n_cols {
                columns[col_idx].push_back(unit); // 👈 用 push_back（或 push_front）
            } else {
                columns[n_cols - 1].push_back(unit);
            }
        }

        columns
    }
}

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
