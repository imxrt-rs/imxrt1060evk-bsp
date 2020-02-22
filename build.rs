use imxrt_boot_gen::serial_flash::opcodes::sdr::*;
use imxrt_boot_gen::serial_flash::*;

/// Instructions for the ISSI IS25WP064A
/// SPI flash memory controller
mod issi {
    pub const FAST_READ_QUAD_IO: u8 = 0xEB;
    pub const READ_STATUS_REGISTER_1: u8 = 0x05;
    pub const WRITE_ENABLE: u8 = 0x06;
    pub const SECTOR_ERASE: u8 = 0x20;
    pub const PAGE_PROGRAM: u8 = 0x02;
    pub const CHIP_ERASE: u8 = 0x60;
}

use issi::*;

//
// Sequences for lookup table
//

const SEQ_READ: Sequence = Sequence([
    Instr::new(CMD, Pads::One, FAST_READ_QUAD_IO),
    Instr::new(RADDR, Pads::Four, 0x18),
    Instr::new(DUMMY, Pads::Four, 0x06),
    Instr::new(READ, Pads::Four, 0x04),
    STOP,
    STOP,
    STOP,
    STOP,
]);

const SEQ_READ_STATUS: Sequence = Sequence([
    Instr::new(CMD, Pads::One, READ_STATUS_REGISTER_1),
    Instr::new(READ, Pads::One, 0x04),
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
]);

const SEQ_WRITE_ENABLE: Sequence = Sequence([
    Instr::new(CMD, Pads::One, WRITE_ENABLE),
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
]);

const SEQ_ERASE_SECTOR: Sequence = Sequence([
    Instr::new(CMD, Pads::One, SECTOR_ERASE),
    Instr::new(RADDR, Pads::One, 0x18),
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
]);

const SEQ_PAGE_PROGRAM: Sequence = Sequence([
    Instr::new(CMD, Pads::One, PAGE_PROGRAM),
    Instr::new(RADDR, Pads::One, 0x18),
    Instr::new(WRITE, Pads::One, 0x04),
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
]);

const SEQ_CHIP_ERASE: Sequence = Sequence([
    Instr::new(CMD, Pads::One, CHIP_ERASE),
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
]);

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

static RERUN_IF_CHANGED: &[&str] = &["build.rs", "link.x"];

fn main() {
    for &rerun_if_changed in RERUN_IF_CHANGED.iter() {
        println!("cargo:rerun-if-changed={}", rerun_if_changed);
    }

    let nor_cb = nor::ConfigurationBlock {
        page_size: 256,
        sector_size: 4096,
        ip_cmd_serial_clk_freq: nor::SerialClockFrequency::MHz100,
    };
    let lookup_table = {
        use imxrt_boot_gen::serial_flash::CommandSequence::*;
        let mut lut = LookupTable::new();
        lut[Read] = SEQ_READ;
        lut[ReadStatus] = SEQ_READ_STATUS;
        lut[WriteEnable] = SEQ_WRITE_ENABLE;
        lut[EraseSector] = SEQ_ERASE_SECTOR;
        lut[PageProgram] = SEQ_PAGE_PROGRAM;
        lut[ChipErase] = SEQ_CHIP_ERASE;
        lut
    };
    let fcb = FCBBuilder::new(DeviceType::SerialNOR(nor_cb), lookup_table)
        .read_sample_clk_src(ReadSampleClockSource::LoopbackFromDQSPad)
        .cs_hold_time(0x03)
        .cs_setup_time(0x03)
        .flash_size(SerialFlashRegion::A1, 8*1024*1024)
        .serial_clk_freq(SerialClockFrequency::MHz100)
        .serial_flash_pad_type(FlashPadType::Quad)
        .build()
        .unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("fcb.rs");
    let mut f = File::create(&dest_path).unwrap();
    writeln!(f, "{}", fcb).unwrap();

    // linker script
    let link_out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let memory_x = include_bytes!("memory.x");
    let mut script = File::create(link_out_dir.join("memory.x")).unwrap();
    script.write_all(memory_x).unwrap();
    println!("cargo:rustc-link-search={}", link_out_dir.display());
}

