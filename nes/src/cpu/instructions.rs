use anyhow::Result;

use crate::cpu;
use crate::cpu::addressing;
use crate::cpu::CpuFlag;
use crate::Nes;

pub fn adc(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;

    let temp: u16 = nes.cpu.ac as u16 + nes.cpu.data as u16 + cpu::get_flag(nes, CpuFlag::C) as u16;
    cpu::set_flag(nes, CpuFlag::C, temp > 255);
    cpu::set_flag(nes, CpuFlag::Z, (temp & 0x00ff) == 0);
    cpu::set_flag(
        nes,
        CpuFlag::V,
        (!(nes.cpu.ac as u16 ^ nes.cpu.data as u16) & (nes.cpu.ac as u16 ^ temp)) & 0x0080 != 0,
    );
    cpu::set_flag(nes, CpuFlag::N, temp & 0x80 != 0);

    nes.cpu.ac = temp as u8;
    nes.cpu.cycles += 1;
    Ok(())
}

pub fn and(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    nes.cpu.ac &= nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0x00);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x80 != 0);
    Ok(())
}

pub fn asl(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    let tmp = (nes.cpu.data as u16) << 1;

    cpu::set_flag(nes, CpuFlag::C, tmp & 0xff00 != 0);
    cpu::set_flag(nes, CpuFlag::Z, tmp & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }
    Ok(())
}

