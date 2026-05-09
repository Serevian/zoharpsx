use bus::Bus;

pub struct Cpu {}

// Instead of a trait, use an enum so it can be easily swapped at runtime
pub trait Backend {
    type Error: std::error::Error;
    fn step(&mut self, state: &mut Cpu, bus: &mut Bus) -> Result<u32, Self::Error>;
}
