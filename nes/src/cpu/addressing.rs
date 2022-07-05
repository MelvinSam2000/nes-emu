use anyhow::Result;

use crate::cpu;
use crate::nesaudio::NesAudio;
use crate::nesscreen::NesScreen;
use crate::Nes;

pub fn abs<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.addr = cpu::pc_fetch_word(nes)?;
    nes.cpu.is_imp = false;
    Ok(())
}

pub fn abx<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    abs(nes)?;
    nes.cpu.addr = nes.cpu.addr.wrapping_add(nes.cpu.x as u16);
    nes.cpu.is_imp = false;
    Ok(())
}

pub fn aby<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    abs(nes)?;
    nes.cpu.addr = nes.cpu.addr.wrapping_add(nes.cpu.y as u16);
    nes.cpu.is_imp = false;
    Ok(())
}

pub fn imm<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.addr = nes.cpu.pc;
    nes.cpu.pc = nes.cpu.pc.wrapping_add(1);
    nes.cpu.is_imp = false;
    Ok(())
}

pub fn imp<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.data = nes.cpu.ac;
    nes.cpu.is_imp = true;
    Ok(())
}

pub fn ind<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    let ptr = cpu::pc_fetch_word(nes)?;
    // emulate page boundary bug or behave normally
    if ptr & 0x00ff == 0x00ff {
        nes.cpu.addr = cpu::read(nes, ptr)? as u16;
        nes.cpu.addr |= (cpu::read(nes, ptr & 0xff00)? as u16) << 8;
    } else {
        nes.cpu.addr = cpu::fetch_word(nes, ptr)?;
    }
    nes.cpu.is_imp = false;
    Ok(())
}

pub fn idx<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    let ptr = cpu::pc_fetch_byte(nes)? as u16;
    let lo = cpu::read(nes, ptr.wrapping_add(nes.cpu.x as u16) & 0x00ff)? as u16;
    let hi = cpu::read(
        nes,
        ptr.wrapping_add(nes.cpu.x as u16).wrapping_add(1) & 0x00ff,
    )? as u16;
    nes.cpu.addr = (hi << 8) | lo;
    nes.cpu.is_imp = false;
    Ok(())
}

pub fn idy<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    let ptr = cpu::pc_fetch_byte(nes)? as u16;
    let lo = cpu::read(nes, ptr & 0x00ff)? as u16;
    let hi = cpu::read(nes, ptr.wrapping_add(1) & 0x00ff)? as u16;
    nes.cpu.addr = (hi << 8) | lo;
    nes.cpu.addr = nes.cpu.addr.wrapping_add(nes.cpu.y as u16);
    nes.cpu.is_imp = false;
    Ok(())
}

pub fn rel<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.addr = cpu::pc_fetch_byte(nes)? as u16;
    if nes.cpu.addr & 0x0080 != 0 {
        nes.cpu.addr |= 0xff00;
    }
    nes.cpu.is_imp = false;
    Ok(())
}

pub fn zpg<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.addr = cpu::pc_fetch_byte(nes)? as u16;
    nes.cpu.addr &= 0x00ff;
    nes.cpu.is_imp = false;
    Ok(())
}

pub fn zpx<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.addr = (cpu::pc_fetch_byte(nes)?.wrapping_add(nes.cpu.x)) as u16;
    nes.cpu.addr &= 0x00ff;
    nes.cpu.is_imp = false;
    Ok(())
}

pub fn zpy<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    nes.cpu.addr = (cpu::pc_fetch_byte(nes)?.wrapping_add(nes.cpu.y)) as u16;
    nes.cpu.addr &= 0x00ff;
    nes.cpu.is_imp = false;
    Ok(())
}

pub fn xxx<S, A>(_nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    Ok(())
}