pub fn bcc(nes: &mut Nes) -> Result<()> {
    if !cpu::get_flag(nes, CpuFlag::C) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn bcs(nes: &mut Nes) -> Result<()> {
    if cpu::get_flag(nes, CpuFlag::C) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn beq(nes: &mut Nes) -> Result<()> {
    if cpu::get_flag(nes, CpuFlag::Z) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn bit(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    let tmp = nes.cpu.ac & nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, tmp == 0x00);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.data & (1 << 7) != 0);
    cpu::set_flag(nes, CpuFlag::V, nes.cpu.data & (1 << 6) != 0);
    Ok(())
}

pub fn bmi(nes: &mut Nes) -> Result<()> {
    if cpu::get_flag(nes, CpuFlag::N) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn bne(nes: &mut Nes) -> Result<()> {
    if !cpu::get_flag(nes, CpuFlag::Z) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn bpl(nes: &mut Nes) -> Result<()> {
    if !cpu::get_flag(nes, CpuFlag::N) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn brk(nes: &mut Nes) -> Result<()> {
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);

    cpu::set_flag(nes, CpuFlag::I, true);
    cpu::write(
        nes,
        (nes.cpu.sp as u16).wrapping_add(0x0100),
        (nes.cpu.pc >> 8) as u8,
    )?;
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    cpu::write(
        nes,
        (nes.cpu.sp as u16).wrapping_add(0x0100),
        nes.cpu.pc as u8,
    )?;
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);

    cpu::set_flag(nes, CpuFlag::B, true);
    cpu::write(
        nes,
        (nes.cpu.sp as u16).wrapping_add(0x0100),
        nes.cpu.status,
    )?;
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    cpu::set_flag(nes, CpuFlag::B, false);

    nes.cpu.pc = cpu::read(nes, 0xfffe)? as u16 | ((cpu::read(nes, 0xffff)? as u16) << 8);
    Ok(())
}

pub fn bvc(nes: &mut Nes) -> Result<()> {
    if !cpu::get_flag(nes, CpuFlag::V) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn bvs(nes: &mut Nes) -> Result<()> {
    if cpu::get_flag(nes, CpuFlag::V) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn clc(nes: &mut Nes) -> Result<()> {
    cpu::set_flag(nes, CpuFlag::C, false);
    Ok(())
}

pub fn cld(nes: &mut Nes) -> Result<()> {
    cpu::set_flag(nes, CpuFlag::D, false);
    Ok(())
}

pub fn cli(nes: &mut Nes) -> Result<()> {
    cpu::set_flag(nes, CpuFlag::I, false);
    Ok(())
}

pub fn clv(nes: &mut Nes) -> Result<()> {
    cpu::set_flag(nes, CpuFlag::V, false);
    Ok(())
}

pub fn cmp(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    let tmp = (nes.cpu.ac as u16).wrapping_sub(nes.cpu.data as u16);
    cpu::set_flag(nes, CpuFlag::C, nes.cpu.ac >= nes.cpu.data);
    cpu::set_flag(nes, CpuFlag::Z, (tmp & 0x00ff) == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
    nes.cpu.cycles += 1;
    Ok(())
}

pub fn cpx(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    let tmp = (nes.cpu.x as u16).wrapping_sub(nes.cpu.data as u16);
    cpu::set_flag(nes, CpuFlag::C, nes.cpu.x >= nes.cpu.data);
    cpu::set_flag(nes, CpuFlag::Z, (tmp & 0x00ff) == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
    Ok(())
}

pub fn cpy(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    let tmp = (nes.cpu.y as u16).wrapping_sub(nes.cpu.data as u16);
    cpu::set_flag(nes, CpuFlag::C, nes.cpu.y >= nes.cpu.data);
    cpu::set_flag(nes, CpuFlag::Z, (tmp & 0x00ff) == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
    Ok(())
}

pub fn dec(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    let tmp = nes.cpu.data.wrapping_sub(1);
    cpu::write(nes, nes.cpu.addr, tmp)?;
    cpu::set_flag(nes, CpuFlag::Z, tmp == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
    Ok(())
}

pub fn dex(nes: &mut Nes) -> Result<()> {
    nes.cpu.x = nes.cpu.x.wrapping_sub(1);
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
    Ok(())
}

pub fn dey(nes: &mut Nes) -> Result<()> {
    nes.cpu.y = nes.cpu.y.wrapping_sub(1);
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.y == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.y & 0x0080 != 0);
    Ok(())
}

pub fn eor(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    nes.cpu.ac ^= nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn inc(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    let tmp = nes.cpu.data.wrapping_add(1);
    cpu::write(nes, nes.cpu.addr, tmp)?;
    cpu::set_flag(nes, CpuFlag::Z, tmp == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
    Ok(())
}

pub fn inx(nes: &mut Nes) -> Result<()> {
    nes.cpu.x = nes.cpu.x.wrapping_add(1);
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
    Ok(())
}

pub fn iny(nes: &mut Nes) -> Result<()> {
    nes.cpu.y = nes.cpu.y.wrapping_add(1);
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.y == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.y & 0x0080 != 0);
    Ok(())
}

pub fn jmp(nes: &mut Nes) -> Result<()> {
    nes.cpu.pc = nes.cpu.addr;
    Ok(())
}

pub fn jsr(nes: &mut Nes) -> Result<()> {
    nes.cpu.pc = nes.cpu.pc.wrapping_sub(1) as u16;
    cpu::write(
        nes,
        (nes.cpu.sp as u16).wrapping_add(0x0100),
        ((nes.cpu.pc >> 8) & 0x00ff) as u8,
    )?;
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    cpu::write(
        nes,
        (nes.cpu.sp as u16).wrapping_add(0x0100),
        nes.cpu.pc as u8,
    )?;
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    nes.cpu.pc = nes.cpu.addr;
    Ok(())
}

pub fn lda(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    nes.cpu.ac = nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn ldx(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    nes.cpu.x = nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
    Ok(())
}

pub fn ldy(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    nes.cpu.y = nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.y == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.y & 0x0080 != 0);
    Ok(())
}

pub fn lsr(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;

    cpu::set_flag(nes, CpuFlag::C, nes.cpu.data & 0x0001 != 0);

    let tmp = (nes.cpu.data as u16) >> 1;

    cpu::set_flag(nes, CpuFlag::Z, tmp & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }
    Ok(())
}

pub fn nop(_nes: &mut Nes) -> Result<()> {
    Ok(())
}

pub fn ora(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    nes.cpu.ac |= nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0x00);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x80 != 0);
    Ok(())
}

pub fn pha(nes: &mut Nes) -> Result<()> {
    cpu::write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), nes.cpu.ac)?;
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    Ok(())
}

pub fn php(nes: &mut Nes) -> Result<()> {
    cpu::write(
        nes,
        (nes.cpu.sp as u16).wrapping_add(0x0100),
        nes.cpu.status | CpuFlag::B as u8,
    )?;
    cpu::set_flag(nes, CpuFlag::B, false);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    Ok(())
}

pub fn pla(nes: &mut Nes) -> Result<()> {
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.ac = cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100))?;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0x00);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x80 != 0);
    Ok(())
}

