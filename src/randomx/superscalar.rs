extern crate blake2b_simd;

use self::blake2b_simd::{blake2b, Hash, Params};
use strum::Display;
use std::convert::TryInto;

const RANDOMX_SUPERSCALAR_LATENCY : usize = 17;
const SUPERSCALAR_MAX_SIZE : usize = 3 * RANDOMX_SUPERSCALAR_LATENCY + 2;

#[allow(nonstandard_style)]
#[derive(Display, Debug, PartialEq)]
pub enum ScOpcode {
    INVALID = -1,
    ISUB_R = 0,
	IXOR_R = 1,
	IADD_RS = 2,
	IMUL_R = 3,
	IROR_C = 4,
	IADD_C7 = 5,
	IXOR_C7 = 6,
	IADD_C8 = 7,
	IXOR_C8 = 8,
	IADD_C9 = 9,
	IXOR_C9 = 10,
	IMULH_R = 11,
	ISMULH_R = 12,
	IMUL_RCP = 13,
	COUNT = 14,
}

pub struct ScInstr<'a> {
	pub info: &'a ScInstrInfo,
	pub mod_v: u8,
	pub imm32: u32,
	pub op_group: ScOpcode,
	pub op_group_par: i32,
	pub group_par_is_source: bool,
}

static SLOT_3L : [&ScInstrInfo; 4] = [&ISUB_R, &IXOR_R, &IMULH_R, &ISMULH_R];
static SLOT_4 : [&ScInstrInfo; 2] = [&IROR_C, &IADD_RS];
static SLOT_7 : [&ScInstrInfo; 2] = [&IXOR_C7, &IADD_C7]; 
static SLOT_8 : [&ScInstrInfo; 2] = [&IXOR_C8, &IADD_C8];  
static SLOT_9 : [&ScInstrInfo; 2] = [&IXOR_C9, &IADD_C9]; 
static SLOT_10: &ScInstrInfo = &IMUL_RCP; 

fn is_zero_or_power_of_2(v: u32) -> bool {
	v & v.wrapping_sub(1) == 0
}

impl ScInstr<'_> {
	pub fn create_for_slot<'a>(gen: &mut Blake2Generator, slot_size: u32, fetch_type: u32, is_last: bool, is_first: bool) -> ScInstr<'a> {
        match slot_size {
			3 => {
				if is_last {
					ScInstr::create(&SLOT_3L[(gen.get_byte() & 3) as usize], gen)
				} else {
					ScInstr::create(&SLOT_3L[(gen.get_byte() & 1) as usize], gen)
				}
			},
			4 => {
				if fetch_type == 4 && !is_last {
					ScInstr::create(&IMUL_R, gen)
				} else {
					ScInstr::create(&SLOT_4[(gen.get_byte() & 1) as usize], gen)
				}
			},
			7 => {
				ScInstr::create(&SLOT_7[(gen.get_byte() & 1) as usize], gen)
			},
			8 => {
				ScInstr::create(&SLOT_8[(gen.get_byte() & 1) as usize], gen)
			},
			9 => {
				ScInstr::create(&SLOT_9[(gen.get_byte() & 1) as usize], gen)
			},
			10 => {
				ScInstr::create(&SLOT_10, gen)
			}
			_ => panic!("illegal slot_size {}", slot_size)
		}
	}

    fn create<'a>(info: &'static ScInstrInfo, gen: &mut Blake2Generator) -> ScInstr<'a> {
		match info.op {
			ScOpcode::ISUB_R => ScInstr{info, mod_v: 0, imm32: 0, op_group: ScOpcode::IADD_RS, group_par_is_source: true, op_group_par: 0 },
			ScOpcode::IXOR_R => ScInstr{info, mod_v: 0, imm32: 0, op_group: ScOpcode::IXOR_R, group_par_is_source: true, op_group_par: 0 },
			ScOpcode::IADD_RS => ScInstr{info, mod_v: gen.get_byte(), imm32: 0, op_group: ScOpcode::IADD_RS, group_par_is_source: true, op_group_par: 0 },
			ScOpcode::IMUL_R => ScInstr{info, mod_v: 0, imm32: 0, op_group: ScOpcode::IMUL_R, group_par_is_source: true, op_group_par: 0 },
			ScOpcode::IROR_C => {
				let mut imm32;
				while {
					imm32 = gen.get_byte() & 63;
					imm32 == 0
				}{};
				ScInstr{info, mod_v: 0, imm32: imm32 as u32, op_group: ScOpcode::IROR_C, group_par_is_source: true, op_group_par: 0 }
			},
			ScOpcode::IADD_C7 | ScOpcode::IADD_C8 | ScOpcode::IADD_C9 => ScInstr{info, mod_v: 0, imm32: gen.get_u32(), op_group: ScOpcode::IADD_C7, group_par_is_source: false, op_group_par: -1},
			ScOpcode::IXOR_C7 | ScOpcode::IXOR_C8 | ScOpcode::IXOR_C9 => ScInstr{info, mod_v: 0, imm32: gen.get_u32(), op_group: ScOpcode::IXOR_C7, group_par_is_source: false, op_group_par: -1},
			ScOpcode::IMULH_R => ScInstr{info, mod_v: 0, imm32: 0, op_group: ScOpcode::IMULH_R, group_par_is_source: true, op_group_par: gen.get_u32() as i32 },
			ScOpcode::ISMULH_R => ScInstr{info, mod_v: 0, imm32: 0, op_group: ScOpcode::ISMULH_R, group_par_is_source: true, op_group_par: gen.get_u32() as i32 },
			ScOpcode::IMUL_RCP => {
				let mut imm32;
				while {
					imm32 = gen.get_u32();
					is_zero_or_power_of_2(imm32)
				}{};
				ScInstr{info, mod_v: 0, imm32: 0, op_group: ScOpcode::IMUL_RCP, group_par_is_source: true, op_group_par: -1 }
			},
			ScOpcode::INVALID | ScOpcode::COUNT => panic!("invalid opcode {} here", info.op)
		}
	}
}

