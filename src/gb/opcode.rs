const OPCODE_MAP: [[Option<OpcodeType>; 16]; 16] = [
	[Some(OpcodeType::NOP),      Some(OpcodeType::LD_BC_d16), Some(OpcodeType::LD_BC_A),       Some(OpcodeType::INC_BC),  Some(OpcodeType::INC_B),         Some(OpcodeType::DEC_B),    Some(OpcodeType::LD_B_d8),  Some(OpcodeType::RLCA),             Some(OpcodeType::LD_a16_SP),        Some(OpcodeType::ADD_HL_BC), Some(OpcodeType::LD_A_BC),       Some(OpcodeType::DEC_BC),    Some(OpcodeType::INC_C),      Some(OpcodeType::DEC_C),    Some(OpcodeType::LD_C_d8),  Some(OpcodeType::RRCA)    ],
	[Some(OpcodeType::STOP_0),   Some(OpcodeType::LD_DE_d16), Some(OpcodeType::LD_DE_A),       Some(OpcodeType::INC_DE),  Some(OpcodeType::INC_D),         Some(OpcodeType::DEC_D),    Some(OpcodeType::LD_D_d8),  Some(OpcodeType::RLA),              Some(OpcodeType::JR_r8),            Some(OpcodeType::ADD_HL_DE), Some(OpcodeType::LD_A_DE),       Some(OpcodeType::DEC_DE),    Some(OpcodeType::INC_E),      Some(OpcodeType::DEC_E),    Some(OpcodeType::LD_E_d8),  Some(OpcodeType::RRA)     ],
	[Some(OpcodeType::JR_NZ_r8), Some(OpcodeType::LD_HL_d16), Some(OpcodeType::LD_HL_plus_A),  Some(OpcodeType::INC_HL),  Some(OpcodeType::INC_H),         Some(OpcodeType::DEC_H),    Some(OpcodeType::LD_H_d8),  Some(OpcodeType::DAA),              Some(OpcodeType::JR_Z_r8),          Some(OpcodeType::ADD_HL_HL), Some(OpcodeType::LD_A_HL_),      Some(OpcodeType::DEC_HL),    Some(OpcodeType::INC_L),      Some(OpcodeType::DEC_L),    Some(OpcodeType::LD_L_d8),  Some(OpcodeType::CPL)     ],
	[Some(OpcodeType::JR_NC_r8), Some(OpcodeType::LD_SP_d16), Some(OpcodeType::LD_HL_minus_A), Some(OpcodeType::INC_SP),  Some(OpcodeType::INC_atHL),      Some(OpcodeType::DEC_atHL), Some(OpcodeType::LD_HL_d8), Some(OpcodeType::SCF),              Some(OpcodeType::JR_C_r8),          Some(OpcodeType::ADD_HL_SP), Some(OpcodeType::LD_A_HL_minus), Some(OpcodeType::DEC_SP),    Some(OpcodeType::INC_A),      Some(OpcodeType::DEC_A),    Some(OpcodeType::LD_A_d8),  Some(OpcodeType::CCF)     ],
	[Some(OpcodeType::LD_B_B),   Some(OpcodeType::LD_B_C),    Some(OpcodeType::LD_B_D),        Some(OpcodeType::LD_B_E),  Some(OpcodeType::LD_B_H),        Some(OpcodeType::LD_B_L),   Some(OpcodeType::LD_B_HL),  Some(OpcodeType::LD_B_A),           Some(OpcodeType::LD_C_B),           Some(OpcodeType::LD_C_C),    Some(OpcodeType::LD_C_D),        Some(OpcodeType::LD_C_E),    Some(OpcodeType::LD_C_H),     Some(OpcodeType::LD_C_L),   Some(OpcodeType::LD_C_HL),  Some(OpcodeType::LD_C_A)  ],
	[Some(OpcodeType::LD_D_B),   Some(OpcodeType::LD_D_C),    Some(OpcodeType::LD_D_D),        Some(OpcodeType::LD_D_E),  Some(OpcodeType::LD_D_H),        Some(OpcodeType::LD_D_L),   Some(OpcodeType::LD_D_HL),  Some(OpcodeType::LD_D_A),           Some(OpcodeType::LD_E_B),           Some(OpcodeType::LD_E_C),    Some(OpcodeType::LD_E_D),        Some(OpcodeType::LD_E_E),    Some(OpcodeType::LD_E_H),     Some(OpcodeType::LD_E_L),   Some(OpcodeType::LD_E_HL),  Some(OpcodeType::LD_E_A)  ],
	[Some(OpcodeType::LD_H_B),   Some(OpcodeType::LD_H_C),    Some(OpcodeType::LD_H_D),        Some(OpcodeType::LD_H_E),  Some(OpcodeType::LD_H_H),        Some(OpcodeType::LD_H_L),   Some(OpcodeType::LD_H_HL),  Some(OpcodeType::LD_H_A),           Some(OpcodeType::LD_L_B),           Some(OpcodeType::LD_L_C),    Some(OpcodeType::LD_L_D),        Some(OpcodeType::LD_L_E),    Some(OpcodeType::LD_L_H),     Some(OpcodeType::LD_L_L),   Some(OpcodeType::LD_L_HL),  Some(OpcodeType::LD_L_A)  ],
	[Some(OpcodeType::LD_HL_B),  Some(OpcodeType::LD_HL_C),   Some(OpcodeType::LD_HL_D),       Some(OpcodeType::LD_HL_E), Some(OpcodeType::LD_HL_H),       Some(OpcodeType::LD_HL_L),  Some(OpcodeType::HALT),     Some(OpcodeType::LD_HL_A),          Some(OpcodeType::LD_A_B),           Some(OpcodeType::LD_A_C),    Some(OpcodeType::LD_A_D),        Some(OpcodeType::LD_A_E),    Some(OpcodeType::LD_A_H),     Some(OpcodeType::LD_A_L),   Some(OpcodeType::LD_A_HL),  Some(OpcodeType::LD_A_A)  ],
	[Some(OpcodeType::ADD_A_B),  Some(OpcodeType::ADD_A_C),   Some(OpcodeType::ADD_A_D),       Some(OpcodeType::ADD_A_E), Some(OpcodeType::ADD_A_H),       Some(OpcodeType::ADD_A_L),  Some(OpcodeType::ADD_A_HL), Some(OpcodeType::ADD_A_A),          Some(OpcodeType::ADC_A_B),          Some(OpcodeType::ADC_A_C),   Some(OpcodeType::ADC_A_D),       Some(OpcodeType::ADC_A_E),   Some(OpcodeType::ADC_A_H),    Some(OpcodeType::ADC_A_L),  Some(OpcodeType::ADC_A_HL), Some(OpcodeType::ADC_A_A) ],
	[Some(OpcodeType::SUB_B),    Some(OpcodeType::SUB_C),     Some(OpcodeType::SUB_D),         Some(OpcodeType::SUB_E),   Some(OpcodeType::SUB_H),         Some(OpcodeType::SUB_L),    Some(OpcodeType::SUB_HL),   Some(OpcodeType::SUB_A),            Some(OpcodeType::SBC_A_B),          Some(OpcodeType::SBC_A_C),   Some(OpcodeType::SBC_A_D),       Some(OpcodeType::SBC_A_E),   Some(OpcodeType::SBC_A_H),    Some(OpcodeType::SBC_A_L),  Some(OpcodeType::SBC_A_HL), Some(OpcodeType::SBC_A_A) ],
	[Some(OpcodeType::AND_B),    Some(OpcodeType::AND_C),     Some(OpcodeType::AND_D),         Some(OpcodeType::AND_E),   Some(OpcodeType::AND_H),         Some(OpcodeType::AND_L),    Some(OpcodeType::AND_HL),   Some(OpcodeType::AND_A),            Some(OpcodeType::XOR_B),            Some(OpcodeType::XOR_C),     Some(OpcodeType::XOR_D),         Some(OpcodeType::XOR_E),     Some(OpcodeType::XOR_H),      Some(OpcodeType::XOR_L),    Some(OpcodeType::XOR_HL),   Some(OpcodeType::XOR_A)   ],
	[Some(OpcodeType::OR_B),     Some(OpcodeType::OR_C),      Some(OpcodeType::OR_D),          Some(OpcodeType::OR_E),    Some(OpcodeType::OR_H),          Some(OpcodeType::OR_L),     Some(OpcodeType::OR_HL),    Some(OpcodeType::OR_A),             Some(OpcodeType::CP_B),             Some(OpcodeType::CP_C),      Some(OpcodeType::CP_D),          Some(OpcodeType::CP_E),      Some(OpcodeType::CP_H),       Some(OpcodeType::CP_L),     Some(OpcodeType::CP_HL),    Some(OpcodeType::CP_A)    ],
	[Some(OpcodeType::RET_NZ),   Some(OpcodeType::POP_BC),    Some(OpcodeType::JP_NZ_a16),     Some(OpcodeType::JP_a16),  Some(OpcodeType::CALL_NZ_a16),   Some(OpcodeType::PUSH_BC),  Some(OpcodeType::ADD_A_d8), Some(OpcodeType::RST_00H),          Some(OpcodeType::RET_Z),            Some(OpcodeType::RET),       Some(OpcodeType::JP_Z_a16),      Some(OpcodeType::PREFIX_CB), Some(OpcodeType::CALL_Z_a16), Some(OpcodeType::CALL_a16), Some(OpcodeType::ADC_A_d8), Some(OpcodeType::RST_08H) ],
	[Some(OpcodeType::RET_NC),   Some(OpcodeType::POP_DE),    Some(OpcodeType::JP_NC_a16),     None,                	  Some(OpcodeType::CALL_NC_a16),   Some(OpcodeType::PUSH_DE),  Some(OpcodeType::SUB_d8),   Some(OpcodeType::RST_10H),          Some(OpcodeType::RET_C),            Some(OpcodeType::RETI),      Some(OpcodeType::JP_C_a16),      None,                  	  Some(OpcodeType::CALL_C_a16), None,                 		Some(OpcodeType::SBC_A_d8), Some(OpcodeType::RST_18H) ],
	[Some(OpcodeType::LDH_a8_A), Some(OpcodeType::POP_HL),    Some(OpcodeType::LD_atC_A),      None,                	  None,                      	   Some(OpcodeType::PUSH_HL),  Some(OpcodeType::AND_d8),   Some(OpcodeType::RST_20H),          Some(OpcodeType::ADD_SP_r8),        Some(OpcodeType::JP_HL),     Some(OpcodeType::LD_a16_A),      None,                  	  None,                   		None,                 		Some(OpcodeType::XOR_d8),   Some(OpcodeType::RST_28H) ],
	[Some(OpcodeType::LDH_A_a8), Some(OpcodeType::POP_AF),    Some(OpcodeType::LD_A_atC),      Some(OpcodeType::DI),      None,                      	   Some(OpcodeType::PUSH_AF),  Some(OpcodeType::OR_d8),    Some(OpcodeType::RST_30H),          Some(OpcodeType::LD_HL_SP_plus_r8), Some(OpcodeType::LD_SP_HL),  Some(OpcodeType::LD_A_a16),      Some(OpcodeType::EI),        None,                   		None,                 		Some(OpcodeType::CP_d8),    Some(OpcodeType::RST_38H) ],
];

