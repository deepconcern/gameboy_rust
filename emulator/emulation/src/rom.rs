use std::collections::HashMap;

use num::ToPrimitive;

// Restarts
const RESTART_DATA_SIZE: u8 = 0x8u8;
const RESTART_AREA_START_ADDRESS: u16 = 0x0000u16;
const RESTART_AREA_END_ADDRESS: u16 = 0x003fu16;
const RESTART_ZERO_ADDRESS: u16 = 0x0000u16;
const RESTART_ONE_ADDRESS: u16 = 0x0008u16;
const RESTART_TWO_ADDRESS: u16 = 0x0010u16;
const RESTART_THREE_ADDRESS: u16 = 0x0018u16;
const RESTART_FOUR_ADDRESS: u16 = 0x0020u16;
const RESTART_FIVE_ADDRESS: u16 = 0x0028u16;
const RESTART_SIX_ADDRESS: u16 = 0x0030u16;
const RESTART_SEVEN_ADDRESS: u16 = 0x0038u16;

// Interrupts
const INTERRUPT_DATA_SIZE: u8 = 0x8u8;
const INTERRUPT_AREA_START_ADDRESS: u16 = 0x0040u16;
const INTERRUPT_AREA_END_ADDRESS: u16 = 0x0067u16;
const VERTICAL_BANKING_INTERRUPT_START_ADDRESS: u16 = 0x0040u16;
const LCDC_INTERRUPT_START_ADDRESS: u16 = 0x0048u16;
const TIMER_OVERFLOW_INTERRUPT_START_ADDRESS: u16 = 0x0050u16;
const SERIAL_TRANSFER_COMPLETION_INTERRUPT_START_ADDRESS: u16 = 0x0058u16;
const TERMINAL_NEGATIVE_EDGE_INTERRUPT_START_ADDRESS: u16 = 0x0060u16;

// Header
const HEADER_START_ADDRESS: u16 = 0x0100u16;
const HEADER_END_ADDRESS: u16 = 0x014fu16;
const INITIAL_INSTRUCTION_ADDRESS: u16 = 0x0100u16;
const JUMP_INSTRUCTION_ADDRESS: u16 = 0x0101u16;
const JUMP_TARGET_LOW_ADDRESS: u16 = 0x0102u16;
const JUMP_TARGET_HIGH_ADDRESS: u16 = 0x0103u16;

// Program area
const PROGRAM_AREA_END_ADDRESS: u16 = 0x7fffu16;

// Markers
const CGB_COMPATIBILITY_ADDRESS: u16 = 0x0143u16;
const GAME_TITLE_END_ADDRESS: u16 = 0x0142u16;
const GAME_TITLE_START_ADDRESS: u16 = 0x0134u16;
const NEW_MAKER_CODE_ADDRESS_HIGH: u16 = 0x0144u16;
const NEW_MAKER_CODE_ADDRESS_LOW: u16 = 0x0145u16;
const OLD_MAKER_CODE_ADDRESS: u16 = 0x014bu16;
const USE_NEW_MAKER_CODE_VALUE: u16 = 0x33u16;

