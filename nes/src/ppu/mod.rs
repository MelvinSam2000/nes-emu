use anyhow::anyhow;
use anyhow::Result;

use crate::buscpu;
use crate::busppu::read;
use crate::busppu::write;
use crate::cartridge::Mirroring;
use crate::cpu;
use crate::nesaudio::NesAudio;
use crate::nesscreen::NesScreen;
use crate::ppu::regs::RegControl;
use crate::ppu::regs::RegMask;
use crate::ppu::regs::RegScroll;
use crate::ppu::regs::RegStatus;
use crate::Nes;

pub struct Ppu {
    pub oam: [u8; 256],
    // screen scanning
    pub scan_line: i16,
    pub scan_cycle: u16,
    // ppu registers for cpu communication
    pub reg_control: RegControl,
    pub reg_mask: RegMask,
    pub reg_status: RegStatus,
    pub reg_scroll: RegScroll,
    pub reg_addr: u16,
    pub reg_data: u8,
    pub addr_latch: bool,
    pub reg_oam_addr: u8,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            oam: [0; 256],

            scan_line: -1,
            scan_cycle: 0,

            reg_control: RegControl::default(),
            reg_mask: RegMask::default(),
            reg_status: RegStatus::default(),
            reg_scroll: RegScroll::default(),

            reg_addr: 0x0000,
            reg_data: 0x00,
            addr_latch: true,

            reg_oam_addr: 0x00,
        }
    }
}

const PALETTE_TO_RGB: [(u8, u8, u8); 64] = [
    (0x80, 0x80, 0x80),
    (0x00, 0x3d, 0xa6),
    (0x00, 0x12, 0xb0),
    (0x44, 0x00, 0x96),
    (0xa1, 0x00, 0x5e),
    (0xc7, 0x00, 0x28),
    (0xba, 0x06, 0x00),
    (0x8c, 0x17, 0x00),
    (0x5c, 0x2f, 0x00),
    (0x10, 0x45, 0x00),
    (0x05, 0x4a, 0x00),
    (0x00, 0x47, 0x2e),
    (0x00, 0x41, 0x66),
    (0x00, 0x00, 0x00),
    (0x05, 0x05, 0x05),
    (0x05, 0x05, 0x05),
    (0xc7, 0xc7, 0xc7),
    (0x00, 0x77, 0xff),
    (0x21, 0x55, 0xff),
    (0x82, 0x37, 0xfa),
    (0xeb, 0x2f, 0xb5),
    (0xff, 0x29, 0x50),
    (0xff, 0x22, 0x00),
    (0xd6, 0x32, 0x00),
    (0xc4, 0x62, 0x00),
    (0x35, 0x80, 0x00),
    (0x05, 0x8f, 0x00),
    (0x00, 0x8a, 0x55),
    (0x00, 0x99, 0xcc),
    (0x21, 0x21, 0x21),
    (0x09, 0x09, 0x09),
    (0x09, 0x09, 0x09),
    (0xff, 0xff, 0xff),
    (0x0f, 0xd7, 0xff),
    (0x69, 0xa2, 0xff),
    (0xd4, 0x80, 0xff),
    (0xff, 0x45, 0xf3),
    (0xff, 0x61, 0x8b),
    (0xff, 0x88, 0x33),
    (0xff, 0x9c, 0x12),
    (0xfa, 0xbc, 0x20),
    (0x9f, 0xe3, 0x0e),
    (0x2b, 0xf0, 0x35),
    (0x0c, 0xf0, 0xa4),
    (0x05, 0xfb, 0xff),
    (0x5e, 0x5e, 0x5e),
    (0x0d, 0x0d, 0x0d),
    (0x0d, 0x0d, 0x0d),
    (0xff, 0xff, 0xff),
    (0xa6, 0xfc, 0xff),
    (0xb3, 0xec, 0xff),
    (0xda, 0xab, 0xeb),
    (0xff, 0xa8, 0xf9),
    (0xff, 0xab, 0xb3),
    (0xff, 0xd2, 0xb0),
    (0xff, 0xef, 0xa6),
    (0xff, 0xf7, 0x9c),
    (0xd7, 0xe8, 0x95),
    (0xa6, 0xed, 0xaf),
    (0xa2, 0xf2, 0xda),
    (0x99, 0xff, 0xfc),
    (0xdd, 0xdd, 0xdd),
    (0x11, 0x11, 0x11),
    (0x11, 0x11, 0x11),
];

