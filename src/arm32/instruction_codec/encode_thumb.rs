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
	Sflag,
	Shifter,
	ThumbOpcode,
};

use core::num::Wrapping;

macro_rules! quick_assert {
	(base_equals_destination: $base:expr, $destination:expr) => {{
		assert_or_err!($base == $destination, Error::IllegalRegister { reason: "base must also be destination on thumb" });
	}};

	(low_register: $register:expr) => {{
		assert_or_err!($register.is_low(), Error::IllegalRegister { reason: "cannot encode low register on thumb" });
	}};

	(predicate_always: $predicate:expr) => {{
		assert_or_err!($predicate == Predicate::Always, Error::IllegalPredicate { reason: "must be `AL` on thumb" });
	}};

	(s_flag_on: $flag:expr) => {{
		assert_or_err!($flag.is_on(), Error::IllegalFlag { reason: "s flag must be on on thumb" });
	}};

	(source_equals_destination: $source:expr, $destination:expr) => {{
		assert_or_err!($source == $destination, Error::IllegalRegister { reason: "source must also be destination on thumb" });
	}};
}

#[inline]
const fn encode_shift_immediate(shift: u32) -> Result<u32> {
	match shift {
		0x0 => Err(Error::IllegalImmediate { reason: "immediate shifts cannot be null on thumb" }),

		0x1..=0x1F => Ok(shift),

		0x20 => Ok(0x0),

		_ => Err(Error::IllegalImmediate { reason: "immediate shifts cannot be greater than (32) on thumb" }),
	}
}

