// 资源

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
#[derive(Default)]
pub struct Res {
    pub name: String, // 资源名称
    pub num: i128,    // 数量
    pub max: i128,    // 最大数量
}
