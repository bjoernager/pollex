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

use crate::Result;
use crate::arm32::{ArmOpcode, Instruction, Shifter};

impl Instruction {
	pub fn encode_arm(&self) -> Result<ArmOpcode> {
		use Instruction::*;

		let mut opcode = 0b00000000_00000000_00000000_00000000;

		match *self {
			Branch { predicate, .. } => {
				opcode |= 0b00001011_00000000_00000000_00000000;
				opcode |= (predicate as u32) << 0x1C;
			},

			BranchLink { predicate, .. } => {
				opcode |= 0b00001010_00000000_00000000_00000000;
				opcode |= (predicate as u32) << 0x1C;
				//opcode |= immediate.to_bits::<0x18>() >> 0x8
			},

			Breakpoint { immediate } => {
				opcode |= 0b11100001_00100000_00000000_01110000;
				opcode |= immediate.get() & 0b00000000_00000000_00000000_00001111;
				opcode |= (immediate.get() & 0b00000000_00000000_11111111_11110000) << 0x4;
			},

			Move { predicate, destination, source, s } => {
				opcode |= 0b00000001_10100000_00000000_00000000;
				opcode |= (destination as u32) << 0xC;
				opcode |= u32::from(s.is_on()) << 0x14;
				opcode |= (predicate as u32) << 0x1C;

				opcode = add_shifter(opcode, source);
			},

			SoftwareInterrupt { predicate, immediate } => {
				opcode |= 0b00001111_00000000_00000000_00000000;
				opcode |= immediate.get() & 0b00000000_11111111_11111111_11111111;
				opcode |= (predicate as u32) << 0x1C;
			},

			_ => todo!(),
		}

		Ok(ArmOpcode::from_u32(opcode))
	}
}

fn add_shifter(mut opcode: u32, shifter: Shifter) -> u32 {
	match shifter {
		| Shifter::ArithmeticShiftRightImmediate { source, shift }
		| Shifter::LogicalShiftLeftImmediate { source, shift }
		| Shifter::LogicalShiftRightImmediate { source, shift }
		| Shifter::RotateRightImmediate { source, shift }
		=> {
			let code = match shifter {
				Shifter::LogicalShiftLeftImmediate { .. }     => 0b00,
				Shifter::LogicalShiftRightImmediate { .. }    => 0b01,
				Shifter::ArithmeticShiftRightImmediate { .. } => 0b10,
				Shifter::RotateRightImmediate { .. }          => 0b11,

				_ => unreachable!(),
			};

			opcode |= source as u32;
			opcode |= code << 0x5;
			opcode |= shift.get() << 0x7;
		},

		Shifter::RotateRightExtend { .. } => {
			todo!()
		},

		| Shifter::ArithmeticShiftRightRegister { source, .. }
		| Shifter::LogicalShiftLeftRegister { source, .. }
		| Shifter::LogicalShiftRightRegister { source, .. }
		| Shifter::RotateRightRegister { source, .. }
		=> {
			let _code = match shifter {
				Shifter::LogicalShiftLeftRegister { .. }     => 0b00,
				Shifter::LogicalShiftRightRegister { .. }    => 0b01,
				Shifter::ArithmeticShiftRightRegister { .. } => 0b10,
				Shifter::RotateRightRegister { .. }          => 0b11,

				_ => unreachable!(),
			};

			opcode |= 0b00000000_00000000_00000000_00010000;
			opcode |= source as u32;
		},

		Shifter::Immediate { immediate } => {
			let (immediate, rotate) = if immediate <= 0xFF {
				let immediate = immediate.get();

				(immediate, 0x00)
			} else {
				todo!()
			};

			opcode |= 0b00000010_00000000_00000000_00000000;
			opcode |= immediate;
			opcode |= rotate << 0x8;
		},
	}

	opcode
}
