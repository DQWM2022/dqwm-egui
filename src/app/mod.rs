#[derive(Clone)]
pub struct Unit {
    pub name: String,
    pub hp: u64,
    pub max_hp: u64,
    pub atk: u64,
    pub def: u64,
}

impl Unit {
    pub fn test() -> Vec<Vec<Unit>> {
        // 1. 生成 200 个 Unit
        let units: Vec<Unit> = (1..=200)
            .map(|i| Unit {
                name: format!("Unit {}", i),
                hp: 100,
                max_hp: 100,
                atk: 20 + i as u64,
                def: if i % 2 == 0 { 10 + i as u64 } else { 0 },
            })
            .collect();

        // 2. 根据数量决定列数
        let n_cols = determine_columns(units.len());
        if n_cols == 0 {
            return vec![];
        }

        // 3. 计算每列应有多少行（向上取整）
        let rows = units.len().div_ceil(n_cols);

        // 4. 按“填满第一列再第二列”的方式分组
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

    // 绘制自身的方法 epaint
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
