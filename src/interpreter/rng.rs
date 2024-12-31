#[derive(Default)]
pub struct Rng {
    seed: u64,
}

impl Rng {
    const MAGIC: u64 = 0x5deece66d;
    const MAX_SEED: u64 = (1 << 48) - 1;

    pub fn new(seed: u64) -> Self {
        let mut random = Self::default();
        random.set_seed(seed);
        random
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.seed = (seed ^ Self::MAGIC) & Self::MAX_SEED;
    }

    fn update_seed(&mut self) {
        self.seed = self.seed.wrapping_mul(Self::MAGIC).wrapping_add(0xb) & Self::MAX_SEED;
    }

    fn next(&mut self, bits: u32) -> u64 {
        self.update_seed();
        self.seed >> 48u32.saturating_sub(bits)
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next(32) as u32
    }

    pub fn next_u32_from_range(&mut self, upper_bound: u64) -> Option<u32> {
        if upper_bound == 0 {
            return None;
        } else if upper_bound.is_power_of_two() {
            return Some((self.next(31).wrapping_mul(upper_bound) >> 31) as u32);
        }
        let max_value: u64 = upper_bound - 1;
        let mut bits: u64;
        let mut value: u64;
        loop {
            bits = self.next(31);
            value = bits % upper_bound;
            if value <= bits + max_value {
                return Some(value as u32);
            }
        }
    }

    pub fn next_f32(&mut self) -> f32 {
        self.next(24) as f32 / (1u32 << 24) as f32
    }

    pub fn next_f64(&mut self) -> f64 {
        ((self.next(26) << 27) + self.next(27)) as f64 / (1u64 << 53) as f64
    }
}