#[repr(u8)]
pub enum ExecutionPort {
	NULL = 0,
	P0 = 1,
	P1 = 2,
	P5 = 4,
	P01 = ExecutionPort::P0 as u8 | ExecutionPort::P1 as u8,
	P05 = ExecutionPort::P0 as u8 | ExecutionPort::P5 as u8,
	P015 =  ExecutionPort::P0 as u8 | ExecutionPort::P1 as u8 | ExecutionPort::P5 as u8
}

pub struct ScMacroOp {
	size: u32,
	latency: u32,
	uop1: ExecutionPort,
	uop2: ExecutionPort,
	dependent: bool,
}

impl ScMacroOp {
	pub const fn new(size: u32, latency: u32, uop1: ExecutionPort, uop2: ExecutionPort) -> ScMacroOp {
		ScMacroOp{size, latency, uop1, uop2, dependent: false}
	}
	pub const fn new_dep(size: u32, latency: u32, uop1: ExecutionPort, uop2: ExecutionPort) -> ScMacroOp {
		ScMacroOp{size, latency, uop1, uop2, dependent: true}
	}
}

static MOP_SUB_RR : ScMacroOp = ScMacroOp::new(3, 1, ExecutionPort::P015, ExecutionPort::NULL); 
static MOP_XOR_RR : ScMacroOp = ScMacroOp::new(3, 1, ExecutionPort::P015, ExecutionPort::NULL); 
static MOP_IMUL_R : ScMacroOp = ScMacroOp::new(3, 4, ExecutionPort::P1, ExecutionPort::P5); 
static MOP_MUL_R : ScMacroOp = ScMacroOp::new(3, 4, ExecutionPort::P1, ExecutionPort::P5); 
static MOP_MOV_RR : ScMacroOp = ScMacroOp::new(3, 1, ExecutionPort::NULL, ExecutionPort::NULL); 

static MOP_LEA_SIB : ScMacroOp = ScMacroOp::new(4, 1, ExecutionPort::P01, ExecutionPort::NULL);   
static MOP_IMUL_RR_DEP : ScMacroOp = ScMacroOp::new_dep(4, 3, ExecutionPort::P1, ExecutionPort::NULL);   
static MOP_ROR_RI : ScMacroOp = ScMacroOp::new(4, 1, ExecutionPort::P05, ExecutionPort::NULL);  

static MOP_ADD_RI : ScMacroOp = ScMacroOp::new(7, 1, ExecutionPort::P015, ExecutionPort::NULL);   
static MOP_XOR_RI : ScMacroOp = ScMacroOp::new(7, 1, ExecutionPort::P015, ExecutionPort::NULL);  

