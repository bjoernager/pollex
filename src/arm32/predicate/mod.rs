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

use core::fmt::Display;

/// An instruction predicate (condition code).
///
/// Most Arm32 opcodes embed a condition code, as well as some Thumb opcodes.
///
/// Any 4-bit values is always a valid predicate *except* `0b1111`, which sometimes denotes a different instruction altogether
/// In most cases, it is invalid, however..
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Predicate {
	Equal              = 0b0000,
	NotEqual           = 0b0001,
	HigherOrSame       = 0b0010,
	Lower              = 0b0011,
	Minus              = 0b0100,
	Plus               = 0b0101,
	Overflow           = 0b0110,
	NoOverflow         = 0b0111,
	Higher             = 0b1000,
	LowerOrSame        = 0b1001,
	GreaterThanOrEqual = 0b1010,
	LessThan           = 0b1011,
	GreaterThan        = 0b1100,
	LessThanOrEqual    = 0b1101,
	Always             = 0b1110,
	//Never              = 0b1111,
}

impl Display for Predicate {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use Predicate::*;

		match *self {
			Equal              => write!(f, "EQ"),
			NotEqual           => write!(f, "NE"),
			HigherOrSame       => write!(f, "HS"),
			Lower              => write!(f, "LO"),
			Minus              => write!(f, "MI"),
			Plus               => write!(f, "PL"),
			Overflow           => write!(f, "VS"),
			NoOverflow         => write!(f, "VC"),
			Higher             => write!(f, "HI"),
			LowerOrSame        => write!(f, "LS"),
			GreaterThanOrEqual => write!(f, "GE"),
			LessThan           => write!(f, "LT"),
			GreaterThan        => write!(f, "GT"),
			LessThanOrEqual    => write!(f, "LE"),
			Always             => Ok(()), // No need to print this one.
			//Never              => write!(f, "NV"),
		}
	}
}
