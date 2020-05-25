use rand::Rng;

#[derive(Copy, Clone)]
pub struct RNG(rand::rngs::ThreadRng);

impl RNG {
    pub fn new() -> Self {
        Self(rand::thread_rng())
    }

    /// Generate a random number in `0..256`
    pub fn rand(&mut self) -> u8 {
        self.0.gen_range(0, 256u16) as u8
    }
}

impl Default for RNG {
    fn default() -> Self {
        Self::new()
    }
}
