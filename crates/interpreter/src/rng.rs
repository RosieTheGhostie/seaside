#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

    fn next(&mut self, n_bits: u32) -> u64 {
        self.update_seed();
        self.seed >> 48_u32.saturating_sub(n_bits)
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next(32) as _
    }

    pub fn next_u32_from_range(&mut self, upper_bound: u64) -> Option<u32> {
        if upper_bound == 0 {
            return None;
        } else if upper_bound.is_power_of_two() {
            return Some((self.next(31).wrapping_mul(upper_bound) >> 31) as _);
        }

        let max_value = upper_bound - 1;
        let mut bits: u64;
        let mut value: u64;
        loop {
            bits = self.next(31);
            value = bits % upper_bound;
            if value <= bits + max_value {
                break Some(value as _);
            }
        }
    }

    pub fn next_f32(&mut self) -> f32 {
        const NORMALIZATION_FACTOR: f32 = ((1_u32 << 24) as f32).recip();
        self.next(24) as f32 * NORMALIZATION_FACTOR
    }

    pub fn next_f64(&mut self) -> f64 {
        const NORMALIZATION_FACTOR: f64 = ((1_u64 << 53) as f64).recip();
        ((self.next(26) << 27) + self.next(27)) as f64 * NORMALIZATION_FACTOR
    }
}