static MOP_MOV_RI64 : ScMacroOp = ScMacroOp::new(10, 1, ExecutionPort::P015, ExecutionPort::NULL);   

static MOP_IMUL_RR : ScMacroOp = ScMacroOp::new(4, 3, ExecutionPort::P1, ExecutionPort::NULL); 

#[allow(nonstandard_style)]
pub struct ScInstrInfo {
	op: ScOpcode,
	macro_ops : &'static [&'static ScMacroOp],
	scr_op: i32
}

impl ScInstrInfo {

	pub const fn new(op: ScOpcode, macro_ops: &'static [&ScMacroOp], scr_op: i32, result_op: u32, dst_op: u32) -> ScInstrInfo {		
		ScInstrInfo{
			op,
			macro_ops,
			scr_op
		}
	}

	pub fn size(&self) -> usize {
		self.macro_ops.len()
	}
}

static NOP : ScInstrInfo = ScInstrInfo::new(ScOpcode::INVALID, &[], 0, 0, 0);

static ISUB_R : ScInstrInfo = ScInstrInfo::new(ScOpcode::ISUB_R, &[&MOP_SUB_RR], 0, 0, 0);
static IXOR_R : ScInstrInfo = ScInstrInfo::new(ScOpcode::IXOR_R, &[&MOP_XOR_RR], 0, 0, 0);
static IADD_RS : ScInstrInfo = ScInstrInfo::new(ScOpcode::IADD_RS, &[&MOP_LEA_SIB], 0, 0, 0);
static IMUL_R : ScInstrInfo = ScInstrInfo::new(ScOpcode::IMUL_R, &[&MOP_IMUL_RR], 0, 0, 0); 
static IROR_C : ScInstrInfo = ScInstrInfo::new(ScOpcode::IROR_C, &[&MOP_ROR_RI], -1, 0, 0); 

static IADD_C7 : ScInstrInfo = ScInstrInfo::new(ScOpcode::IADD_C7, &[&MOP_ADD_RI], -1, 0, 0);  
static IXOR_C7 : ScInstrInfo = ScInstrInfo::new(ScOpcode::IXOR_C7, &[&MOP_XOR_RI], -1, 0, 0);  
static IADD_C8 : ScInstrInfo = ScInstrInfo::new(ScOpcode::IADD_C8, &[&MOP_ADD_RI], -1, 0, 0);  
static IXOR_C8 : ScInstrInfo = ScInstrInfo::new(ScOpcode::IXOR_C8, &[&MOP_XOR_RI], -1, 0, 0);  
static IADD_C9 : ScInstrInfo = ScInstrInfo::new(ScOpcode::IADD_C9, &[&MOP_ADD_RI], -1, 0, 0);  
static IXOR_C9 : ScInstrInfo = ScInstrInfo::new(ScOpcode::IXOR_C9, &[&MOP_XOR_RI], -1, 0, 0);  

static IMULH_R : ScInstrInfo = ScInstrInfo::new(ScOpcode::IMULH_R, &[&MOP_MOV_RR, &MOP_MUL_R, &MOP_MOV_RR], 1, 0, 1);
static ISMULH_R : ScInstrInfo = ScInstrInfo::new(ScOpcode::ISMULH_R, &[&MOP_MOV_RR, &MOP_IMUL_R, &MOP_MOV_RR], 1, 0, 1);
static IMUL_RCP : ScInstrInfo = ScInstrInfo::new(ScOpcode::IMUL_RCP, &[&MOP_MOV_RI64, &MOP_IMUL_RR_DEP], -1, 1, 1);

const BLAKE_GEN_DATA_LEN : usize = 64;
pub struct Blake2Generator {
	index: usize,
	data: [u8; BLAKE_GEN_DATA_LEN],
	gen_params: Params,
}

impl Blake2Generator {
	pub fn new(seed: [u8; BLAKE_GEN_DATA_LEN-4], nonce: u32) -> Blake2Generator {
		let mut params = Params::new();
		params.hash_length(BLAKE_GEN_DATA_LEN);

		let mut data : [u8; BLAKE_GEN_DATA_LEN] = [0; BLAKE_GEN_DATA_LEN];
		data[..BLAKE_GEN_DATA_LEN-4].copy_from_slice(&seed);
		data[BLAKE_GEN_DATA_LEN-4..BLAKE_GEN_DATA_LEN].copy_from_slice(&nonce.to_le_bytes());

		return Blake2Generator{
			index: 0,
			data,
			gen_params: params,
		};
	}

