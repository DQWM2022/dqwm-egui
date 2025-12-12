static NEXT_UNIT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone, Debug)]
pub struct Unit {
    pub id: usize,
    pub name: &'static str,
    pub hp: u128,
    pub max_hp: u128,
    pub atk: u128,
    pub def: u128,
    pub speek: u64,              // 攻击间隔（毫秒）
    pub next_attack_at: Instant, // 下次可攻击时间
}
impl Unit {
    /// 创建新单位
    pub fn new(name: &'static str, hp: u128, atk: u128, def: u128, speek: u64) -> Self {
        Self {
            id: NEXT_UNIT_ID.fetch_add(1, Ordering::Relaxed),
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
        if num == 0 {
            return vec![];
        }

        let now = Instant::now();
        let n_cols = determine_columns(num);
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
                next_attack_at: now,
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
}
