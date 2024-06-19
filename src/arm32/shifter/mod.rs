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

use crate::{Error, Result};
use crate::arm32::Register;

use core::fmt::{Display, Formatter};

/// A shifter operand.
///
/// Some Arm instructions take these to minimise instruction usage.
/// For example, the following code:
///
/// ```as
/// LSL r1, r1, #2
/// ADD r0, r1
/// ```
///
/// is functionally equivalent to:
///
/// ```as
/// ADD r0, r1, LSL #2
/// ```
///
/// In fact, the mentioned `LSL` instruction will encode identically to the following:
///
/// ```as
/// MOV r1, r1, LSL #2
/// ```
///
/// # Immediates
///
/// Shifters can store any immediate value with the [`Immediate`](Shifter::Immediate) variant.
///
/// In theory, these values are limited to *at most* `24` bits on Arm, and to `11` bits on Thumb.
/// In practice, however, these are even further limited -- in some cases down to `3` bits.
///
/// # Shifts
///
/// As mentioned, shifters can also incorporate the functionality of other instructions.
/// This is done as *shifts* (hence the name *shifter*), and can use one of the following functions:
///
/// * Logical shift left --- `LSL`
/// * Logical shift right --- `LSR`
/// * Arithmetic shift right --- `ASR`
/// * Rotate right --- `ROR`
/// * Rotate right extend --- `RRX`
///
/// With the exception of `RRX`, all of these functions can take a *shift value* -- which sets the ammount of digits to shifts -- from either an immediate or a register.
/// For immediates, this is limited to the range `1` to `32`, inclusive, except for `LSL`, which uses the range `0` to `31` (again inclusive; see also *Registers*).
///
/// Therefore, the following two examples are somewhat equivalent:
///
/// ```as
/// ; First example:
/// ADD r0, r0, r1, LSL #1
///
/// ; Second example:
/// MOV r2, #1
/// ADD r0, r0, r1, LSL r2
/// ```
///
/// Of course, the first example has a smaller memory footprint and a (in theory) shorter runtime.
///
/// ## Registers
///
/// Whilst lone registers are accepted as shifter operands, they aren't encoded specially.
/// Instead, a single, unshifted register is encoded as itself shifted to the left by zero figures:
///
/// ```as
/// ; These are identical:
/// RSB r0, r0, r7
/// RSB r0, r0, r7, LSL #0
/// ```
///
/// This instead makes `LSL` a special case in that it cannot encode shift ammounts of `32` figures -- in contrast to the other shift functions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Shifter {
	ArithmeticShiftRightImmediate { source: Register, shift: u32 },

	ArithmeticShiftRightRegister { source: Register, shift: Register },

	Immediate(u32),

	LogicalShiftLeftImmediate { source: Register, shift: u32 },

	LogicalShiftLeftRegister { source: Register, shift: Register },

	LogicalShiftRightImmediate { source: Register, shift: u32 },

	LogicalShiftRightRegister { source: Register, shift: Register },

	RotateRightExtend { source: Register },

	RotateRightImmediate { source: Register, shift: u32 },

	RotateRightRegister { source: Register, shift: Register },
}

impl Shifter {
	/// Creates a new shifter from a register.
	///
	/// This is identical to creating a [`LogicalShiftLeftImmediate`](Shifter::LogicalShiftLeftImmediate) instance with `source: register` and `shift: 0x0`.
	#[inline(always)]
	#[must_use]
	pub const fn from_register(register: Register) -> Self {
		Self::LogicalShiftLeftImmediate { source: register, shift: 0x0 }
	}

	/// Collapses the shifter into a single register.
	///
	/// # Errors
	///
	/// If this shifter does **not** fit the pattern `LogicalShiftLeft { shift: 0x0, .. }`, an [`IllegalShifter`](Error::IllegalShifter) error is returned.
	#[inline]
	pub const fn as_register(self) -> Result<Register> {
		if let Self::LogicalShiftLeftImmediate { source: register, shift: 0x0 } = self {
			Ok(register)
		} else {
			Err(Error::IllegalShifter { reason: "cannot collapse to register" })
		}
	}
}

impl Display for Shifter {
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		use Shifter::*;

		match *self {
			ArithmeticShiftRightImmediate { source, shift }
			=> write!(f, "{source}, ASR #{shift}"),

			ArithmeticShiftRightRegister { source, shift }
			=> write!(f, "{source}, ASR {shift}"),

			Immediate(source)
			=> write!(f, "#{source}"),

			LogicalShiftLeftImmediate { source, shift: 0x0 }
			=> write!(f, "{source}"),

			LogicalShiftLeftImmediate { source, shift }
			=> write!(f, "{source}, LSL #{shift}"),

			LogicalShiftLeftRegister { source, shift }
			=> write!(f, "{source}, LSL {shift}"),

			LogicalShiftRightImmediate { source, shift }
			=> write!(f, "{source}, LSR #{shift}"),

			LogicalShiftRightRegister { source, shift }
			=> write!(f, "{source}, LSR {shift}"),

			RotateRightExtend { source }
			=> write!(f, "{source}, RRX"),

			RotateRightImmediate { source, shift }
			=> write!(f, "{source}, ROR #{shift}"),

			RotateRightRegister { source, shift }
			=> write!(f, "{source}, ROR {shift}"),
		}
	}
}
