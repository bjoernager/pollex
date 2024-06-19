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

/// A Thumb opcode.
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct ThumbOpcode(u16);

impl ThumbOpcode {
	/// The size of any Thumb opcode, in bytes.
	///
	/// This is **not** including `BL` prefixes and suffixes, which instead count as *two* opcodes.
	pub const SIZE: u32 = 0x2;

	/// Creates a new opcode from a primitive.
	#[inline(always)]
	#[must_use]
	pub const fn from_u16(value: u16) -> Self { Self(value.to_le()) }

	/// Extracts the opcode as a primitive.
	#[inline(always)]
	#[must_use]
	pub const fn to_u16(self) -> u16 { self.0.to_le() }
}

impl Debug for ThumbOpcode {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		write!(f, "{:016b}", self.to_u16())
	}
}

impl Display for ThumbOpcode {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		write!(f, "{:#010X}", self.to_u16())
	}
}

impl From<u16> for ThumbOpcode {
	#[inline(always)]
	fn from(value: u16) -> Self { Self::from_u16(value) }
}

impl PartialEq<u16> for ThumbOpcode {
	#[inline(always)]
	fn eq(&self, other: &u16) -> bool { self.0 == *other }
}

impl From<ThumbOpcode> for u16 {
	#[inline(always)]
	fn from(value: ThumbOpcode) -> Self { value.to_u16() }
}
