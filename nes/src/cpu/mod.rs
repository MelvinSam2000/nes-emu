use anyhow::Result;

use crate::Nes;

#[derive(Default)]
pub struct Cpu {
    // registers
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub status: u8,

    // helper variables
    pub cycles: u8,
    pub addr: u16,
    pub addr_mode: usize,
    pub data: u8,
    pub is_imp: bool,
}

pub enum CpuFlag {
    C = 1 << 0, // Carry Bit
    Z = 1 << 1, // Zero
    I = 1 << 2, // Disable Interrupts
    D = 1 << 3, // Decimal Mode (not used)
    B = 1 << 4, // Break
    U = 1 << 5, // Unused (break 2)
    V = 1 << 6, // Overflow
    N = 1 << 7, // Negative
}

pub fn reset(nes: &mut Nes) -> Result<()> {
    nes.cpu.ac = 0;
    nes.cpu.x = 0;
    nes.cpu.y = 0;
    nes.cpu.sp = 0xfd;
    nes.cpu.status = CpuFlag::I as u8 | CpuFlag::U as u8;
    nes.cpu.pc = fetch_word(nes, 0xfffc)?;
    nes.cpu.cycles = 8;
    Ok(())
}

pub fn clock(nes: &mut Nes) -> Result<()> {
    if nes.cpu.cycles > 0 {
        nes.cpu.cycles -= 1;
        return Ok(());
    }

    // fetch
    let opcode = read(nes, nes.cpu.pc)?;
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);
    // decode
    let decoded = decode::decode(opcode);
    nes.cpu.cycles = decoded.cycles;
    // execute
    nes.cpu.addr_mode = decoded.addr_mode as usize;
    (decoded.addr_mode)(nes)?;
    (decoded.instruction)(nes)?;
    Ok(())
}

pub fn irq(nes: &mut Nes) -> Result<()> {
    if get_flag(nes, CpuFlag::I) {
        return Ok(());
    }
    write(
        nes,
        (nes.cpu.sp as u16).wrapping_add(0x0100),
        ((nes.cpu.pc >> 8) & 0x00ff) as u8,
    )?;
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    write(
        nes,
        (nes.cpu.sp as u16).wrapping_add(0x0100),
        nes.cpu.pc as u8,
    )?;
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    write(
        nes,
        (nes.cpu.sp as u16).wrapping_add(0x0100),
        nes.cpu.status,
    )?;
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);

    set_flag(nes, CpuFlag::B, false);
    set_flag(nes, CpuFlag::I, true);

    nes.cpu.pc = fetch_word(nes, 0xfffe)?;

    nes.cpu.cycles = 7;
    Ok(())
}

pub fn nmi(nes: &mut Nes) -> Result<()> {
    write(
        nes,
        (nes.cpu.sp as u16).wrapping_add(0x0100),
        ((nes.cpu.pc >> 8) & 0x00ff) as u8,
    )?;
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    write(
        nes,
        (nes.cpu.sp as u16).wrapping_add(0x0100),
        nes.cpu.pc as u8,
    )?;
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    write(
        nes,
        (nes.cpu.sp as u16).wrapping_add(0x0100),
        nes.cpu.status,
    )?;
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);

    set_flag(nes, CpuFlag::B, false);
    set_flag(nes, CpuFlag::I, true);

    nes.cpu.pc = fetch_word(nes, 0xfffa)?;

    nes.cpu.cycles = 8;
    Ok(())
}

// HELPER METHODS

pub fn read(_nes: &mut Nes, _addr: u16) -> Result<u8> {
    //buscpu::read(nes, addr)
    Ok(0)
}

pub fn write(_nes: &mut Nes, _addr: u16, _data: u8) -> Result<()> {
    //buscpu::write(nes, addr, data);
    Ok(())
}

pub fn set_flag(nes: &mut Nes, flag: CpuFlag, val: bool) {
    if val {
        nes.cpu.status |= flag as u8;
    } else {
        nes.cpu.status &= !(flag as u8);
    }
}

pub fn get_flag(nes: &Nes, flag: CpuFlag) -> bool {
    flag as u8 & nes.cpu.status != 0x00
}

pub fn fetch_word(nes: &mut Nes, addr: u16) -> Result<u16> {
    let lo = read(nes, addr)? as u16;
    let hi = read(nes, addr.wrapping_add(1))? as u16;
    Ok(hi << 8 | lo)
}

pub fn pc_fetch_byte(nes: &mut Nes) -> Result<u8> {
    let data = read(nes, nes.cpu.pc)?;
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);
    Ok(data)
}

pub fn pc_fetch_word(nes: &mut Nes) -> Result<u16> {
    let data = fetch_word(nes, nes.cpu.pc)?;
    nes.cpu.pc = nes.cpu.pc.wrapping_add(2);
    Ok(data)
}

pub fn fetch_data(nes: &mut Nes) -> Result<()> {
    if !nes.cpu.is_imp {
        nes.cpu.data = read(nes, nes.cpu.addr)?;
    }
    Ok(())
}

pub mod addressing;
pub mod decode;
pub mod instructions;
