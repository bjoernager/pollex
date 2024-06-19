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
use crate::arm32::{
	Instruction,
	InstructionCodec,
	Predicate,
	Register,
	Sflag,
	Shifter,
	ThumbOpcode,
};

use core::num::Wrapping;

impl InstructionCodec {
	/// Decodes the given Thumb opcode.
	///
	/// # Errors
	///
	/// If the provided opcode is invalid (i.e. does not match any known pattern), an [InvalidOpcode](crate::Error::InvalidOpcode) error is returned.
	pub fn decode_thumb(&mut self, opcode: ThumbOpcode) -> Result<Instruction> {
		use Instruction::*;

		let opcode = opcode.to_u16();

		macro_rules! match_bits {
			($mask:expr) => {{
				opcode & $mask == $mask
			}};
		}

		let instruction = if match_bits!(0b11010000_00000000) {
			let predicate = (opcode & 0b00001111_00000000).wrapping_shr(0x8) as u8;
			let immediate = u32::from(opcode & 0b00000000_11111111);

			if predicate == 0b1111 {
				SoftwareInterrupt {
					predicate: Predicate::Always,
					immediate,
				}
			} else {
				let predicate = Predicate::from_u8(predicate).unwrap();

				let mut immediate = (immediate.wrapping_shl(0x18) as i32).wrapping_shl(0x17) as u32;
				immediate = immediate.wrapping_add(self.address.0).wrapping_add(0x4);

				Branch {
					predicate,
					immediate,
				}
			}
		} else if match_bits!(0b00011000_00000000) {
			let destination = Register::from_u8((opcode & 0b00000000_00000111) as u8).unwrap();
			let base        = Register::from_u8((opcode & 0b00000000_00111000).wrapping_shr(0x3) as u8).unwrap();

			let subtract = opcode & 0b00000010_00000000 != 0x0;

			let immediate_source = opcode & 0b00000100_00000000 != 0x0;
			if immediate_source {
				let source = u32::from((opcode & 0b00000001_11000000).wrapping_shr(0x6));

				if subtract {
					Subtract {
						predicate:   Predicate::Always,
						destination,
						base,
						source:      Shifter::Immediate(source),
						s:           Sflag::On,
					}
				} else {
					Add {
						predicate:   Predicate::Always,
						destination,
						base,
						source:      Shifter::Immediate(source),
						s:           Sflag::On,
					}
				}
			} else {
				// register source

				let source = Register::from_u8((opcode & 0b00000001_11000000).wrapping_shr(0x6) as u8).unwrap();

				if subtract {
					Subtract {
						predicate:   Predicate::Always,
						destination,
						base,
						source:      Shifter::from_register(source),
						s:           Sflag::On,
					}
				} else {
					Add {
						predicate:   Predicate::Always,
						destination,
						base,
						source:      Shifter::from_register(source),
						s:           Sflag::On,
					}
				}
			}
		} else if match_bits!(0b01000111_00000000) {
			let source = Register::from_u8((opcode & 0b00000000_01111000).wrapping_shr(0x3) as u8).unwrap();
			let l      = opcode & 0b00000000_10000000 != 0x0;

			if l {
				BranchLinkExchange {
					predicate: Predicate::Always,
					source:    Shifter::from_register(source),
				}
			} else {
				BranchExchange {
					predicate: Predicate::Always,
					source,
				}
			}
		} else {
			let _code = (opcode & 0b00011000_00000000).wrapping_shr(0xB) as u8;

			todo!();
		};

		self.address += Wrapping(ThumbOpcode::SIZE);
		Ok(instruction)
	}
}
