use anyhow::Result;

use crate::cpu;
use crate::cpu::addressing;
use crate::cpu::CpuFlag;
use crate::nesaudio::NesAudio;
use crate::nesscreen::NesScreen;
use crate::Nes;

pub fn adc<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
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

pub fn and<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    nes.cpu.ac &= nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0x00);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x80 != 0);
    Ok(())
}

pub fn asl<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    let tmp = (nes.cpu.data as u16) << 1;

    cpu::set_flag(nes, CpuFlag::C, tmp & 0xff00 != 0);
    cpu::set_flag(nes, CpuFlag::Z, tmp & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);

    if nes.cpu.addr_mode == addressing::imp::<S, A> as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }
    Ok(())
}

pub fn bcc<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    if !cpu::get_flag(nes, CpuFlag::C) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn bcs<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    if cpu::get_flag(nes, CpuFlag::C) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn beq<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    if cpu::get_flag(nes, CpuFlag::Z) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn bit<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    let tmp = nes.cpu.ac & nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, tmp == 0x00);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.data & (1 << 7) != 0);
    cpu::set_flag(nes, CpuFlag::V, nes.cpu.data & (1 << 6) != 0);
    Ok(())
}

pub fn bmi<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    if cpu::get_flag(nes, CpuFlag::N) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn bne<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    if !cpu::get_flag(nes, CpuFlag::Z) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn bpl<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    if !cpu::get_flag(nes, CpuFlag::N) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn brk<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
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

pub fn bvc<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    if !cpu::get_flag(nes, CpuFlag::V) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn bvs<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    if cpu::get_flag(nes, CpuFlag::V) {
        nes.cpu.cycles += 1;
        let addr = nes.cpu.pc.wrapping_add(nes.cpu.addr);
        nes.cpu.pc = addr;
    }
    Ok(())
}

pub fn clc<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::set_flag(nes, CpuFlag::C, false);
    Ok(())
}

pub fn cld<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::set_flag(nes, CpuFlag::D, false);
    Ok(())
}

pub fn cli<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::set_flag(nes, CpuFlag::I, false);
    Ok(())
}

pub fn clv<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::set_flag(nes, CpuFlag::V, false);
    Ok(())
}

pub fn cmp<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    let tmp = (nes.cpu.ac as u16).wrapping_sub(nes.cpu.data as u16);
    cpu::set_flag(nes, CpuFlag::C, nes.cpu.ac >= nes.cpu.data);
    cpu::set_flag(nes, CpuFlag::Z, (tmp & 0x00ff) == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
    nes.cpu.cycles += 1;
    Ok(())
}

pub fn cpx<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    let tmp = (nes.cpu.x as u16).wrapping_sub(nes.cpu.data as u16);
    cpu::set_flag(nes, CpuFlag::C, nes.cpu.x >= nes.cpu.data);
    cpu::set_flag(nes, CpuFlag::Z, (tmp & 0x00ff) == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
    Ok(())
}

pub fn cpy<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    let tmp = (nes.cpu.y as u16).wrapping_sub(nes.cpu.data as u16);
    cpu::set_flag(nes, CpuFlag::C, nes.cpu.y >= nes.cpu.data);
    cpu::set_flag(nes, CpuFlag::Z, (tmp & 0x00ff) == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
    Ok(())
}

pub fn dec<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    let tmp = nes.cpu.data.wrapping_sub(1);
    cpu::write(nes, nes.cpu.addr, tmp)?;
    cpu::set_flag(nes, CpuFlag::Z, tmp == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
    Ok(())
}

pub fn dex<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.x = nes.cpu.x.wrapping_sub(1);
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
    Ok(())
}

pub fn dey<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.y = nes.cpu.y.wrapping_sub(1);
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.y == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.y & 0x0080 != 0);
    Ok(())
}

pub fn eor<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    nes.cpu.ac ^= nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn inc<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    let tmp = nes.cpu.data.wrapping_add(1);
    cpu::write(nes, nes.cpu.addr, tmp)?;
    cpu::set_flag(nes, CpuFlag::Z, tmp == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);
    Ok(())
}