pub fn plp(nes: &mut Nes) -> Result<()> {
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.status = cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100))?;
    cpu::set_flag(nes, CpuFlag::B, false);
    cpu::set_flag(nes, CpuFlag::U, true);
    Ok(())
}

pub fn rol(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    let mut tmp = (nes.cpu.data as u16) << 1;
    if cpu::get_flag(nes, CpuFlag::C) {
        tmp |= 0x0001;
    }

    cpu::set_flag(nes, CpuFlag::C, tmp & 0xff00 != 0);
    cpu::set_flag(nes, CpuFlag::Z, tmp & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }
    Ok(())
}

pub fn ror(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    let mut tmp = (nes.cpu.data as u16) >> 1;
    if cpu::get_flag(nes, CpuFlag::C) {
        tmp |= 0x0080;
    }

    cpu::set_flag(nes, CpuFlag::C, nes.cpu.data & 0x0001 != 0);
    cpu::set_flag(nes, CpuFlag::Z, tmp & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }
    Ok(())
}

pub fn rti(nes: &mut Nes) -> Result<()> {
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.status = cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100))?;
    cpu::set_flag(nes, CpuFlag::U, true);
    cpu::set_flag(nes, CpuFlag::B, false);
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.pc = cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100))? as u16;
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.pc |= (cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100))? as u16) << 8;
    Ok(())
}

pub fn rts(nes: &mut Nes) -> Result<()> {
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.pc = cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100))? as u16;
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.pc |= (cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100))? as u16) << 8;
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);
    Ok(())
}

pub fn sbc(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    let temp1: u16 = nes.cpu.data as u16 ^ 0x00ff;
    let mut temp2: u16 = (nes.cpu.ac as u16).wrapping_add(temp1);
    if cpu::get_flag(nes, CpuFlag::C) {
        temp2 = temp2.wrapping_add(1);
    }

    cpu::set_flag(nes, CpuFlag::C, temp2 & 0xff00 != 0);
    cpu::set_flag(nes, CpuFlag::Z, temp2 & 0x00ff == 0);
    cpu::set_flag(
        nes,
        CpuFlag::V,
        (temp2 ^ nes.cpu.ac as u16) & (temp1 ^ temp2) & 0x0080 != 0,
    );
    cpu::set_flag(nes, CpuFlag::N, temp2 & 0x0080 != 0);

    nes.cpu.ac = temp2 as u8;
    nes.cpu.cycles += 1;
    Ok(())
}

pub fn sec(nes: &mut Nes) -> Result<()> {
    cpu::set_flag(nes, CpuFlag::C, true);
    Ok(())
}

pub fn sed(nes: &mut Nes) -> Result<()> {
    cpu::set_flag(nes, CpuFlag::D, true);
    Ok(())
}

pub fn sei(nes: &mut Nes) -> Result<()> {
    cpu::set_flag(nes, CpuFlag::I, true);
    Ok(())
}

pub fn sta(nes: &mut Nes) -> Result<()> {
    cpu::write(nes, nes.cpu.addr, nes.cpu.ac)?;
    Ok(())
}

pub fn stx(nes: &mut Nes) -> Result<()> {
    cpu::write(nes, nes.cpu.addr, nes.cpu.x)?;
    Ok(())
}

