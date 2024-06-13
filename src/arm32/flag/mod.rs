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

/// A flag.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Flag<const C: char> {
	Off = 0x0,
	On  = 0x1,
}

impl<const C: char> Flag<C> {
	#[inline(always)]
	#[must_use]
	pub const fn is_off(self) -> bool { self as u8 == Self::Off as u8 }

	#[inline(always)]
	#[must_use]
	pub const fn is_on(self) -> bool { self as u8 == Self::On as u8 }
}

impl<const C: char> Display for Flag<C> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		if self.is_on() {
			write!(f, "{C}")?;
		}

		Ok(())
	}
}
