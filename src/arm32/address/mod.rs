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

use crate::arm32::{Register, Shifter};

use core::fmt::{Display, Formatter};

/// An address operand.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Address {
	ImmediateOffset { base: Register, source: i32 },

	RegisterOffset { base: Register, source: Register },

	ScaledRegisterOffset { base: Register, source: Register, shift: Shifter },

}

impl Display for Address {
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		use Address::*;

		match *self {
			ImmediateOffset { base, source }
			=> write!(f, "[{base}, #{source}]"),

			RegisterOffset { base, source }
			=> write!(f, "[{base}, {source}]"),

			ScaledRegisterOffset { base, source, shift }
			=> write!(f, "[{base}, {source}, {shift}]"),
		}
	}
}