pub fn sty(nes: &mut Nes) -> Result<()> {
    cpu::write(nes, nes.cpu.addr, nes.cpu.y)?;
    Ok(())
}

pub fn tax(nes: &mut Nes) -> Result<()> {
    nes.cpu.x = nes.cpu.ac;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
    Ok(())
}

pub fn tay(nes: &mut Nes) -> Result<()> {
    nes.cpu.y = nes.cpu.ac;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.y == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.y & 0x0080 != 0);
    Ok(())
}

pub fn tsx(nes: &mut Nes) -> Result<()> {
    nes.cpu.x = nes.cpu.sp;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
    Ok(())
}

pub fn txa(nes: &mut Nes) -> Result<()> {
    nes.cpu.ac = nes.cpu.x;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn txs(nes: &mut Nes) -> Result<()> {
    nes.cpu.sp = nes.cpu.x;
    Ok(())
}

pub fn tya(nes: &mut Nes) -> Result<()> {
    nes.cpu.ac = nes.cpu.y;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn xxx(_nes: &mut Nes) -> Result<()> {
    Ok(())
}

// UNOFFICIAL OPCODES:

pub fn dcp(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    nes.cpu.data = nes.cpu.data.wrapping_sub(1);
    cpu::write(nes, nes.cpu.addr, nes.cpu.data)?;
    if nes.cpu.ac >= nes.cpu.data {
        cpu::set_flag(nes, CpuFlag::C, true);
    }
    let tmp = nes.cpu.ac.wrapping_sub(nes.cpu.data);
    cpu::set_flag(nes, CpuFlag::Z, tmp == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
    Ok(())
}

pub fn dop(nes: &mut Nes) -> Result<()> {
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);
    Ok(())
}

pub fn isb(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    let tmp = nes.cpu.data.wrapping_add(1);
    cpu::write(nes, nes.cpu.addr, tmp)?;
    nes.cpu.ac = nes.cpu.ac.wrapping_sub(tmp);
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn lax(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    nes.cpu.ac = nes.cpu.data;
    nes.cpu.x = nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.data == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.data & 0x0080 != 0);
    Ok(())
}

pub fn rla(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    let mut tmp = (nes.cpu.data as u16) << 1;
    if cpu::get_flag(nes, CpuFlag::C) {
        tmp |= 0x0001;
    }

    cpu::set_flag(nes, CpuFlag::C, tmp & 0xff00 != 0);

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }

    nes.cpu.ac &= tmp as u8;

    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn rra(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    let mut tmp = (nes.cpu.data as u16) >> 1;
    if cpu::get_flag(nes, CpuFlag::C) {
        tmp |= 0x0080;
    }

    cpu::set_flag(nes, CpuFlag::C, nes.cpu.data & 0x0001 != 0);

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }

    // ADD
    nes.cpu.ac |= tmp as u8;

    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn sax(nes: &mut Nes) -> Result<()> {
    let val = nes.cpu.ac & nes.cpu.x;
    cpu::write(nes, nes.cpu.addr, val)?;
    Ok(())
}

pub fn slo(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    let tmp = (nes.cpu.data as u16) << 1;

    cpu::set_flag(nes, CpuFlag::C, tmp & 0xff00 != 0);

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }

    nes.cpu.ac |= tmp as u8;

    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn sre(nes: &mut Nes) -> Result<()> {
    cpu::fetch_data(nes)?;
    cpu::set_flag(nes, CpuFlag::C, nes.cpu.data & 0x0001 != 0);

    let tmp = (nes.cpu.data as u16) >> 1;

    if nes.cpu.addr_mode == addressing::imp as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }

    nes.cpu.ac ^= tmp as u8;

    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);

    Ok(())
}

pub fn top(nes: &mut Nes) -> Result<()> {
    nes.cpu.pc = nes.cpu.pc.wrapping_add(2);
    Ok(())
}
