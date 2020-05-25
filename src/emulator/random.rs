use rand::Rng;

pub struct RNG(rand::rngs::ThreadRng);

impl RNG {
    pub fn new() -> Self {
        Self(rand::thread_rng())
    }

    pub fn rand(&mut self) -> u8 {
        self.0.gen_range(0, 256u16) as u8
    }
}

impl Default for RNG {
    fn default() -> Self {
        Self::new()
    }
}