impl InstructionCodec {
	/// Encodes the given Thumb instruction.
	///
	/// # Errors
	///
	/// If the given instruction cannot be encoded for Thumb, an error is returned.
	pub fn encode_thumb(&mut self, instruction: Instruction) -> Result<(ThumbOpcode, Option<ThumbOpcode>)> {
		use Instruction::*;

		let mut opcode = (0b00000000_00000000_u16, 0b00000000_00000000_u16);
		let mut has_opcode1 = false;

		match instruction {
//			Add {
//				predicate,
//				destination,
//				base,
//				source,
//				s,
//			} => {
//
//			}

			AddCarry {
				predicate,
				destination,
				base,
				source,
				s,
			} => {
				let source = source.as_register()?;

				quick_assert!(predicate_always: predicate);
				quick_assert!(base_equals_destination: base, destination);
				quick_assert!(low_register: destination);
				quick_assert!(low_register: source);
				quick_assert!(s_flag_on: s);

				opcode.0 |= 0b01000001_01000000;
				opcode.0 |= destination as u16;
				opcode.0 |= (source as u16).wrapping_shl(0x3);
			}

			And {
				predicate,
				destination,
				base,
				source,
				s,
			} => {
				let source = source.as_register()?;

				quick_assert!(predicate_always: predicate);
				quick_assert!(base_equals_destination: base, destination);
				quick_assert!(low_register: destination);
				quick_assert!(low_register: source);
				quick_assert!(s_flag_on: s);

				opcode.0 |= 0b01000000_00000000;
				opcode.0 |= destination as u16;
				opcode.0 |= (source as u16).wrapping_shl(0x3);
			}

			BitClear {
				predicate,
				destination,
				base,
				source,
				s,
			} => {
				let source = source.as_register()?;

				quick_assert!(predicate_always: predicate);
				quick_assert!(base_equals_destination: base, destination);
				quick_assert!(low_register: destination);
				quick_assert!(low_register: source);
				quick_assert!(s_flag_on: s);

				opcode.0 |= 0b01000011_10000000;
				opcode.0 |= destination as u16;
				opcode.0 |= (source as u16).wrapping_shl(0x3);
			}

			Branch {
				predicate,
				immediate,
			} => {
				let offset = (Wrapping(immediate) - self.address - Wrapping(0x4)).0 as i32;

				if predicate == Predicate::Always {
					assert_or_err!(offset >= -0x800, Error::IllegalImmediate { reason: "cannot enocde offset larger than (-2048) on thumb" });
					assert_or_err!(offset <= 0x7FE, Error::IllegalImmediate { reason: "cannot enocde offset larger than (2046) on thumb" });

					let offset = offset as u32;

					opcode.0 |= 0b11100000_00000000;
					opcode.0 |= ((offset & 0b00001111_11111111).wrapping_shr(0x1)) as u16;
				} else {
					opcode.0 |= 0b11010000_00000000;
					opcode.0 |= (predicate as u16).wrapping_shl(0x8);
					opcode.0 |= ((offset & 0b00000001_11111111).wrapping_shr(0x1)) as u16;
				}
			}

			BranchExchange {
				predicate,
				source,
			} => {
				quick_assert!(predicate_always: predicate);

				opcode.0 |= 0b01000111_00000000;
				opcode.0 |= (source as u16).wrapping_shl(0x3);
			}

			BranchLink {
				predicate,
				source,
			} => {
				quick_assert!(predicate_always: predicate);

				let offset = (Wrapping(source) - self.address - Wrapping(0x4)).0 as i32;

				assert_or_err!(offset >= -0x00400000, Error::IllegalImmediate { reason: "cannot encode offset larger than (-4194304) on thumb" });
				assert_or_err!(offset <= 0x003FFFFE, Error::IllegalImmediate { reason: "cannot encode offset larger than (4194302) on thumb" });
				assert_or_err!(offset % 0x2 == 0x0, Error::IllegalImmediate { reason: "cannot encode uneven offset on thumb" });

				let offset = offset as u32;

				opcode.0 |= 0b11110000_00000000;
				opcode.0 |= ((offset & 0b00000000_00111111_11110000_00000000).wrapping_shr(0xC)) as u16;

				opcode.1 |= 0b11111000_00000000;
				opcode.1 |= ((offset & 0b00000000_00000000_00001111_11111110).wrapping_shr(0x1)) as u16;

				has_opcode1 = true;
			}

			BranchLinkExchange {
				predicate,
				source,
			} => {
				quick_assert!(predicate_always: predicate);

				let source = source.as_register()
					.map_err(|_| Error::IllegalShifter { reason: "can only encode registers on thumb" })?;

				opcode.0 |= 0b01000111_10000000;
				opcode.0 |= (source as u16).wrapping_shl(0x3);
			}

			Breakpoint {
				immediate,
			} => {
				assert_or_err!(immediate <= 0xFF, Error::IllegalImmediate { reason: "cannot encode larger than (255) on thumb" });

				opcode.0 |= 0b10111110_00000000;
				opcode.0 |= immediate as u16;
			}

			ExclusiveOr {
				predicate,
				destination,
				base,
				source,
				s,
			} => {
				let source = source.as_register()?;

				quick_assert!(predicate_always: predicate);
				quick_assert!(base_equals_destination: base, destination);
				quick_assert!(low_register: destination);
				quick_assert!(low_register: source);
				quick_assert!(s_flag_on: s);

				opcode.0 |= 0b01000000_01000000;
				opcode.0 |= destination as u16;
				opcode.0 |= (source as u16).wrapping_shl(0x3);
			}

			InclusiveOr {
				predicate,
				destination,
				base,
				source,
				s,
			} => {
				let source = source.as_register()?;

				quick_assert!(predicate_always: predicate);
				quick_assert!(base_equals_destination: base, destination);
				quick_assert!(low_register: destination);
				quick_assert!(low_register: source);
				quick_assert!(s_flag_on: s);

				opcode.0 |= 0b01000011_00000000;
				opcode.0 |= destination as u16;
				opcode.0 |= (source as u16).wrapping_shl(0x3);
			}

			Move {
				predicate,
				destination,
				source:      Shifter::ArithmeticShiftRightImmediate { source, shift },
				s,
			} => {
					quick_assert!(predicate_always: predicate);
					quick_assert!(source_equals_destination: source, destination);
					quick_assert!(low_register: destination);
					quick_assert!(low_register: source);
					quick_assert!(s_flag_on: s);

					let shift = encode_shift_immediate(shift)?;

					opcode.0 |= 0b00010000_00000000;
					opcode.0 |= destination as u16;
					opcode.0 |= (source as u16).wrapping_shl(0x3);
					opcode.0 |= (shift as u16).wrapping_shl(0x6);
			}

			Move {
				predicate,
				destination,
				source:      Shifter::ArithmeticShiftRightRegister { source, shift },
				s,
			} => {
					quick_assert!(predicate_always: predicate);
					quick_assert!(source_equals_destination: source, destination);
					quick_assert!(low_register: destination);
					quick_assert!(low_register: source);
					quick_assert!(s_flag_on: s);

					opcode.0 |= 0b01000001_00000000;
					opcode.0 |= destination as u16;
					opcode.0 |= (shift as u16).wrapping_shl(0x3);
			}

			Move {
				predicate,
				destination,
				source:      Shifter::LogicalShiftLeftImmediate { source, shift },
				s,
			} => {
					quick_assert!(predicate_always: predicate);
					quick_assert!(source_equals_destination: source, destination);
					quick_assert!(low_register: destination);
					quick_assert!(low_register: source);
					quick_assert!(s_flag_on: s);

					// Remember that LSL does takes null shifts.
					assert_or_err!(shift <= 0x1F, Error::IllegalImmediate { reason: "shift must be at most (31)" });

					opcode.0 |= destination as u16;
					opcode.0 |= (source as u16).wrapping_shl(0x3);
					opcode.0 |= (shift as u16).wrapping_shl(0x6);
			}

			Move {
				predicate,
				destination,
				source:      Shifter::LogicalShiftLeftRegister { source, shift },
				s,
			} => {
					quick_assert!(predicate_always: predicate);
					quick_assert!(source_equals_destination: source, destination);
					quick_assert!(low_register: destination);
					quick_assert!(low_register: source);
					quick_assert!(s_flag_on: s);

					opcode.0 |= 0b01000000_10000000;
					opcode.0 |= destination as u16;
					opcode.0 |= (shift as u16).wrapping_shl(0x3);
			}

			Move {
				predicate,
				destination,
				source:      Shifter::LogicalShiftRightImmediate { source, shift },
				s,
			} => {
					quick_assert!(predicate_always: predicate);
					quick_assert!(source_equals_destination: source, destination);
					quick_assert!(low_register: destination);
					quick_assert!(low_register: source);
					quick_assert!(s_flag_on: s);

					let shift = encode_shift_immediate(shift)?;

					opcode.0 |= 0b00001000_00000000;
					opcode.0 |= destination as u16;
					opcode.0 |= (source as u16).wrapping_shl(0x3);
					opcode.0 |= (shift as u16).wrapping_shl(0x6);
			}

			Move {
				predicate,
				destination,
				source:      Shifter::LogicalShiftRightRegister { source, shift },
				s,
			} => {
					quick_assert!(predicate_always: predicate);
					quick_assert!(source_equals_destination: source, destination);
					quick_assert!(low_register: destination);
					quick_assert!(low_register: source);
					quick_assert!(s_flag_on: s);

					opcode.0 |= 0b01000000_11000000;
					opcode.0 |= destination as u16;
					opcode.0 |= (shift as u16).wrapping_shl(0x3);
			}

			Move {
				predicate,
				destination,
				source:      Shifter::RotateRightRegister { source, shift },
				s,
			} => {
				quick_assert!(predicate_always: predicate);
				quick_assert!(source_equals_destination: source, destination);
				quick_assert!(low_register: destination);
				quick_assert!(low_register: source);
				quick_assert!(s_flag_on: s);

				opcode.0 |= 0b01000001_11000000;
				opcode.0 |= destination as u16;
				opcode.0 |= (shift as u16).wrapping_shl(0x3);
			}

			Move {
				predicate,
				destination,
				source,
				s:           Sflag::Off,
			} => {
				quick_assert!(predicate_always: predicate);

				if let Ok(source) = source.as_register() {
					opcode.0 |= 0b01000110_00000000;

					let h0 = destination.is_high();
					let h1 = source.is_high();

					opcode.0 |= destination as u16 & 0b00000000_00000111;
					opcode.0 |= (source as u16 & 0b00000000_00000111).wrapping_shl(0x3);
					opcode.0 |= u16::from(h0).wrapping_shl(0x6);
					opcode.0 |= u16::from(h1).wrapping_shl(0x7);
				} else {
					return Err(Error::IllegalShifter { reason: "can only encode registers with s flag off" });
				}
			}

			Move {
				predicate,
				destination,
				source,
				s:           Sflag::On,
			} => {
				quick_assert!(predicate_always: predicate);

				if let Shifter::Immediate(source) = source {
					quick_assert!(low_register: destination);

					assert_or_err!(source <= 0xFF, Error::IllegalImmediate { reason: "cannot encode larger than (255) on thumb" });

					opcode.0 |= 0b00100000_00000000;
					opcode.0 |= source as u16;
					opcode.0 |= (destination as u16).wrapping_shl(0x8);
				} else if let Ok(source) = source.as_register() {
					quick_assert!(low_register: destination);
					quick_assert!(low_register: source);

					opcode.0 |= 0b00100000_00000000;
					opcode.0 |= destination as u16;
					opcode.0 |= (source as u16).wrapping_shl(0x3);
				} else {
					return Err(Error::IllegalShifter { reason: "not encodable on thumb" });
				}
			}

			MoveNot {
				predicate,
				destination,
				source,
				s,
			} => {
				let source = source.as_register()?;

				quick_assert!(predicate_always: predicate);
				quick_assert!(low_register: destination);
				quick_assert!(low_register: source);
				quick_assert!(s_flag_on: s);

				opcode.0 |= 0b01000011_11000000;
				opcode.0 |= destination as u16;
				opcode.0 |= (source as u16).wrapping_shl(0x3);
			}

			Multiply {
				predicate,
				destination,
				base,
				source,
				s,
			} => {
				quick_assert!(predicate_always: predicate);
				quick_assert!(base_equals_destination: base, destination);
				quick_assert!(low_register: destination);
				quick_assert!(low_register: source);
				quick_assert!(s_flag_on: s);

				opcode.0 |= 0b01000011_01000000;
				opcode.0 |= destination as u16;
				opcode.0 |= (source as u16).wrapping_shl(0x3);
			}

			SoftwareInterrupt {
				predicate,
				immediate,
			} => {
				quick_assert!(predicate_always: predicate);

				assert_or_err!(immediate <= 0xFF, Error::IllegalImmediate { reason: "cannot encode larger than (255) on thumb" });

				opcode.0 |= 0b11011111_00000000;
				opcode.0 |= immediate as u16;
			}

			SubtractCarry {
				predicate,
				destination,
				base,
				source,
				s,
			} => {
				let source = source.as_register()?;

				quick_assert!(predicate_always: predicate);
				quick_assert!(base_equals_destination: base, destination);
				quick_assert!(low_register: destination);
				quick_assert!(low_register: source);
				quick_assert!(s_flag_on: s);

				opcode.0 |= 0b01000000_11000000;
				opcode.0 |= destination as u16;
				opcode.0 |= (source as u16).wrapping_shl(0x3);
			}

			_ => return Err(Error::IllegalInstruction { reason: "not supported on thumb" } ),
		}

		let opcode_count = 0x1 + u32::from(has_opcode1);
		self.skip_halfwords(opcode_count);

		Ok((opcode.0.into(), has_opcode1.then_some(opcode.1.into())))
	}
}