// The size (in bytes) that instructions with the given opcode will be.
const OPCODE_SIZES: [[u8; 16]; 16] = [
	[1, 3, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 2, 1],
	[2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1],
	[2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1],
	[2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1],
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
	[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
	[1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3, 1, 3, 3, 2, 1],
	[1, 1, 3, 0, 3, 1, 2, 1, 1, 1, 3, 0, 3, 0, 2, 1],
	[2, 1, 2, 0, 0, 1, 2, 1, 2, 1, 3, 0, 0, 0, 2, 1],
	[2, 1, 2, 1, 0, 1, 2, 1, 2, 1, 3, 1, 0, 0, 2, 1],
];

// Some instructions will require more than the base number of clock cycles.
// This map represents the *minimum* number of cycles required for instructions.
const OPCODE_BASE_CYCLES: [[u8; 16]; 16] = [
	[4,  12, 8,  8,  4,  4,  8,  4,  20, 8,  8,  8, 4,  4,  8, 4],
	[4,  12, 8,  8,  4,  4,  8,  4,  12, 8,  8,  8, 4,  4,  8, 4],
	[8,  12, 8,  8,  4,  4,  8,  4,  8,  8,  8,  8, 4,  4,  8, 4],
	[8,  12, 8,  8,  12, 12, 12, 4,  8,  8,  8,  8, 4,  4,  8, 4],
	[4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4, 4,  4,  8, 4],
	[4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4, 4,  4,  8, 4],
	[4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4, 4,  4,  8, 4],
	[8,  8,  8,  8,  8,  8,  4,  8,  4,  4,  4,  4, 4,  4,  8, 4],
	[4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4, 4,  4,  8, 4],
	[4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4, 4,  4,  8, 4],
	[4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4, 4,  4,  8, 4],
	[4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4, 4,  4,  8, 4],
	[8,  12, 12, 16, 12, 16, 8,  16, 8,  16, 12, 4, 12, 24, 8, 16],
	[8,  12, 12, 0,  12, 16, 8,  16, 8,  16, 12, 0, 12, 0,  8, 16],
	[12, 12, 8,  0,  0,  16, 8,  16, 16, 4,  16, 0, 0,  0,  8, 16],
	[12, 12, 8,  4,  0,  16, 8,  16, 12, 8,  16, 4, 0,  0,  8, 16],
];

pub struct Opcode {
	pub data: u8,
	pub type_: OpcodeType,
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum OpcodeType {
	NOP,
	LD_BC_d16,
	LD_BC_A,
	INC_BC,
	INC_B,
	DEC_B,
	LD_B_d8,
	RLCA,
	LD_a16_SP,
	ADD_HL_BC,
	LD_A_BC,
	DEC_BC,
	INC_C,
	DEC_C,
	LD_C_d8,
	RRCA,
	STOP_0,
	LD_DE_d16,
	LD_DE_A,
	INC_DE,
	INC_D,
	DEC_D,
	LD_D_d8,
	RLA,
	JR_r8,
	ADD_HL_DE,
	LD_A_DE,
	DEC_DE,
	INC_E,
	DEC_E,
	LD_E_d8,
	RRA,
	JR_NZ_r8,
	LD_HL_d16,
	LD_HL_plus_A,
	INC_HL,
	INC_H,
	DEC_H,
	LD_H_d8,
	DAA,
	JR_Z_r8,
	ADD_HL_HL,
	LD_A_HL_,
	DEC_HL,
	INC_L,
	DEC_L,
	LD_L_d8,
	CPL,
	JR_NC_r8,
	LD_SP_d16,
	LD_HL_minus_A,
	INC_SP,
	INC_atHL,
	DEC_atHL,
	LD_HL_d8,
	SCF,
	JR_C_r8,
	ADD_HL_SP,
	LD_A_HL_minus,
	DEC_SP,
	INC_A,
	DEC_A,
	LD_A_d8,
	CCF,
	LD_B_B,
	LD_B_C,
	LD_B_D,
	LD_B_E,
	LD_B_H,
	LD_B_L,
	LD_B_HL,
	LD_B_A,
	LD_C_B,
	LD_C_C,
	LD_C_D,
	LD_C_E,
	LD_C_H,
	LD_C_L,
	LD_C_HL,
	LD_C_A,
	LD_D_B,
	LD_D_C,
	LD_D_D,
	LD_D_E,
	LD_D_H,
	LD_D_L,
	LD_D_HL,
	LD_D_A,
	LD_E_B,
	LD_E_C,
	LD_E_D,
	LD_E_E,
	LD_E_H,
	LD_E_L,
	LD_E_HL,
	LD_E_A,
	LD_H_B,
	LD_H_C,
	LD_H_D,
	LD_H_E,
	LD_H_H,
	LD_H_L,
	LD_H_HL,
	LD_H_A,
	LD_L_B,
	LD_L_C,
	LD_L_D,
	LD_L_E,
	LD_L_H,
	LD_L_L,
	LD_L_HL,
	LD_L_A,
	LD_HL_B,
	LD_HL_C,
	LD_HL_D,
	LD_HL_E,
	LD_HL_H,
	LD_HL_L,
	HALT,
	LD_HL_A,
	LD_A_B,
	LD_A_C,
	LD_A_D,
	LD_A_E,
	LD_A_H,
	LD_A_L,
	LD_A_HL,
	LD_A_A,
	ADD_A_B,
	ADD_A_C,
	ADD_A_D,
	ADD_A_E,
	ADD_A_H,
	ADD_A_L,
	ADD_A_HL,
	ADD_A_A,
	ADC_A_B,
	ADC_A_C,
	ADC_A_D,
	ADC_A_E,
	ADC_A_H,
	ADC_A_L,
	ADC_A_HL,
	ADC_A_A,
	SUB_B,
	SUB_C,
	SUB_D,
	SUB_E,
	SUB_H,
	SUB_L,
	SUB_HL,
	SUB_A,
	SBC_A_B,
	SBC_A_C,
	SBC_A_D,
	SBC_A_E,
	SBC_A_H,
	SBC_A_L,
	SBC_A_HL,
	SBC_A_A,
	AND_B,
	AND_C,
	AND_D,
	AND_E,
	AND_H,
	AND_L,
	AND_HL,
	AND_A,
	XOR_B,
	XOR_C,
	XOR_D,
	XOR_E,
	XOR_H,
	XOR_L,
	XOR_HL,
	XOR_A,
	OR_B,
	OR_C,
	OR_D,
	OR_E,
	OR_H,
	OR_L,
	OR_HL,
	OR_A,
	CP_B,
	CP_C,
	CP_D,
	CP_E,
	CP_H,
	CP_L,
	CP_HL,
	CP_A,
	RET_NZ,
	POP_BC,
	JP_NZ_a16,
	JP_a16,
	CALL_NZ_a16,
	PUSH_BC,
	ADD_A_d8,
	RST_00H,
	RET_Z,
	RET,
	JP_Z_a16,
	PREFIX_CB,
	CALL_Z_a16,
	CALL_a16,
	ADC_A_d8,
	RST_08H,
	RET_NC,
	POP_DE,
	JP_NC_a16,
	CALL_NC_a16,
	PUSH_DE,
	SUB_d8,
	RST_10H,
	RET_C,
	RETI,
	JP_C_a16,
	CALL_C_a16,
	SBC_A_d8,
	RST_18H,
	LDH_a8_A,
	POP_HL,
	LD_atC_A,
	PUSH_HL,
	AND_d8,
	RST_20H,
	ADD_SP_r8,
	JP_HL,
	LD_a16_A,
	XOR_d8,
	RST_28H,
	LDH_A_a8,
	POP_AF,
	LD_A_atC,
	DI,
	PUSH_AF,
	OR_d8,
	RST_30H,
	LD_HL_SP_plus_r8,
	LD_SP_HL,
	LD_A_a16,
	EI,
	CP_d8,
	RST_38H,
}

impl Opcode {
    pub fn parse(data: u8) -> Option<Self> {
        let (row, col) = Self::get_indices(data);
		match &OPCODE_MAP[row][col] {
			Some(type_) => Some(Self { data, type_: type_.clone() }),
			None => None,
		}
    }

    pub fn size(&self) -> u8 {
        let (row, col) = Self::get_indices(self.data);
        OPCODE_SIZES[row][col]
    }

    pub fn base_cycles(&self) -> u8 {
        let (row, col) = Self::get_indices(self.data);
        OPCODE_BASE_CYCLES[row][col]
    }

    fn get_indices(opcode: u8) -> (usize, usize) {
        ((opcode >> 4) as usize, (opcode & 0xF) as usize)
    }
}
