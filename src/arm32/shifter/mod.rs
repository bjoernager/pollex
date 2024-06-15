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

use crate::arm32::Register;

use core::fmt::Display;

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
/// In fact, the first example will encode identically to the following:
///
/// ```as
/// MOV r1, r1, LSL #2
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Shifter {
	ArithmeticShiftRightImmediate { source: Register, shift: u32 },

	ArithmeticShiftRightRegister { source: Register, shift: Register },

	Immediate { immediate: u32 },

	LogicalShiftLeftImmediate { source: Register, shift: u32 },

	LogicalShiftLeftRegister { source: Register, shift: Register },

	LogicalShiftRightImmediate { source: Register, shift: u32 },

	LogicalShiftRightRegister { source: Register, shift: Register },

	RotateRightExtend { source: Register },

	RotateRightImmediate { source: Register, shift: u32 },

	RotateRightRegister { source: Register, shift: Register },
}

impl Display for Shifter {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use Shifter::*;

		match *self {
			ArithmeticShiftRightImmediate { source, shift } => {
				write!(f, "{source}, ASR #{shift}")
			},

			ArithmeticShiftRightRegister { source, shift } => {
				write!(f, "{source}, ASR {shift}")
			},

			Immediate { immediate } => {
				write!(f, "#{immediate}<")
			},

			LogicalShiftLeftImmediate { source, shift: 0x0 } => {
				write!(f, "{source}")
			},

			LogicalShiftLeftImmediate { source, shift } => {
				write!(f, "{source}, LSL #{shift}")
			},

			LogicalShiftLeftRegister { source, shift } => {
				write!(f, "{source}, LSL {shift}")
			},

			LogicalShiftRightImmediate { source, shift } => {
				write!(f, "{source}, LSR #{shift}")
			},

			LogicalShiftRightRegister { source, shift } => {
				write!(f, "{source}, LSR {shift}")
			},

			RotateRightExtend { source } => {
				write!(f, "{source}, RRX")
			},

			RotateRightImmediate { source, shift } => {
				write!(f, "{source}, ROR #{shift}")
			},

			RotateRightRegister { source, shift } => {
				write!(f, "{source}, ROR {shift}")
			},
		}
	}
}
