// Copyright 2024 Gabriel Bjørnager Jensen.
//
// This file is part of Pollex.
//
// Pollex is free software: you can redistribute it
// and/or modify it under the terms of the GNU Af-
// fero General Public License as published by the
// Free Software Foundation, either version 3 of
// the License, or (at your option) any later ver-
// sion.
//
// Pollex is distributed in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Affero General Public License for more details.
//
// You should have received a copy of the GNU Af-
// fero General Public License along with Pollex.
// If not, see <https://www.gnu.org/licenses/>.

use crate::{assert_or_err, Error, Result};
use crate::arm32::{
	ArmOpcode,
	Instruction,
	InstructionCodec,
	Shifter,
};

use core::num::Wrapping;

fn add_shifter(mut opcode: u32, shifter: Shifter) -> Result<u32> {
	use Shifter::*;

	let get_shift_code = |shifter: Shifter| match shifter {
		LogicalShiftLeftImmediate { .. }     => 0b00,
		LogicalShiftRightImmediate { .. }    => 0b01,
		ArithmeticShiftRightImmediate { .. } => 0b10,
		RotateRightImmediate { .. }          => 0b11,

		_ => panic!("cannot get shifter code of `{shifter:?}`"),
	};

	match shifter {
		LogicalShiftLeftImmediate { source, shift: 0x0 }
		=> {
			opcode |= source as u32;
		}

		| ArithmeticShiftRightImmediate { source, shift }
		| LogicalShiftLeftImmediate { source, shift }
		| LogicalShiftRightImmediate { source, shift }
		| RotateRightImmediate { source, shift }
		=> {
			assert_or_err!(shift != 0x0, Error::IllegalImmediate { reason: "immediate shift cannot be null on arm" });

			let code = get_shift_code(shifter);

			opcode |= source as u32;
			opcode |= code << 0x5;
			opcode |= shift << 0x7;
		}

		RotateRightExtend { .. } => {
			todo!()
		}

		| ArithmeticShiftRightRegister { source, .. }
		| LogicalShiftLeftRegister { source, .. }
		| LogicalShiftRightRegister { source, .. }
		| RotateRightRegister { source, .. }
		=> {
			let _code = get_shift_code(shifter);

			opcode |= 0b00000000_00000000_00000000_00010000;
			opcode |= source as u32;
		}

		Immediate(source) => {
			let (source, rotate) = if source <= 0xFF {
				(source, 0x00)
			} else {
				todo!()
			};

			opcode |= 0b00000010_00000000_00000000_00000000;
			opcode |= source;
			opcode |= rotate << 0x8;
		}
	}

	Ok(opcode)
}

impl InstructionCodec {
	/// Encodes the given Arm instruction.
	///
	/// # Errors
	///
	/// If the operands of the provided instruction cannot be encoded in the given combination, or are incompatible with the mnemonic, an error is returned.
	pub fn encode_arm(&mut self, instruction: Instruction) -> Result<ArmOpcode> {
		use Instruction::*;

		let mut opcode = 0b00000000_00000000_00000000_00000000_u32;

		match instruction {
			Branch {
				predicate,
				..
			} => {
				opcode |= 0b00001011_00000000_00000000_00000000;
				opcode |= (predicate as u32) << 0x1C;
			}

			BranchExchange {
				predicate,
				source,
			} => {
				opcode |= 0b00000001_00101111_11111111_00010000;
				opcode |= source as u32;
				opcode |= (predicate as u32) << 0x1C;
			}

			BranchLink {
				predicate,
				..
			} => {
				opcode |= 0b00001010_00000000_00000000_00000000;
				opcode |= (predicate as u32) << 0x1C;
			}

			Breakpoint {
				immediate,
			} => {
				opcode |= 0b11100001_00100000_00000000_01110000;
				opcode |= immediate & 0b00000000_00000000_00000000_00001111;
				opcode |= (immediate & 0b00000000_00000000_11111111_11110000) << 0x4;
			}

			Move {
				predicate,
				destination,
				source,
				s,
			} => {
				opcode |= 0b00000001_10100000_00000000_00000000;
				opcode |= (destination as u32) << 0xC;
				opcode |= u32::from(s) << 0x14;
				opcode |= (predicate as u32) << 0x1C;

				opcode = add_shifter(opcode, source)?;
			}

			SoftwareInterrupt {
				predicate,
				immediate,
			} => {
				opcode |= 0b00001111_00000000_00000000_00000000;
				opcode |= immediate & 0b00000000_11111111_11111111_11111111;
				opcode |= (predicate as u32) << 0x1C;
			}

			_ => todo!(),
		}

		self.skip_words(0x1);

		Ok(opcode.into())
	}
}
