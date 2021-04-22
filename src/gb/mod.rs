mod cartridge;
mod cpu;
mod opcode;
pub use cartridge::Cartridge;
use cpu::LR35902;
use opcode::Opcode;
use std::fmt;

const MEMORY_SIZE: usize = 8192;

pub struct Gameboy {
    pub cartridge: Option<Cartridge>,
    pub cpu: LR35902,
    pub memory: [u8; MEMORY_SIZE],
}

impl Gameboy {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge: Some(cartridge),
            cpu: LR35902 { pc: 0x0100, sp: 0xFFFE, a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0 },
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn tick(&mut self) -> Result<u8, GameboyError> {
        if self.cartridge.is_none() {
            return Err(GameboyError::NoCartridge);
        }
        // TODO-PERF: check if as_ref() has a large amount of overhead?
        let cartridge = self.cartridge.as_ref().unwrap();
        let opcode = cartridge.read_bytes(self.cpu.pc, 1) as u8;
        let base_cycles = Opcode::base_cycles(opcode);
        let size = Opcode::size(opcode) as u16;
        let opcode = match Opcode::parse(opcode) {
            Some(op) => op,
            None => {
                return Err(GameboyError::InvalidOpcode { opcode, address: self.cpu.pc });
            }
        };
        // TODO-PERF: avoid re-reading memory here, but it's the simplest solution atm.
        let instruction = cartridge.read_bytes(self.cpu.pc, size);
        let mut skip_pc = false;
        match opcode {
            Opcode::NOP => (),
            Opcode::LD_BC_d16 => {
                self.cpu.set_bc((instruction & 0xFFFF) as u16);
            },
            Opcode::LD_BC_A => {
                self.memory[self.cpu.bc() as usize] = self.cpu.a;
            },
            Opcode::INC_BC => {
            },
            Opcode::INC_B => {
            },
            Opcode::DEC_B => {
            },
            Opcode::LD_B_d8 => {
                self.cpu.b = (instruction & 0xFF) as u8;
            },
            Opcode::RLCA => {
                // TODO-Q: Is this the right thing to do? How do we set Z?
                self.cpu.set_flags(1, 0, 0, self.cpu.z());
            },
            Opcode::LD_a16_SP => {
            },
            Opcode::ADD_HL_BC => {
            },
            Opcode::LD_A_BC => {
                self.cpu.a = self.memory[self.cpu.bc() as usize];
            },
            Opcode::DEC_BC => {
            },
            Opcode::INC_C => {
            },
            Opcode::DEC_C => {
            },
            Opcode::LD_C_d8 => {
                self.cpu.c = (instruction & 0xFF) as u8;
            },
            Opcode::RRCA => {
            },
            Opcode::STOP_0 => {
            },
            Opcode::LD_DE_d16 => {
            },
            Opcode::LD_DE_A => {
                self.memory[self.cpu.de() as usize] = self.cpu.a;
            },
            Opcode::INC_DE => {
            },
            Opcode::INC_D => {
            },
            Opcode::DEC_D => {
            },
            Opcode::LD_D_d8 => {
                self.cpu.d = (instruction & 0xFF) as u8;
            },
            Opcode::RLA => {
            },
            Opcode::JR_r8 => {
            },
            Opcode::ADD_HL_DE => {
            },
            Opcode::LD_A_DE => {
                self.cpu.a = self.memory[self.cpu.de() as usize];
            },
            Opcode::DEC_DE => {
            },
            Opcode::INC_E => {
            },
            Opcode::DEC_E => {
            },
            Opcode::LD_E_d8 => {
                self.cpu.e = (instruction & 0xFF) as u8;
            },
            Opcode::RRA => {
            },
            Opcode::JR_NZ_r8 => {
            },
            Opcode::LD_HL_d16 => {
            },
            Opcode::LD_HL_plus_A => {
            },
            Opcode::INC_HL => {
            },
            Opcode::INC_H => {
            },
            Opcode::DEC_H => {
            },
            Opcode::LD_H_d8 => {
                self.cpu.h = (instruction & 0xFF) as u8;
            },
            Opcode::DAA => {
            },
            Opcode::JR_Z_r8 => {
            },
            Opcode::ADD_HL_HL => {
            },
            Opcode::LD_A_HL_ => {
            },
            Opcode::DEC_HL => {
            },
            Opcode::INC_L => {
            },
            Opcode::DEC_L => {
            },
            Opcode::LD_L_d8 => {
                self.cpu.l = (instruction & 0xFF) as u8;
            },
            Opcode::CPL => {
            },
            Opcode::JR_NC_r8 => {
            },
            Opcode::LD_SP_d16 => {
            },
            Opcode::LD_HL_minus_A => {
            },
            Opcode::INC_SP => {
            },
            Opcode::INC_atHL => {
            },
            Opcode::DEC_atHL => {
            },
            Opcode::LD_HL_d8 => {
            },
            Opcode::SCF => {
            },
            Opcode::JR_C_r8 => {
            },
            Opcode::ADD_HL_SP => {
            },
            Opcode::LD_A_HL_minus => {
            },
            Opcode::DEC_SP => {
            },
            Opcode::INC_A => {
            },
            Opcode::DEC_A => {
            },
            Opcode::LD_A_d8 => {
            },
            Opcode::CCF => {
            },
            Opcode::LD_B_B => (),
            Opcode::LD_B_C => {
                self.cpu.b = self.cpu.c;
            },
            Opcode::LD_B_D => {
                self.cpu.b = self.cpu.d;
            },
            Opcode::LD_B_E => {
                self.cpu.b = self.cpu.e;
            },
            Opcode::LD_B_H => {
                self.cpu.b = self.cpu.h;
            },
            Opcode::LD_B_L => {
                self.cpu.b = self.cpu.l;
            },
            Opcode::LD_B_HL => {
                self.cpu.b = self.memory[self.cpu.hl() as usize];
            },
            Opcode::LD_B_A => {
                self.cpu.b = self.cpu.a;
            },
            Opcode::LD_C_B => {
            },
            Opcode::LD_C_C => {
            },
            Opcode::LD_C_D => {
            },
            Opcode::LD_C_E => {
            },
            Opcode::LD_C_H => {
            },
            Opcode::LD_C_L => {
            },
            Opcode::LD_C_HL => {
            },
            Opcode::LD_C_A => {
                self.cpu.c = self.cpu.a;
            },
            Opcode::LD_D_B => {
            },
            Opcode::LD_D_C => {
            },
            Opcode::LD_D_D => {
            },
            Opcode::LD_D_E => {
            },
            Opcode::LD_D_H => {
            },
            Opcode::LD_D_L => {
            },
            Opcode::LD_D_HL => {
            },
            Opcode::LD_D_A => {
                self.cpu.d = self.cpu.a;
            },
            Opcode::LD_E_B => {
            },
            Opcode::LD_E_C => {
            },
            Opcode::LD_E_D => {
            },
            Opcode::LD_E_E => {
            },
            Opcode::LD_E_H => {
            },
            Opcode::LD_E_L => {
            },
            Opcode::LD_E_HL => {
            },
            Opcode::LD_E_A => {
                self.cpu.e = self.cpu.a;
            },
            Opcode::LD_H_B => {
            },
            Opcode::LD_H_C => {
            },
            Opcode::LD_H_D => {
            },
            Opcode::LD_H_E => {
            },
            Opcode::LD_H_H => {
            },
            Opcode::LD_H_L => {
            },
            Opcode::LD_H_HL => {
            },
            Opcode::LD_H_A => {
                self.cpu.h = self.cpu.a;
            },
            Opcode::LD_L_B => {
            },
            Opcode::LD_L_C => {
            },
            Opcode::LD_L_D => {
            },
            Opcode::LD_L_E => {
            },
            Opcode::LD_L_H => {
            },
            Opcode::LD_L_L => {
            },
            Opcode::LD_L_HL => {
            },
            Opcode::LD_L_A => {
                self.cpu.l = self.cpu.a;
            },
            Opcode::LD_HL_B => {
                self.memory[self.cpu.hl() as usize] = self.cpu.b;
            },
            Opcode::LD_HL_C => {
            },
            Opcode::LD_HL_D => {
            },
            Opcode::LD_HL_E => {
            },
            Opcode::LD_HL_H => {
            },
            Opcode::LD_HL_L => {
            },
            Opcode::HALT => {
            },
            Opcode::LD_HL_A => {
                self.memory[self.cpu.hl() as usize] = self.cpu.a;
            },
            Opcode::LD_A_B => {
                self.cpu.a = self.cpu.b;
            },
            Opcode::LD_A_C => {
                self.cpu.a = self.cpu.c;
            },
            Opcode::LD_A_D => {
                self.cpu.a = self.cpu.d;
            },
            Opcode::LD_A_E => {
                self.cpu.a = self.cpu.e;
            },
            Opcode::LD_A_H => {
                self.cpu.a = self.cpu.h;
            },
            Opcode::LD_A_L => {
                self.cpu.a = self.cpu.l;
            },
            Opcode::LD_A_HL => {
                self.cpu.a = self.memory[self.cpu.hl() as usize];
            },
            Opcode::LD_A_A => (),
            Opcode::ADD_A_B => {
            },
            Opcode::ADD_A_C => {
            },
            Opcode::ADD_A_D => {
            },
            Opcode::ADD_A_E => {
            },
            Opcode::ADD_A_H => {
            },
            Opcode::ADD_A_L => {
            },
            Opcode::ADD_A_HL => {
            },
            Opcode::ADD_A_A => {
            },
            Opcode::ADC_A_B => {
            },
            Opcode::ADC_A_C => {
            },
            Opcode::ADC_A_D => {
            },
            Opcode::ADC_A_E => {
            },
            Opcode::ADC_A_H => {
            },
            Opcode::ADC_A_L => {
            },
            Opcode::ADC_A_HL => {
            },
            Opcode::ADC_A_A => {
            },
            Opcode::SUB_B => {
            },
            Opcode::SUB_C => {
            },
            Opcode::SUB_D => {
            },
            Opcode::SUB_E => {
            },
            Opcode::SUB_H => {
            },
            Opcode::SUB_L => {
            },
            Opcode::SUB_HL => {
            },
            Opcode::SUB_A => {
            },
            Opcode::SBC_A_B => {
            },
            Opcode::SBC_A_C => {
            },
            Opcode::SBC_A_D => {
            },
            Opcode::SBC_A_E => {
            },
            Opcode::SBC_A_H => {
            },
            Opcode::SBC_A_L => {
            },
            Opcode::SBC_A_HL => {
            },
            Opcode::SBC_A_A => {
            },
            Opcode::AND_B => {
            },
            Opcode::AND_C => {
            },
            Opcode::AND_D => {
            },
            Opcode::AND_E => {
            },
            Opcode::AND_H => {
            },
            Opcode::AND_L => {
            },
            Opcode::AND_HL => {
            },
            Opcode::AND_A => {
            },
            Opcode::XOR_B => {
            },
            Opcode::XOR_C => {
            },
            Opcode::XOR_D => {
            },
            Opcode::XOR_E => {
            },
            Opcode::XOR_H => {
            },
            Opcode::XOR_L => {
            },
            Opcode::XOR_HL => {
            },
            Opcode::XOR_A => {
            },
            Opcode::OR_B => {
            },
            Opcode::OR_C => {
            },
            Opcode::OR_D => {
            },
            Opcode::OR_E => {
            },
            Opcode::OR_H => {
            },
            Opcode::OR_L => {
            },
            Opcode::OR_HL => {
            },
            Opcode::OR_A => {
            },
            Opcode::CP_B => {
            },
            Opcode::CP_C => {
            },
            Opcode::CP_D => {
            },
            Opcode::CP_E => {
            },
            Opcode::CP_H => {
            },
            Opcode::CP_L => {
            },
            Opcode::CP_HL => {
            },
            Opcode::CP_A => {
            },
            Opcode::RET_NZ => {
            },
            Opcode::POP_BC => {
            },
            Opcode::JP_NZ_a16 => {
            },
            Opcode::JP_a16 => {
                self.cpu.pc = (instruction & 0xFFFF) as u16;
                skip_pc = true;
            },
            Opcode::CALL_NZ_a16 => {
            },
            Opcode::PUSH_BC => {
            },
            Opcode::ADD_A_d8 => {
            },
            Opcode::RST_00H => {
            },
            Opcode::RET_Z => {
            },
            Opcode::RET => {
            },
            Opcode::JP_Z_a16 => {
            },
            Opcode::PREFIX_CB => {
            },
            Opcode::CALL_Z_a16 => {
            },
            Opcode::CALL_a16 => {
            },
            Opcode::ADC_A_d8 => {
            },
            Opcode::RST_08H => {
            },
            Opcode::RET_NC => {
            },
            Opcode::POP_DE => {
            },
            Opcode::JP_NC_a16 => {
            },
            Opcode::CALL_NC_a16 => {
            },
            Opcode::PUSH_DE => {
            },
            Opcode::SUB_d8 => {
            },
            Opcode::RST_10H => {
            },
            Opcode::RET_C => {
            },
            Opcode::RETI => {
            },
            Opcode::JP_C_a16 => {
            },
            Opcode::CALL_C_a16 => {
            },
            Opcode::SBC_A_d8 => {
            },
            Opcode::RST_18H => {
            },
            Opcode::LDH_a8_A => {
            },
            Opcode::POP_HL => {
            },
            Opcode::LD_atC_A => {
            },
            Opcode::PUSH_HL => {
            },
            Opcode::AND_d8 => {
            },
            Opcode::RST_20H => {
            },
            Opcode::ADD_SP_r8 => {
            },
            Opcode::JP_HL => {
            },
            Opcode::LD_a16_A => {
                self.memory[(instruction & 0xFFFF) as usize] = self.cpu.a;
            },
            Opcode::XOR_d8 => {
            },
            Opcode::RST_28H => {
            },
            Opcode::LDH_A_a8 => {
            },
            Opcode::POP_AF => {
            },
            Opcode::LD_A_atC => {
                // TODO: Have to handle memory mapping to different pieces of hardware depending on address.
                //self.cpu.a = self.memory[(0xFF00 + (self.cpu.c as u16)) as usize];
            },
            Opcode::DI => {
            },
            Opcode::PUSH_AF => {
            },
            Opcode::OR_d8 => {
            },
            Opcode::RST_30H => {
            },
            Opcode::LD_HL_SP_plus_r8 => {
            },
            Opcode::LD_SP_HL => {
            },
            Opcode::LD_A_a16 => {
                self.cpu.a = self.memory[(instruction & 0xFFFF) as usize];
            },
            Opcode::EI => {
            },
            Opcode::CP_d8 => {
            },
            Opcode::RST_38H => {
            },
        }
        if !skip_pc {
            self.cpu.pc += size;
        }
        Ok(base_cycles)
    }
}