	pub fn get_byte(&mut self) -> u8 {
		self.check_data(1);
		let v = self.data[self.index];
		self.index += 1;
		v
	}

	pub fn get_u32(&mut self) -> u32 {
		self.check_data(4);
		let v = u32::from_le_bytes(self.data[self.index..(self.index+4)].try_into().unwrap());
		self.index += 4;
		v
	}
	
	fn check_data(&mut self, needed: usize) {
		if self.index + needed > BLAKE_GEN_DATA_LEN {
			let out = self.gen_params.hash(&self.data);
			self.data = *out.as_array();
			self.index = 0;
		}
	}
}

pub struct DecoderBuffer {
	index: u32,
	counts: &'static[u32],
}

static BUFFER_484 : DecoderBuffer = DecoderBuffer{index: 0, counts: &[4,8,4]};
static BUFFER_7333 : DecoderBuffer = DecoderBuffer{index: 1, counts: &[7, 3, 3, 3]};
static BUFFER_3733 : DecoderBuffer = DecoderBuffer{index: 2, counts: &[3, 7, 3, 3]};
static BUFFER_493 : DecoderBuffer = DecoderBuffer{index: 3, counts: &[4, 9, 3]};
static BUFFER_4444 : DecoderBuffer = DecoderBuffer{index: 4, counts: &[4, 4, 4, 4]};
static BUFFFER_3310 : DecoderBuffer = DecoderBuffer{index: 5, counts: &[3, 3, 10]};

static DECODE_BUFFERS : [&DecoderBuffer; 4] = [
	&BUFFER_484,
	&BUFFER_7333,
	&BUFFER_3733,
	&BUFFER_493,	
];

impl DecoderBuffer {

	fn initial() -> DecoderBuffer {
		DecoderBuffer{
			index: 0,
			counts: &[]
		}
	}

	pub fn size(&self) -> usize {
		self.counts.len()
	}

	pub fn fetch_next(&self, instr: &ScInstr, decode_cycle: usize, mul_count: usize, gen: &mut Blake2Generator) -> &'static DecoderBuffer {
		if instr.info.op == ScOpcode::IMULH_R || instr.info.op == ScOpcode::ISMULH_R  {
			return &BUFFFER_3310;
		}
		if mul_count < decode_cycle + 1 {
			return &BUFFER_4444
		}
		if instr.info.op == ScOpcode::IMUL_RCP {
			return if gen.get_byte() & 0x1 == 1 {  &BUFFER_484 } else { &BUFFER_493 };
		}
		
		return DECODE_BUFFERS[(gen.get_byte() & 0x11) as usize];
	}
}

pub struct ScProgram<'a> {
	pub prog : Vec<ScInstr<'a>>,
}

impl ScProgram<'_> {
	pub fn generate(gen: &mut Blake2Generator) -> ScProgram<'static> {

		let mut prog = Vec::with_capacity(SUPERSCALAR_MAX_SIZE);

		let mut cycle = 0;

		let mut macro_op_index = 0;
		let mut macro_op_count = 0;
		let mut ports_saturated = false;
		let mut program_size = 0;
		let mut mul_count = 0;
		let mut decode_cycle = 0;
		let mut decode_buffer = &DecoderBuffer::initial();
		let mut current_instr = ScInstr{info: &NOP, mod_v: 0, imm32: 0, op_group: ScOpcode::INVALID, group_par_is_source: false, op_group_par: -1 };
		while decode_cycle < RANDOMX_SUPERSCALAR_LATENCY && !ports_saturated && program_size < SUPERSCALAR_MAX_SIZE {
			decode_buffer = decode_buffer.fetch_next(&current_instr, decode_cycle, mul_count, gen);
			
			let buffer_index = 0;
			while buffer_index < decode_buffer.size() {
				let top_cycle = cycle;
				if macro_op_index >= current_instr.info.size() {
					if ports_saturated || program_size >= SUPERSCALAR_MAX_SIZE {
						break;
					}

					current_instr = ScInstr::create_for_slot(gen, decode_buffer.counts[buffer_index], decode_buffer.index, decode_buffer.size() == buffer_index + 1, buffer_index == 0);
				    macro_op_index = 0
				}

				//TODO Impl 
			}
			
			
			decode_cycle += 1
		}

		ScProgram{prog}
	}
}
