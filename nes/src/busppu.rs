use anyhow::anyhow;
use anyhow::Result;

use crate::cartridge;
use crate::cartridge::Mirroring;
use crate::Nes;

pub struct BusPpu {
    pub vram: [u8; 0x1000], // 4 KB of VRAM
    pub palette: [u8; 0x20],
}

impl Default for BusPpu {
    fn default() -> Self {
        Self {
            vram: [0; 0x1000],
            palette: [0; 0x20],
        }
    }
}

pub fn read<S, A>(nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
    match addr {
        0x0000..=0x1fff => cartridge::chr_read(nes, addr),
        0x2000..=0x2fff => {
            let mapped_addr = mirror_vram_addr(nes, addr);
            Ok(nes.bus_ppu.vram[mapped_addr as usize])
        }
        0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
            let mut addr_mirror = (addr - 0x10) & 0x3f;
            if nes.ppu.reg_mask.grayscale() {
                addr_mirror &= 0x30;
            }
            Ok(nes.bus_ppu.palette[addr_mirror as usize])
        }
        0x3f00..=0x3fff => {
            let mut addr_mirror = (addr & 0x3f1f) & 0x3f;
            if nes.ppu.reg_mask.grayscale() {
                addr_mirror &= 0x30;
            }
            Ok(nes.bus_ppu.palette[addr_mirror as usize])
        }
        _ => Err(anyhow!("Invalid read on ppu bus at address {:x}", addr)),
    }
}

pub fn write<S, A>(nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()> {
    match addr {
        0x0000..=0x1fff => {
            cartridge::chr_write(nes, addr, data)?;
        }
        0x2000..=0x2fff => {
            let mapped_addr = mirror_vram_addr(nes, addr);
            nes.bus_ppu.vram[mapped_addr as usize] = data;
        }
        0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
            let add_mirror = addr - 0x10;
            nes.bus_ppu.palette[(add_mirror - 0x3f00) as usize] = data;
        }
        0x3f00..=0x3fff => {
            let addr_mirror = addr & 0x3f1f;
            nes.bus_ppu.palette[(addr_mirror - 0x3f00) as usize] = data;
        }
        _ => {
            Err(anyhow!("Invalid write on ppu bus at address {:x}", addr))?;
        }
    }
    Ok(())
}

pub fn mirror_vram_addr<S, A>(nes: &mut Nes<S, A>, addr: u16) -> u16 {
    let mut mapped_addr = addr & 0x0fff;
    match &nes.cartridge.mirroring {
        Mirroring::HORIZONTAL => {
            if (0x0400..0x0800).contains(&mapped_addr) || mapped_addr >= 0x0c00 {
                mapped_addr -= 0x0400;
            }
        }
        Mirroring::VERTICAL => {
            if (0x0800..0x0c00).contains(&mapped_addr) || mapped_addr >= 0x0c00 {
                mapped_addr -= 0x0800;
            }
        }
    }
    mapped_addr
}