#[derive(Debug)]
pub enum GameboyError {
    InvalidOpcode { opcode: u8, address: u16 },
    NoCartridge,
}

impl GameboyError {
    pub fn recoverable(&self) -> bool {
        match self {
            Self::InvalidOpcode { .. } => false,
            Self::NoCartridge => true,
        }
    }
}

impl fmt::Display for GameboyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", match self {
            Self::InvalidOpcode { opcode, address } => format!("Invalid opcode ({}) at address: {}", opcode, address),
            Self::NoCartridge => "No cartridge found!".to_owned(),
        })
    }
}

// TODO: Some of these tests can be made better by explicitly setting register / memory values beforehand instead of inferring.
mod tests {
    use super::{Cartridge, Gameboy};

    #[allow(dead_code)] // This isn't dead code but go off.
    fn create_test_gameboy(mock_rom: Option<Vec<u8>>) -> Gameboy {
        let mut rom = vec![0; 0xFFFF];
        if mock_rom.is_some() {
            for (idx, byte) in mock_rom.unwrap().iter().enumerate() {
                rom[0x100 + idx] = *byte;
            }
        }
        Gameboy::new(Cartridge { rom })
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_NOP() {
        let mut gameboy = create_test_gameboy(None);
        let cycles = gameboy.tick().unwrap();
        assert_eq!(4, cycles);
        assert_eq!(0x101, gameboy.cpu.pc);
    }

    // TODO: Can we iterate?
    #[test]
    #[allow(non_snake_case)]
    fn test_8bit_LD_d8() {
        let mut gameboy = create_test_gameboy(Some(vec![0x06, 0xFF, 0x0E, 0xFF, 0x16, 0xFF, 0x1E, 0xFF, 0x26, 0xFF, 0x2E, 0xFF]));
        assert_eq!(8, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.b);
        assert_eq!(8, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.c);
        assert_eq!(8, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.d);
        assert_eq!(8, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.e);
        assert_eq!(8, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.h);
        assert_eq!(8, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.l);
    }

    // TODO: Can we iterate?
    #[test]
    #[allow(non_snake_case)]
    fn test_8bit_LD_r1r2() {
        let mut gameboy = create_test_gameboy(Some(vec![
            0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E,
            0x41, 0x42, 0x43, 0x44, 0x45, 0x46,
        ]));
        gameboy.cpu.a = 0;
        gameboy.cpu.b = 0xFF;
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.a);
        gameboy.cpu.a = 0;
        gameboy.cpu.c = 0xFF;
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.a);
        gameboy.cpu.a = 0;
        gameboy.cpu.d = 0xFF;
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.a);
        gameboy.cpu.a = 0;
        gameboy.cpu.e = 0xFF;
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.a);
        gameboy.cpu.a = 0;
        gameboy.cpu.h = 0xFF;
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.a);
        gameboy.cpu.a = 0;
        gameboy.cpu.l = 0xFF;
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.a);
        gameboy.cpu.a = 0;
        gameboy.cpu.h = 0x13;
        gameboy.cpu.l = 0x37;
        gameboy.memory[0x1337] = 0xFF;
        assert_eq!(8, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.a);

        gameboy.cpu.b = 0;
        gameboy.cpu.c = 0xFF;
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.b);
        gameboy.cpu.b = 0;
        gameboy.cpu.d = 0xFF;
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.b);
        gameboy.cpu.b = 0;
        gameboy.cpu.e= 0xFF;
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.b);
        gameboy.cpu.b = 0;
        gameboy.cpu.h = 0xFF;
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.b);
        gameboy.cpu.b = 0;
        gameboy.cpu.l = 0xFF;
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.b);
        gameboy.cpu.b = 0;
        gameboy.cpu.h = 0x13;
        gameboy.cpu.l = 0x37;
        gameboy.memory[0x1337] = 0xFF;
        assert_eq!(8, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.b);

        // TODO: Other tests here.
    }

    // TODO: Can we iterate?
    #[test]
    #[allow(non_snake_case)]
    fn test_8bit_LD_A_n() {
        let mut gameboy = create_test_gameboy(Some(vec![0x0A, 0x1A, 0xFA, 0x10, 0x25])); // TODO: A,#?
        gameboy.cpu.set_bc(0x1337);
        gameboy.memory[0x1337] = 0xFF;
        assert_eq!(8, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.a);

        gameboy.cpu.a = 0;
        gameboy.cpu.set_de(0x505);
        gameboy.memory[0x505] = 0xAC;
        assert_eq!(8, gameboy.tick().unwrap());
        assert_eq!(0xAC, gameboy.cpu.a);

        gameboy.cpu.a = 0;
        gameboy.memory[0x1025] = 0xAB;
        assert_eq!(16, gameboy.tick().unwrap());
        assert_eq!(0xAB, gameboy.cpu.a);
    }

    // TODO: Can we iterate?
    #[test]
    #[allow(non_snake_case)]
    fn test_8bit_LD_n_A() {
        let mut gameboy = create_test_gameboy(Some(vec![0x47, 0x4F, 0x57, 0x5F, 0x67, 0x6F, 0x02, 0x12, 0x77, 0xEA, 0x13, 0x37]));
        gameboy.cpu.a = 0xFF;
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.b);
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.c);
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.d);
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.e);
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.h);
        assert_eq!(4, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.cpu.l);

        gameboy.cpu.set_bc(0x1000);
        assert_eq!(8, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.memory[0x1000]);
        gameboy.cpu.set_de(0x1010);
        assert_eq!(8, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.memory[0x1010]);
        gameboy.cpu.set_hl(0x1020);
        assert_eq!(8, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.memory[0x1020]);

        assert_eq!(16, gameboy.tick().unwrap());
        assert_eq!(0xFF, gameboy.memory[0x1337]);
    }
    
    #[test]
    #[allow(non_snake_case)]
    fn test_LD_A_atC() {
        let mut gameboy = create_test_gameboy(Some(vec![0xF2, 0x00])); // TODO: Is the second byte junk?
        gameboy.cpu.a = 0;
        gameboy.cpu.c = 0x0A;
        gameboy.memory[0x50] = 0x3C;
        assert_eq!(8, gameboy.tick().unwrap());
        assert_eq!(0x3C, gameboy.cpu.a);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LD_BC_d16() {
        let mut gameboy = create_test_gameboy(Some(vec![0x01, 0x13, 0x37]));
        let cycles = gameboy.tick().unwrap();
        assert_eq!(12, cycles);
        assert_eq!(0x1337, gameboy.cpu.bc());
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_RLCA() {
        // TODO: Need to figure out what this operation actually does first.
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LD_HL_B() {
        let mut gameboy = create_test_gameboy(Some(vec![0x70]));
        gameboy.cpu.h = 0x13;
        gameboy.cpu.l = 0x37;
        gameboy.cpu.b = 0xFF;
        let cycles = gameboy.tick().unwrap();
        assert_eq!(8, cycles);
        assert_eq!(0xFF, gameboy.memory[0x1337]);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_JP_a16() {
        let mut gameboy = create_test_gameboy(Some(vec![0xC3, 0x13, 0x37]));
        let cycles = gameboy.tick().unwrap();
        assert_eq!(16, cycles);
        assert_eq!(0x1337, gameboy.cpu.pc);
    }
}