pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

impl Fern {
    /// 模拟蕨类植物一天的生长
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

/// 运行并模拟指定天数的生长状况
pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}