pub fn inx<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.x = nes.cpu.x.wrapping_add(1);
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
    Ok(())
}

pub fn iny<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.y = nes.cpu.y.wrapping_add(1);
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.y == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.y & 0x0080 != 0);
    Ok(())
}

pub fn jmp<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.pc = nes.cpu.addr;
    Ok(())
}

pub fn jsr<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
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

pub fn lda<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    nes.cpu.ac = nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn ldx<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    nes.cpu.x = nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
    Ok(())
}

pub fn ldy<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    nes.cpu.y = nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.y == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.y & 0x0080 != 0);
    Ok(())
}

pub fn lsr<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;

    cpu::set_flag(nes, CpuFlag::C, nes.cpu.data & 0x0001 != 0);

    let tmp = (nes.cpu.data as u16) >> 1;

    cpu::set_flag(nes, CpuFlag::Z, tmp & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);

    if nes.cpu.addr_mode == addressing::imp::<S, A> as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }
    Ok(())
}

pub fn nop<S, A>(_nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    Ok(())
}

pub fn ora<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    nes.cpu.ac |= nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0x00);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x80 != 0);
    Ok(())
}

pub fn pha<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::write(nes, (nes.cpu.sp as u16).wrapping_add(0x0100), nes.cpu.ac)?;
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    Ok(())
}

pub fn php<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::write(
        nes,
        (nes.cpu.sp as u16).wrapping_add(0x0100),
        nes.cpu.status | CpuFlag::B as u8,
    )?;
    cpu::set_flag(nes, CpuFlag::B, false);
    nes.cpu.sp = nes.cpu.sp.wrapping_sub(1);
    Ok(())
}

pub fn pla<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.ac = cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100))?;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0x00);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x80 != 0);
    Ok(())
}

pub fn plp<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.status = cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100))?;
    cpu::set_flag(nes, CpuFlag::B, false);
    cpu::set_flag(nes, CpuFlag::U, true);
    Ok(())
}

pub fn rol<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    let mut tmp = (nes.cpu.data as u16) << 1;
    if cpu::get_flag(nes, CpuFlag::C) {
        tmp |= 0x0001;
    }

    cpu::set_flag(nes, CpuFlag::C, tmp & 0xff00 != 0);
    cpu::set_flag(nes, CpuFlag::Z, tmp & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);

    if nes.cpu.addr_mode == addressing::imp::<S, A> as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }
    Ok(())
}

pub fn ror<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    let mut tmp = (nes.cpu.data as u16) >> 1;
    if cpu::get_flag(nes, CpuFlag::C) {
        tmp |= 0x0080;
    }

    cpu::set_flag(nes, CpuFlag::C, nes.cpu.data & 0x0001 != 0);
    cpu::set_flag(nes, CpuFlag::Z, tmp & 0x00ff == 0);
    cpu::set_flag(nes, CpuFlag::N, tmp & 0x0080 != 0);

    if nes.cpu.addr_mode == addressing::imp::<S, A> as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }
    Ok(())
}

pub fn rti<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
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

pub fn rts<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.pc = cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100))? as u16;
    nes.cpu.sp = nes.cpu.sp.wrapping_add(1);
    nes.cpu.pc |= (cpu::read(nes, (nes.cpu.sp as u16).wrapping_add(0x0100))? as u16) << 8;
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);
    Ok(())
}

pub fn sbc<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
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

pub fn sec<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::set_flag(nes, CpuFlag::C, true);
    Ok(())
}

pub fn sed<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::set_flag(nes, CpuFlag::D, true);
    Ok(())
}

pub fn sei<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::set_flag(nes, CpuFlag::I, true);
    Ok(())
}

pub fn sta<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::write(nes, nes.cpu.addr, nes.cpu.ac)?;
    Ok(())
}

pub fn stx<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::write(nes, nes.cpu.addr, nes.cpu.x)?;
    Ok(())
}

