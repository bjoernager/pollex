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

use core::fmt::{Debug, Display, Formatter};

/// An Arm opcode.
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct ArmOpcode(u32);

impl ArmOpcode {
	/// Creates a new opcode from a primitive.
	#[inline(always)]
	#[must_use]
	pub const fn from_u32(value: u32) -> Self { Self(value.to_le()) }

	/// Extracts the opcode as a primitive.
	#[inline(always)]
	#[must_use]
	pub const fn to_u32(self) -> u32 { self.0.to_le() }
}

impl Debug for ArmOpcode {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		write!(f, "{:032b}", self.to_u32())
	}
}

impl Display for ArmOpcode {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		write!(f, "{:#018X}", self.to_u32())
	}
}

impl PartialEq<u32> for ArmOpcode {
	#[inline(always)]
	fn eq(&self, other: &u32) -> bool { self.0 == *other }
}