const PPUCTRL: u16 = 0x2000;
const PPUMASK: u16 = 0x2001;
const PPUSTATUS: u16 = 0x2002;
const OAMADDR: u16 = 0x2003;
const OAMDATA: u16 = 0x2004;
const PPUSCROLL: u16 = 0x2005;
const PPUADDR: u16 = 0x2006;
const PPUDATA: u16 = 0x2007;
const OAMDMA: u16 = 0x4014;

/*
    MAIN PPU CLOCK
*/

pub struct ViewPort {
    pub x1: u8,
    pub y1: u8,
    pub x2: u8,
    pub y2: u8,
}

impl From<[u8; 4]> for ViewPort {
    fn from(other: [u8; 4]) -> Self {
        ViewPort {
            x1: other[0],
            y1: other[1],
            x2: other[2],
            y2: other[3],
        }
    }
}

pub fn clock<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    // Enter VBLANK
    if nes.ppu.scan_line == 241 && nes.ppu.scan_cycle == 1 {
        nes.ppu.reg_status.set_vblank(true);
        nes.ppu.reg_status.set_sprite_0_hit(false);
        //let (back, front) = split_sprites_back_and_front(nes);
        render_background(nes)?;
        render_sprites(nes)?;
        //render_sprites(nes, &front)?;
        nes.screen.vblank()?;
        if nes.ppu.reg_control.is_nmi_enabled() {
            cpu::nmi(nes)?;
        }
    }

    nes.ppu.scan_cycle += 1;
    if nes.ppu.scan_cycle >= 341 {
        if is_sprite_0_hit(&nes.ppu, nes.ppu.scan_cycle) {
            nes.ppu.reg_status.set_sprite_0_hit(true);
        }

        nes.ppu.scan_cycle = 0;
        nes.ppu.scan_line += 1;
        if nes.ppu.scan_line >= 261 {
            nes.ppu.scan_line = -1;
            nes.ppu.reg_status.set_sprite_0_hit(false);
            nes.ppu.reg_status.set_vblank(false);
        }
    }
    Ok(())
}

/*
    PPU BUS FUNCTIONS
*/

pub fn read_ppu_reg<S, A>(nes: &mut Nes<S, A>, addr: u16) -> Result<u8> {
    match addr {
        PPUCTRL | PPUMASK | PPUSCROLL | PPUADDR | OAMADDR | OAMDMA => {
            // these registers are write only
            log::warn!("Ppu register at {:#x} is write only!", addr);
            Ok(0)
        }
        PPUSTATUS => {
            let data = nes.ppu.reg_status.get_bits();
            nes.ppu.reg_status.set_vblank(false);
            nes.ppu.addr_latch = true;
            nes.ppu.reg_scroll.latch = false;
            Ok(data)
        }
        OAMDATA => Ok(nes.ppu.oam[nes.ppu.reg_oam_addr as usize]),
        PPUDATA => {
            let maddr = nes.ppu.reg_addr;
            nes.ppu.reg_addr =
                nes.ppu
                    .reg_addr
                    .wrapping_add(if nes.ppu.reg_control.is_inc_mode() {
                        32
                    } else {
                        1
                    });

            match maddr {
                0x0000..=0x2fff => {
                    let output = nes.ppu.reg_data;
                    nes.ppu.reg_data = read(nes, maddr)?;
                    Ok(output)
                }
                0x3f00..=0x3fff => read(nes, maddr),
                _ => Err(anyhow!("Invalid read of PPU REG ADDR: {:#x}", maddr)),
            }
        }
        _ => Err(anyhow!("No PPU register can be read at {:#x}", addr)),
    }
}