pub fn sty<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::write(nes, nes.cpu.addr, nes.cpu.y)?;
    Ok(())
}

pub fn tax<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.x = nes.cpu.ac;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
    Ok(())
}

pub fn tay<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.y = nes.cpu.ac;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.y == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.y & 0x0080 != 0);
    Ok(())
}

pub fn tsx<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.x = nes.cpu.sp;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.x == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.x & 0x0080 != 0);
    Ok(())
}

pub fn txa<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.ac = nes.cpu.x;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn txs<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.sp = nes.cpu.x;
    Ok(())
}

pub fn tya<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.ac = nes.cpu.y;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn xxx<S, A>(_nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    Ok(())
}

// UNOFFICIAL OPCODES:

pub fn dcp<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
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

pub fn dop<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);
    Ok(())
}

pub fn isb<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    let mut tmp = nes.cpu.data.wrapping_add(1);
    cpu::write(nes, nes.cpu.addr, tmp)?;
    tmp ^= 0xff;
    let res = nes.cpu.ac as u16 + tmp as u16 + cpu::get_flag(nes, CpuFlag::C) as u16;
    cpu::set_flag(nes, CpuFlag::C, res > 0xff);
    cpu::set_flag(
        nes,
        CpuFlag::V,
        nes.cpu.ac & 0x80 == tmp & 0x80 && tmp & 0x80 != res as u8 & 0x80,
    );
    nes.cpu.ac = res as u8;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn lax<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    nes.cpu.ac = nes.cpu.data;
    nes.cpu.x = nes.cpu.data;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.data == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.data & 0x0080 != 0);
    Ok(())
}

pub fn rla<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    let mut tmp = (nes.cpu.data as u16) << 1;
    if cpu::get_flag(nes, CpuFlag::C) {
        tmp |= 0x0001;
    }

    cpu::set_flag(nes, CpuFlag::C, tmp & 0xff00 != 0);

    if nes.cpu.addr_mode == addressing::imp::<S, A> as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }

    nes.cpu.ac &= tmp as u8;

    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn rra<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    let carry = nes.cpu.data & 0x01;
    let result = (nes.cpu.data >> 1) | (cpu::get_flag(nes, CpuFlag::C) as u8) << 7;
    cpu::write(nes, nes.cpu.addr, result)?;
    let add_res = nes.cpu.ac as u16 + result as u16 + carry as u16;
    cpu::set_flag(nes, CpuFlag::C, add_res > 0xff);
    cpu::set_flag(
        nes,
        CpuFlag::V,
        nes.cpu.ac & 0x80 == result & 0x80 && result & 0x80 != add_res as u8 & 0x80,
    );
    nes.cpu.ac = add_res as u8;
    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn sax<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    let val = nes.cpu.ac & nes.cpu.x;
    cpu::write(nes, nes.cpu.addr, val)?;
    Ok(())
}

pub fn slo<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    let tmp = (nes.cpu.data as u16) << 1;

    cpu::set_flag(nes, CpuFlag::C, tmp & 0xff00 != 0);

    if nes.cpu.addr_mode == addressing::imp::<S, A> as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }

    nes.cpu.ac |= tmp as u8;

    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);
    Ok(())
}

pub fn sre<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    cpu::fetch_data(nes)?;
    cpu::set_flag(nes, CpuFlag::C, nes.cpu.data & 0x0001 != 0);

    let tmp = (nes.cpu.data as u16) >> 1;

    if nes.cpu.addr_mode == addressing::imp::<S, A> as usize {
        nes.cpu.ac = tmp as u8;
    } else {
        cpu::write(nes, nes.cpu.addr, tmp as u8)?;
    }

    nes.cpu.ac ^= tmp as u8;

    cpu::set_flag(nes, CpuFlag::Z, nes.cpu.ac == 0);
    cpu::set_flag(nes, CpuFlag::N, nes.cpu.ac & 0x0080 != 0);

    Ok(())
}

pub fn top<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.pc = nes.cpu.pc.wrapping_add(2);
    Ok(())
}