// Maps
lazy_static! {
    static ref NEW_MAKER_CODES: HashMap<u8, String> = {
        let mut new_maker_codes = HashMap::new();

        new_maker_codes.insert(0x00u8, String::from("none"));
        new_maker_codes.insert(0x01u8, String::from("nintendo"));
        new_maker_codes.insert(0x08u8, String::from("capcom"));
        new_maker_codes.insert(0x13u8, String::from("electronic arts"));
        new_maker_codes.insert(0x18u8, String::from("hudsonsoft"));
        new_maker_codes.insert(0x19u8, String::from("b-ai"));
        new_maker_codes.insert(0x20u8, String::from("kss"));
        new_maker_codes.insert(0x22u8, String::from("pow"));
        new_maker_codes.insert(0x24u8, String::from("pcm complete"));
        new_maker_codes.insert(0x25u8, String::from("san-x"));
        new_maker_codes.insert(0x28u8, String::from("kemco japan"));
        new_maker_codes.insert(0x29u8, String::from("seta"));
        new_maker_codes.insert(0x30u8, String::from("viacom"));
        new_maker_codes.insert(0x31u8, String::from("nintendo"));
        new_maker_codes.insert(0x32u8, String::from("bandia"));
        new_maker_codes.insert(0x33u8, String::from("ocean/acclaim"));
        new_maker_codes.insert(0x34u8, String::from("konami"));
        new_maker_codes.insert(0x35u8, String::from("hector"));
        new_maker_codes.insert(0x37u8, String::from("taito"));
        new_maker_codes.insert(0x38u8, String::from("hudson"));
        new_maker_codes.insert(0x39u8, String::from("banpresto"));
        new_maker_codes.insert(0x41u8, String::from("ubi soft"));
        new_maker_codes.insert(0x42u8, String::from("atlus"));
        new_maker_codes.insert(0x44u8, String::from("malibu"));
        new_maker_codes.insert(0x46u8, String::from("angel"));
        new_maker_codes.insert(0x47u8, String::from("pullet-proof"));
        new_maker_codes.insert(0x49u8, String::from("irem"));
        new_maker_codes.insert(0x50u8, String::from("absolute"));
        new_maker_codes.insert(0x51u8, String::from("acclaim"));
        new_maker_codes.insert(0x52u8, String::from("activision"));
        new_maker_codes.insert(0x53u8, String::from("american sammy"));
        new_maker_codes.insert(0x54u8, String::from("konami"));
        new_maker_codes.insert(0x55u8, String::from("hi tech entertainment"));
        new_maker_codes.insert(0x56u8, String::from("ljn"));
        new_maker_codes.insert(0x57u8, String::from("matchbox"));
        new_maker_codes.insert(0x58u8, String::from("mattel"));
        new_maker_codes.insert(0x59u8, String::from("milton bradley"));
        new_maker_codes.insert(0x60u8, String::from("titus"));
        new_maker_codes.insert(0x61u8, String::from("virgin"));
        new_maker_codes.insert(0x64u8, String::from("lucasarts"));
        new_maker_codes.insert(0x67u8, String::from("ocean"));
        new_maker_codes.insert(0x69u8, String::from("electronic arts"));
        new_maker_codes.insert(0x70u8, String::from("infogrames"));
        new_maker_codes.insert(0x71u8, String::from("interplay"));
        new_maker_codes.insert(0x72u8, String::from("broderbund"));
        new_maker_codes.insert(0x73u8, String::from("sculptured"));
        new_maker_codes.insert(0x75u8, String::from("sci"));
        new_maker_codes.insert(0x78u8, String::from("t*hq"));
        new_maker_codes.insert(0x79u8, String::from("accolade"));
        new_maker_codes.insert(0x80u8, String::from("misawa"));
        new_maker_codes.insert(0x83u8, String::from("lozc"));
        new_maker_codes.insert(0x86u8, String::from("tokuma shoten i*"));
        new_maker_codes.insert(0x87u8, String::from("tsukuda ori*"));
        new_maker_codes.insert(0x91u8, String::from("chun soft"));
        new_maker_codes.insert(0x92u8, String::from("video system"));
        new_maker_codes.insert(0x93u8, String::from("ocean/acclaim"));
        new_maker_codes.insert(0x95u8, String::from("varie"));
        new_maker_codes.insert(0x96u8, String::from("yonezawa/s'pal"));
        new_maker_codes.insert(0x97u8, String::from("kaneko"));
        new_maker_codes.insert(0x99u8, String::from("pack in soft "));

        new_maker_codes
    };
}

#[derive(ToPrimitive)]
#[repr(u8)]
pub enum CbgCompatibility {
    CGBIncompatible = 0x00u8,
    CGBCompatible = 0x80u8,
    CGBExclusive = 0xc0u8,
}

pub enum RomSize {
    Size256Kilobits = 256 * 1000,      // 256Kb / 32KB
    Size512Kilobits = 512 * 1000,      // 512Kb / 64KB
    Size1Megabits = 1 * 1000 * 1000,   // 1Mb / 128KB
    Size2Megabits = 2 * 1000 * 1000,   // 2Mb / 256KB
    Size4Megabits = 4 * 1000 * 1000,   // 4Mb / 512KB
    Size8Megabits = 8 * 1000 * 1000,   // 8Mb / 1MB
    Size16Megabits = 16 * 1000 * 1000, // 160b / 2MB
    Size32Megabits = 32 * 1000 * 1000, // 320b / 4MB
    Size64Megabits = 64 * 1000 * 1000, // 640b / 8MB
}

pub struct RomBuilder {
    cgb_compatibility: CbgCompatibility,
    game_title: Vec<u8>,
    program_data: Vec<u8>,
    rom_size: RomSize,
}

impl RomBuilder {
    pub fn new() -> Self {
        RomBuilder {
            cgb_compatibility: CbgCompatibility::CGBCompatible,
            game_title: Vec::new(),
            program_data: Vec::new(),
            rom_size: RomSize::Size256Kilobits,
        }
    }

    pub fn build(&self) -> Vec<u8> {
        let mut rom = vec![];

        for i in 0u16..(rom.len() as u16) {
            // Game title
            if i >= GAME_TITLE_START_ADDRESS && i <= GAME_TITLE_END_ADDRESS {
                let char_index = i - GAME_TITLE_START_ADDRESS;

                rom[i as usize] = self.game_title[char_index as usize];

                continue;
            }

            // CGB compatibility
            if i == CGB_COMPATIBILITY_ADDRESS {
                rom[i as usize] = self.cgb_compatibility.to_u8().unwrap();

                continue;
            }
        }

        rom
    }

    pub fn cgb_compatibility(&mut self, cgb_compatibility: CbgCompatibility) -> &mut Self {
        self.cgb_compatibility = cgb_compatibility;

        self
    }

    pub fn game_title(&mut self, game_title: String) -> &mut Self {
        self.game_title = game_title.as_bytes().to_vec();

        self
    }

    pub fn rom_size(&mut self, rom_size: RomSize) -> &mut Self {
        self.rom_size = rom_size;

        self
    }
}
