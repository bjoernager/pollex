// Copyright 2021-2024 Gabriel Bjørnager Jensen.
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

/// An addressing mode.
pub enum Address {
	ArithmeticShiftRightImmediate { source: Register, shift: u32 },

	ArithmeticShiftRightRegister { source: Register, shift: Register },

	Immediage { immediate: u32 },

	LogicalShiftLeftImmediate { source: Register, shift: u32 },

	LogicalShiftLeftRegister { source: Register, shift: Register },

	LogicalShiftRightImmediate { source: Register, shift: u32 },

	LogicalShiftRightRegister { source: Register, shift: Register },

	RotateRightExtend { source: Register },

	RotateRightRegister { source: Register, shift: Register },
}

impl Display for Address {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use Address::*;

		match *self {
			ArithmeticShiftRightImmediate { source, shift } => {
				write!(f, "{source}, ASR #{shift}")
			},

			ArithmeticShiftRightRegister { source, shift } => {
				write!(f, "{source}, ASR {shift}")
			},

			Immediage { immediate } => {
				write!(f, "#{immediate}<")
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

			RotateRightRegister { source, shift } => {
				write!(f, "{source}, ROR {shift}")
			},
		}
	}
}