pub fn write_ppu_reg<S, A>(nes: &mut Nes<S, A>, addr: u16, data: u8) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    match addr {
        PPUSTATUS => {
            log::warn!("PPUSTATUS register is read only!");
        }
        PPUCTRL => {
            nes.ppu.reg_control.update(data);
        }
        PPUMASK => {
            nes.ppu.reg_mask.update(data);
        }
        PPUSCROLL => {
            nes.ppu.reg_scroll.write(data);
        }
        OAMADDR => {
            nes.ppu.reg_oam_addr = data;
        }
        OAMDATA => {
            nes.ppu.oam[nes.ppu.reg_oam_addr as usize] = data;
            nes.ppu.reg_oam_addr = nes.ppu.reg_oam_addr.wrapping_add(1);
        }
        PPUADDR => {
            if nes.ppu.addr_latch {
                // set high byte
                nes.ppu.reg_addr = (nes.ppu.reg_addr & 0x00ff) | (data as u16) << 8;
            } else {
                // set low byte
                nes.ppu.reg_addr = (nes.ppu.reg_addr & 0xff00) | data as u16;
            }

            nes.ppu.reg_addr &= 0x3fff;

            nes.ppu.addr_latch = !nes.ppu.addr_latch;
        }
        PPUDATA => {
            write(nes, nes.ppu.reg_addr, data)?;
            nes.ppu.reg_addr =
                nes.ppu
                    .reg_addr
                    .wrapping_add(if nes.ppu.reg_control.is_inc_mode() {
                        32
                    } else {
                        1
                    });
        }
        OAMDMA => {
            let page: u16 = (data as u16) << 8;
            nes.cpu.cycles = 0xff;
            for i in 0..256 {
                nes.ppu.oam[nes.ppu.reg_oam_addr as usize] = buscpu::read(nes, page + i)?;
                nes.ppu.reg_oam_addr = nes.ppu.reg_oam_addr.wrapping_add(1);
            }
        }
        _ => {
            Err(anyhow!("No PPU register can be written at {:#x}", addr))?;
        }
    }
    Ok(())
}

/*
    RENDERING FUNCTIONS
*/

pub fn render_background<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    let (nt1, nt2) = match (
        &nes.cartridge.mirroring,
        nes.ppu.reg_control.nametable_offset(),
    ) {
        (Mirroring::Vertical, 0x000)
        | (Mirroring::Vertical, 0x800)
        | (Mirroring::Horizontal, 0x000)
        | (Mirroring::Horizontal, 0x400) => (0x2000, 0x2400),
        (Mirroring::Vertical, 0x400)
        | (Mirroring::Vertical, 0xc00)
        | (Mirroring::Horizontal, 0x800)
        | (Mirroring::Horizontal, 0xc00) => (0x2400, 0x2000),
        _ => Err(anyhow!(
            "Unsupported Mirroring type: {:?}",
            &nes.cartridge.mirroring
        ))?,
    };
    let scroll_x = nes.ppu.reg_scroll.scroll_x;
    let scroll_y = nes.ppu.reg_scroll.scroll_y;

    render_nametable(
        nes,
        nt1,
        ViewPort::from([scroll_x, scroll_y, 255, 240]),
        -(scroll_x as isize),
        -(scroll_y as isize),
    )?;
    if scroll_x > 0 {
        render_nametable(
            nes,
            nt2,
            ViewPort::from([0, 0, scroll_x, 240]),
            256 - scroll_x as isize,
            0,
        )?;
    } else if scroll_y > 0 {
        render_nametable(
            nes,
            nt2,
            ViewPort::from([0, 0, 255, scroll_y]),
            0,
            240 - scroll_y as isize,
        )?;
    }

    Ok(())
}

#[allow(dead_code)]
fn split_sprites_back_and_front<S, A>(nes: &mut Nes<S, A>) -> (Vec<u8>, Vec<u8>)
where
    S: NesScreen,
    A: NesAudio,
{
    let mut back = vec![];
    let mut front = vec![];
    for i in (0..255).step_by(4).rev() {
        if nes.ppu.oam[i as usize + 2] >> 5 & 1 != 0 {
            back.push(i);
        } else {
            front.push(i);
        }
    }
    (back, front)
}

