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
	Instruction,
	InstructionCodec,
	Predicate,
	Shifter,
	ThumbOpcode,
};

impl InstructionCodec {
	/// Encodes the given Thumb instruction.
	///
	/// # Errors
	///
	/// If the given instruction cannot be encoded for Thumb, an error is returned.
	pub fn encode_thumb(&mut self, instruction: Instruction) -> Result<ThumbOpcode> {
		use Instruction::*;

		let mut opcode = 0b00000000_00000000_u16;

		match instruction {
			Move {
				predicate,
				destination,
				source,
				s,
			} => {
				assert_or_err!(predicate == Predicate::Always, Error::IllegalPredicate);

				if s.is_on() {
					if let Shifter::Immediate { source } = source {
						assert_or_err!(destination.is_low(), Error::IllegalRegister);
						assert_or_err!(source <= 0xFF, Error::IllegalRegister);

						opcode |= 0b00100000_00000000;
						opcode |= source as u16;
						opcode |= (destination as u16) << 0x8;

					} else if let Shifter::LogicalShiftLeftImmediate { source, shift: 0x0 } = source {
						assert_or_err!(destination.is_low(), Error::IllegalRegister);
						assert_or_err!(source.is_low(), Error::IllegalRegister);

						opcode |= 0b00100000_00000000;
						opcode |= destination as u16;
						opcode |= (source as u16) << 0x3;
					} else {
						return Err(Error::IllegalInstruction);
					}
				} else if let Shifter::LogicalShiftLeftImmediate { source, shift: 0x0 } = source {
					opcode |= 0b01000110_00000000;

					let h0 = destination.is_high();
					let h1 = source.is_high();

					opcode |= destination as u16 & 0b00000000_00000111;
					opcode |= (source as u16 & 0b00000000_00000111) << 0x3;
					opcode |= u16::from(h0) << 0x6;
					opcode |= u16::from(h1) << 0x7;
				} else {
					// TODO: Shifters &c.
					todo!();
				}
			}

			_ => return Err(Error::IllegalInstruction),
		}

		self.address += ThumbOpcode::SIZE;
		Ok(opcode.into())
	}
}
