use ::nes::nesaudio::NesAudio;
use ::nes::nesscreen::NesScreen;
use ::nes::Nes;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use nes::buscpu;
use nes::busppu;
use nes::cpu::Cpu;
use nes::ppu::Ppu;
use regex::Regex;
use rs6502::Disassembler;

#[derive(Debug)]
pub enum Command {
    CpuRegs,
    Disassemble(u16, u16),
    CpuMemory(u16, u16),
    PpuMemory(u16, u16),
    PpuOam,
}

pub fn parse(s: &str) -> Result<Command> {
    if Regex::new(r"^cpu\n?$")?.is_match(s) {
        Ok(Command::CpuRegs)
    } else if Regex::new(r"^dasm [a-fA-F\d]{1,4} [a-fA-F\d]{1,4}\n?$")?.is_match(s) {
        let args = s.split(' ').collect::<Vec<&str>>();
        let addr_start = args
            .get(1)
            .map(|addr| u16::from_str_radix(addr, 16))
            .context("Invalid dasm args")??;
        let addr_end = args
            .get(2)
            .map(|addr| u16::from_str_radix(addr, 16))
            .context("Invalid dasm args")??;
        Ok(Command::Disassemble(addr_start, addr_end))
    } else if Regex::new(r"^cpumem [a-fA-F\d]{1,4} [a-fA-F\d]{1,4}\n?$")?.is_match(s) {
        let args = s.split(' ').collect::<Vec<&str>>();
        let addr_start = args
            .get(1)
            .map(|addr| u16::from_str_radix(addr, 16))
            .context("Invalid cpumem args")??;
        let addr_end = args
            .get(2)
            .map(|addr| u16::from_str_radix(addr, 16))
            .context("Invalid cpumem args")??;
        Ok(Command::CpuMemory(addr_start, addr_end))
    } else if Regex::new(r"^ppumem [a-fA-F\d]{1,4} [a-fA-F\d]{1,4}\n?$")?.is_match(s) {
        let args = s.split(' ').collect::<Vec<&str>>();
        let addr_start = args
            .get(1)
            .map(|addr| u16::from_str_radix(addr, 16))
            .context("Invalid ppumem args")??;
        let addr_end = args
            .get(2)
            .map(|addr| u16::from_str_radix(addr, 16))
            .context("Invalid ppumem args")??;
        Ok(Command::PpuMemory(addr_start, addr_end))
    } else if Regex::new(r"^oam\n?$")?.is_match(s) {
        Ok(Command::PpuOam)
    } else {
        Err(anyhow!("Invalid command: {}", s))
    }
}

pub fn exec<S, A>(cmd: Command, nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    println!("EXEC... {:#x?}", &cmd);
    match cmd {
        Command::CpuRegs => cpuregs(&nes.cpu),
        Command::Disassemble(addr_start, addr_end) => disassemble(addr_start, addr_end, nes)?,
        Command::CpuMemory(addr_start, addr_end) => cpumem(addr_start, addr_end, nes),
        Command::PpuMemory(addr_start, addr_end) => ppumem(addr_start, addr_end, nes),
        Command::PpuOam => oam(&nes.ppu),
    }
    Ok(())
}

// Print cpu registers
fn cpuregs(cpu: &Cpu) {
    println!(
        "A: {:#02x} X: {:#02x} Y: {:#02x} PC: {:#04x} SP: {:#02x} ",
        cpu.ac, cpu.x, cpu.y, cpu.pc, cpu.sp
    );
}

// Disassemble
fn disassemble<S, A>(addr_start: u16, addr_end: u16, nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    let dasm = Disassembler::with_offset(addr_start);
    let code = (addr_start..addr_end)
        .map(|addr| buscpu::read(nes, addr))
        .collect::<Result<Vec<u8>>>()?;
    let asm = dasm.disassemble(&code);
    println!("{}", asm);
    Ok(())
}

// Print raw memory as seen by the CPU bus
fn cpumem<S, A>(addr_start: u16, addr_end: u16, nes: &mut Nes<S, A>)
where
    S: NesScreen,
    A: NesAudio,
{
    (addr_start..addr_end).step_by(16).for_each(|addr| {
        let mut data_row = [0u8; 16];
        (0..16)
            .map(|offset| buscpu::read(nes, addr + offset).unwrap())
            .enumerate()
            .for_each(|(offset, data)| {
                data_row[offset] = data;
            });
        let data_row = format!(
            "{:04x}: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} \
             {:02x} {:02x} {:02x} {:02x} {:02x}",
            addr,
            data_row[0],
            data_row[1],
            data_row[2],
            data_row[3],
            data_row[4],
            data_row[5],
            data_row[6],
            data_row[7],
            data_row[8],
            data_row[9],
            data_row[10],
            data_row[11],
            data_row[12],
            data_row[13],
            data_row[14],
            data_row[15]
        );

        println!("{}", data_row);
    })
}

// Print raw memory as seen by the PPU bus
fn ppumem<S, A>(addr_start: u16, addr_end: u16, nes: &mut Nes<S, A>)
where
    S: NesScreen,
    A: NesAudio,
{
    (addr_start..addr_end).step_by(16).for_each(|addr| {
        let mut data_row = [0u8; 16];
        (0..16)
            .map(|offset| busppu::read(nes, addr + offset).unwrap())
            .enumerate()
            .for_each(|(offset, data)| {
                data_row[offset] = data;
            });
        let data_row = format!(
            "{:04x}: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} \
             {:02x} {:02x} {:02x} {:02x} {:02x}",
            addr,
            data_row[0],
            data_row[1],
            data_row[2],
            data_row[3],
            data_row[4],
            data_row[5],
            data_row[6],
            data_row[7],
            data_row[8],
            data_row[9],
            data_row[10],
            data_row[11],
            data_row[12],
            data_row[13],
            data_row[14],
            data_row[15]
        );

        println!("{}", data_row);
    })
}

// Print OAM contents
fn oam(ppu: &Ppu) {
    (0..=255).step_by(4).for_each(|i| {
        let id = ppu.oam[i + 1] as u16;
        let x = ppu.oam[i + 3];
        let y = ppu.oam[i];
        let attr = ppu.oam[i + 2];
        println!(
            "{:02} => ID: {:#04x}, x: {:#04x}, y: {:#04x}, attr: {:#010b}",
            i / 4,
            id,
            x,
            y,
            attr
        );
    })
}