pub fn render_sprites<S, A>(nes: &mut Nes<S, A>) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    for i in (0..=255).step_by(4) {
        let tile_id = nes.ppu.oam[i + 1];
        let tile_x = nes.ppu.oam[i + 3];
        let tile_y = nes.ppu.oam[i];
        let tile_attr = nes.ppu.oam[i + 2];

        let flip_v = tile_attr >> 7 & 1 != 0;
        let flip_h = tile_attr >> 6 & 1 != 0;
        let palette_id = tile_attr & 0b11;

        let spr_height_16 = nes.ppu.reg_control.spr_height_16();
        let spr_height_8_offset = (nes.ppu.reg_control.get_spr() as u16) * 0x1000;

        let mut render_sprite = |tile: u16, tile_y: u8| -> Result<()> {
            for y in 0..=7 {
                let mut upper = read(nes, tile + y as u16)?;
                let mut lower = read(nes, tile + y as u16 + 8)?;
                for x in (0..=7).rev() {
                    let value = (1 & lower) << 1 | (1 & upper);
                    upper >>= 1;
                    lower >>= 1;

                    if value == 0 {
                        continue;
                    }

                    let pal_pixel_id = 0x11 + palette_id * 4 + value - 1;
                    let mut rgb = PALETTE_TO_RGB[read(nes, 0x3f00 + pal_pixel_id as u16)? as usize];
                    emphasis(&nes.ppu.reg_mask, &mut rgb);

                    // FIX SPRITES WITH 16x8 HEIGHT
                    /*
                    let (pixel_x, pixel_y) = if !spr_height_16 {
                        match (flip_h, flip_v) {
                            (false, false) => (tile_x.wrapping_add(x), tile_y.wrapping_add(y)),
                            (true, false) => (tile_x.wrapping_add(7 - x), tile_y.wrapping_add(y)),
                            (false, true) => {
                                (tile_x.wrapping_add(x), tile_y.wrapping_add(7 - y as u8))
                            }
                            (true, true) => {
                                (tile_x.wrapping_add(7 - x), tile_y.wrapping_add(7 - y as u8))
                            }
                        }
                    } else {
                        match (flip_h, flip_v) {
                            (false, false) => (tile_x.wrapping_add(x), tile_y.wrapping_add(y)),
                            (true, false) => (tile_x.wrapping_add(7 - x), tile_y.wrapping_add(y)),
                            (false, true) => {
                                (tile_x.wrapping_add(x), tile_y.wrapping_add(7 - y as u8))
                            }
                            (true, true) => {
                                (tile_x.wrapping_add(7 - x), tile_y.wrapping_add(7 - y as u8))
                            }
                        }
                    };
                    */
                    let (pixel_x, pixel_y) = match (flip_h, flip_v) {
                        (false, false) => (tile_x.wrapping_add(x), tile_y.wrapping_add(y)),
                        (true, false) => (tile_x.wrapping_add(7 - x), tile_y.wrapping_add(y)),
                        (false, true) => (tile_x.wrapping_add(x), tile_y.wrapping_add(7 - y as u8)),
                        (true, true) => {
                            (tile_x.wrapping_add(7 - x), tile_y.wrapping_add(7 - y as u8))
                        }
                    };

                    if pixel_y < 240 {
                        nes.screen.draw_pixel(pixel_x, pixel_y, rgb)?;
                    }
                }
            }
            Ok(())
        };

        if !spr_height_16 {
            render_sprite(spr_height_8_offset + tile_id as u16 * 16, tile_y)?;
        } else {
            let tile = tile_id as u16 & 0xfe;
            render_sprite(tile * 16, tile_y)?;
            render_sprite(
                (tile + if tile_id & 1 != 0 { 256 } else { 1 }) * 16,
                tile_y + 1,
            )?;
        }
    }
    Ok(())
}

