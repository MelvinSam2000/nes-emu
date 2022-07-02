use anyhow::Result;

use crate::cpu::Cpu;

pub struct Nes {
    cpu: Cpu,
}

impl Nes {
    pub fn clock(&mut self) -> Result<()> {
        cpu::clock(self)?;
        Ok(())
    }
}

pub mod cpu;

#[cfg(test)]
mod tests {
    mod cpu;
}
