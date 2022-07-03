use anyhow::anyhow;
use anyhow::Result;

use crate::cartridge;
use crate::ppu;
use crate::Nes;

pub struct BusCpu {
    pub ram: [u8; 0x0800], // 2 KB of RAM
}

impl Default for BusCpu {
    fn default() -> Self {
        Self { ram: [0; 0x0800] }
    }
}

pub fn read(nes: &mut Nes, addr: u16) -> Result<u8> {
    match addr {
        0x0000..=0x1fff => Ok(nes.bus_cpu.ram[addr as usize & 0x07ff]),
        0x2000..=0x3fff => ppu::read_ppu_reg(nes, addr & 0x2007),
        0x4016 => Ok(nes.joypad.read()),
        /*
        0x4000 ..= 0x4013 | 0x4015 => {
            //return apu::read(nes, addr);
        },
        */
        0x4020..=0xffff => cartridge::prg_read(nes, addr),
        _ => Err(anyhow!("Invalid read on cpu bus at address {:x}", addr)),
    }
}

pub fn write(nes: &mut Nes, addr: u16, data: u8) -> Result<()> {
    match addr {
        0x0000..=0x1fff => {
            nes.bus_cpu.ram[addr as usize & 0x07ff] = data;
        }
        0x2000..=0x3fff => {
            ppu::write_ppu_reg(nes, addr & 0x2007, data)?;
        }
        0x4014 => {
            // OAM DMA
            ppu::write_ppu_reg(nes, 0x4014, data)?;
        }
        0x4016 => {
            nes.joypad.write(data);
        }
        /*
        0x4000 ..= 0x4013 | 0x4015 => {
            //apu::write(nes, addr, data);
        },
        */
        0x4020..=0xffff => {
            cartridge::prg_write(nes, addr, data)?;
        }

        _ => {
            Err(anyhow!("Invalid write on cpu bus at address {:x}", addr))?;
        }
    };
    Ok(())
}