/*
    UTILITY FUNCTIONS
*/

pub fn render_nametable<S, A>(
    nes: &mut Nes<S, A>,
    nt_base: u16,
    view_port: ViewPort,
    scroll_x: isize,
    scroll_y: isize,
) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    let chr_bank = (nes.ppu.reg_control.get_bg() as u16) * 0x1000;
    // First nametable
    for nt_offset in 0x000..=0x3bf {
        let nt_addr = nt_base + nt_offset;
        // get tile ID from vram
        let tile = read(nes, nt_addr)?;
        let tile_col = nt_offset % 32;
        let tile_row = nt_offset / 32;

        let attr_table_idx = tile_row / 4 * 8 + tile_col / 4;
        let attr_byte = read(nes, nt_base + 0x3c0 + attr_table_idx)?;

        let palette_idx = match (tile_col % 4 / 2, tile_row % 4 / 2) {
            (0, 0) => attr_byte & 0b11,
            (1, 0) => (attr_byte >> 2) & 0b11,
            (0, 1) => (attr_byte >> 4) & 0b11,
            (1, 1) => (attr_byte >> 6) & 0b11,
            _ => unreachable!(),
        };
        let palette_start = 4 * palette_idx;

        // Draw tile
        for row in 0..8 {
            let mut tile_lsb = read(nes, chr_bank + (tile as u16) * 16 + row)?;
            let mut tile_msb = read(nes, chr_bank + (tile as u16) * 16 + row + 8)?;
            for col in 0..8 {
                let pixel = ((tile_msb & 0x01) << 1) | (tile_lsb & 0x01);
                tile_lsb >>= 1;
                tile_msb >>= 1;
                let palette_idx = match pixel {
                    0 => read(nes, 0x3f00)?,
                    1 | 2 | 3 => read(nes, 0x3f00 + palette_start as u16 + pixel as u16)?,
                    _ => unreachable!(),
                };

                let pixel_x = (tile_col * 8 + (7 - col)) as u8;
                let pixel_y = (tile_row * 8 + row) as u8;
                if pixel_x >= view_port.x1
                    && pixel_x < view_port.x2
                    && pixel_y >= view_port.y1
                    && pixel_y < view_port.y2
                {
                    let mut rgb = PALETTE_TO_RGB[palette_idx as usize];
                    emphasis(&nes.ppu.reg_mask, &mut rgb);
                    nes.screen.draw_pixel(
                        (scroll_x + pixel_x as isize) as u8,
                        (scroll_y + pixel_y as isize) as u8,
                        rgb,
                    )?;
                }
            }
        }
    }
    Ok(())
}

pub fn emphasis(rmask: &RegMask, rgb: &mut (u8, u8, u8)) {
    if rmask.emphasis_r() {
        rgb.2 = (1.1 * (rgb.2 as f32)) as u8;
        rgb.1 = (0.9 * (rgb.1 as f32)) as u8;
        rgb.0 = (0.9 * (rgb.0 as f32)) as u8;
    }
    if rmask.emphasis_g() {
        rgb.2 = (0.9 * (rgb.2 as f32)) as u8;
        rgb.1 = (1.1 * (rgb.1 as f32)) as u8;
        rgb.0 = (0.9 * (rgb.0 as f32)) as u8;
    }
    if rmask.emphasis_b() {
        rgb.2 = (0.9 * (rgb.2 as f32)) as u8;
        rgb.1 = (0.9 * (rgb.1 as f32)) as u8;
        rgb.0 = (1.1 * (rgb.0 as f32)) as u8;
    }
}

fn is_sprite_0_hit(ppu: &Ppu, cycle: u16) -> bool {
    let y = ppu.oam[0] as usize;
    let x = ppu.oam[3] as usize;
    y == ppu.scan_line as usize && x <= cycle as usize && ppu.reg_mask.render_spr_enabled()
}

/*
    DEBUG FUNCTIONS
*/

