use std::cell::RefCell;
use std::rc::Rc;

use anyhow::anyhow;
use anyhow::Result;

use crate::mappers::cnrom::Cnrom;
use crate::mappers::gxrom::Gxrom;
use crate::mappers::mmc1::Mmc1;
use crate::mappers::nrom::Nrom;
use crate::mappers::uxrom::Uxrom;
use crate::mappers::Mapper;
use crate::Nes;

pub struct Cartridge<S, A> {
    pub prgmem: Vec<u8>,
    pub chrmem: Vec<u8>,
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub mapper: Rc<RefCell<dyn Mapper<S, A>>>,
    pub mirroring: Mirroring,
}

#[derive(Debug)]
pub enum Mirroring {
    HORIZONTAL,
    VERTICAL,
    //ONESCREEN_LO,
    //ONESCREEN_HI,
}

impl<S, A> Default for Cartridge<S, A> {
    fn default() -> Self {
        Self {
            prgmem: vec![],
            chrmem: vec![],
            prg_banks: 0,
            chr_banks: 0,
            mapper: Rc::new(RefCell::new(Nrom)),
            mirroring: Mirroring::HORIZONTAL,
        }
    }
}

pub fn load_cartridge<S, A>(nes: &mut Nes<S, A>, rom_bytes: &[u8]) -> Result<()> {
    if &rom_bytes[0..3] != b"NES" {
        Err(anyhow!(
            "Invalid NES ROM was provided: Missing magic NES header"
        ))?;
    }

    // read file header
    let prg_banks = rom_bytes[0x4];
    let chr_banks = rom_bytes[0x5];

    let trainer_is_present = rom_bytes[0x6] & 0x04 != 0;
    let mirroring = rom_bytes[0x6] & 0x01 != 0;
    let prg_size = 0x4000 * prg_banks as u64;
    let chr_size = 0x2000 * chr_banks as u64;

    nes.cartridge.mirroring = if !mirroring {
        Mirroring::HORIZONTAL
    } else {
        Mirroring::VERTICAL
    };
    log::info!("Mirroring: {:?}", nes.cartridge.mirroring);

    // resize cartridge roms
    nes.cartridge.prg_banks = prg_banks as u8;
    nes.cartridge.chr_banks = chr_banks as u8;
    nes.cartridge.prgmem.resize(prg_size as usize, 0);
    nes.cartridge.chrmem.resize(chr_size as usize, 0);
    if chr_size == 0 {
        nes.cartridge.chrmem.resize(0x2000, 0);
    }
    log::info!("PRG banks: {}", prg_banks);
    log::info!("CHR banks: {}", chr_banks);

    // choose mapper
    let mapper_id = (rom_bytes[0x7] & 0xf0) | ((rom_bytes[0x6] & 0xf0) >> 4);
    nes.cartridge.mapper = match mapper_id {
        0 => Rc::new(RefCell::new(Nrom)),
        1 => Rc::new(RefCell::new(Mmc1::new())),
        2 => Rc::new(RefCell::new(Uxrom::default())),
        3 => Rc::new(RefCell::new(Cnrom::default())),
        66 => Rc::new(RefCell::new(Gxrom::default())),
        _ => Err(anyhow!("Mapper {} not supported yet...", mapper_id))?,
    };
    log::info!("Loaded Mapper {}", mapper_id);

    // fill memories
    let mut offset = 16;
    if trainer_is_present {
        offset += 512;
    }
    for i in 0..prg_size as u64 {
        nes.cartridge.prgmem[i as usize] = rom_bytes[(offset + i) as usize];
    }

    for i in 0..chr_size as u64 {
        nes.cartridge.chrmem[i as usize] = rom_bytes[(prg_size + offset + i) as usize];
    }
    Ok(())
}

pub fn prg_read<S, A>(nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
    let mapper = nes.cartridge.mapper.clone();
    let mut mapper_ref = mapper.borrow_mut();
    mapper_ref.read_prg(nes, addr)
}

pub fn prg_write<S, A>(nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()> {
    let mapper = nes.cartridge.mapper.clone();
    let mut mapper_ref = mapper.borrow_mut();
    mapper_ref.write_prg(nes, addr, data)
}

pub fn chr_read<S, A>(nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
    let mapper = nes.cartridge.mapper.clone();
    let mut mapper_ref = mapper.borrow_mut();
    mapper_ref.read_chr(nes, addr)
}

pub fn chr_write<S, A>(nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()> {
    let mapper = nes.cartridge.mapper.clone();
    let mut mapper_ref = mapper.borrow_mut();
    mapper_ref.write_chr(nes, addr, data)
}
