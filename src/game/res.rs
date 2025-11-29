// 资源

use std::time::Instant;

pub enum ResType {
    Food,  // 食物
    Wood,  // 木材
    Stone, // 石头
}

impl ResType {
    pub fn to(&self) -> String {
        match self {
            ResType::Food => "食物".to_string(),
            ResType::Wood => "木材".to_string(),
            ResType::Stone => "石头".to_string(),
        }
    }
}

pub struct Res {
    pub name: String,         // 资源名称
    pub num: i128,            // 数量
    pub max: i128,            // 最大数量
    pub last_update: Instant, // 上次更新时间
    pub change_interval: u64, // ← 每多少秒变化一次（单位：毫秒）
    pub change_value: i128,   // ← 每次增长多少
}

impl Res {
    pub fn new(
        name: String,
        num: i128,
        max: i128,
        change_interval: u64,
        change_value: i128,
    ) -> Self {
        Self {
            name,
            num,
            max,
            last_update: Instant::now(),
            change_interval,
            change_value,
        }
    }
}