pub fn draw_chr<S, A>(nes: &mut Nes<S, A>, bank: u16, dbg_screen: &mut impl NesScreen) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    for tile_x in 0..16 {
        for tile_y in 0..16 {
            let offset = tile_x * 256 + tile_y * 16;
            for row in 0..8 {
                let mut tile_lsb = read(nes, bank * 0x1000 + offset + row)?;
                let mut tile_msb = read(nes, bank * 0x1000 + offset + row + 8)?;
                for col in 0..8 {
                    let pixel = (tile_msb & 0x01) + (tile_lsb & 0x01);
                    tile_lsb >>= 1;
                    tile_msb >>= 1;

                    let mut rgb = PALETTE_TO_RGB[read(nes, 0x3f00 + pixel as u16)? as usize];
                    emphasis(&nes.ppu.reg_mask, &mut rgb);
                    dbg_screen.draw_pixel(
                        (tile_y * 8 + (7 - col)) as u8,
                        (tile_x * 8 + row) as u8,
                        rgb,
                    )?;
                }
            }
        }
    }
    Ok(())
}

pub fn draw_vram<S, A>(
    nes: &mut Nes<S, A>,
    screen_no: usize,
    dbg_screen: &mut impl NesScreen,
) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    let (nt_start, attr_start) = match screen_no {
        0 => (0x2000, 0x23c0),
        1 => (0x2400, 0x27c0),
        2 => (0x2800, 0x2bc0),
        3 => (0x2c00, 0x2fc0),
        _ => Err(anyhow!("Invalid screen number..."))?,
    };

    let chr_bank = nes.ppu.reg_control.get_bg() as u16;
    // First nametable
    for i in nt_start..attr_start {
        // get tile ID from vram
        let tile = read(nes, i)?;
        let tile_col = (i - nt_start) % 32;
        let tile_row = (i - nt_start) / 32;

        let attr_table_idx = tile_row / 4 * 8 + tile_col / 4;
        let attr_byte = read(nes, attr_start + attr_table_idx)?;

        let palette_idx = match (tile_col % 4 / 2, tile_row % 4 / 2) {
            (0, 0) => attr_byte & 0b11,
            (1, 0) => (attr_byte >> 2) & 0b11,
            (0, 1) => (attr_byte >> 4) & 0b11,
            (1, 1) => (attr_byte >> 6) & 0b11,
            (_, _) => Err(anyhow!("Invalid palette index..."))?,
        };
        let palette_start = 4 * palette_idx;

        // Draw tile
        for row in 0..8 {
            let mut tile_lsb = read(nes, chr_bank * 0x1000 + (tile as u16) * 16 + row)?;
            let mut tile_msb = read(nes, chr_bank * 0x1000 + (tile as u16) * 16 + row + 8)?;
            for col in 0..8 {
                let pixel = ((tile_msb & 0x01) << 1) | (tile_lsb & 0x01);
                tile_lsb >>= 1;
                tile_msb >>= 1;
                let palette_idx = match pixel {
                    0 => read(nes, 0x3f00)?,
                    1 | 2 | 3 => read(nes, 0x3f00 + palette_start as u16 + pixel as u16)?,
                    _ => 0,
                };
                let mut rgb = PALETTE_TO_RGB[palette_idx as usize];
                emphasis(&nes.ppu.reg_mask, &mut rgb);
                dbg_screen.draw_pixel(
                    (tile_col * 8 + (7 - col)) as u8,
                    (tile_row * 8 + row) as u8,
                    rgb,
                )?;
            }
        }
    }
    Ok(())
}

pub fn draw_palette<S, A>(nes: &mut Nes<S, A>, dbg_screen: &mut impl NesScreen) -> Result<()>
where
    S: NesScreen,
    A: NesAudio,
{
    for i in 0..32 {
        let mut rgb = PALETTE_TO_RGB[read(nes, 0x3f00 + i as u16)? as usize % 64];
        emphasis(&nes.ppu.reg_mask, &mut rgb);
        dbg_screen.draw_pixel(i % 16, i / 16, rgb)?;
    }
    Ok(())
}

pub mod regs;
