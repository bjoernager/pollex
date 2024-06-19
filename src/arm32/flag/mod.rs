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

// TODO: This type is usefule for easily being for-
// matted, but is otherwise quite akwardly imple-
// mented (see the constant generic). Could we re-
// it?

use core::fmt::Display;

macro_rules! define_flag {
	{
		vis: $vis:vis,
		name: $name:ident,
		symbol: $symbol:literal,
		doc: $doc:literal $(,)?
	} => {
		#[doc = $doc]
		#[derive(Clone, Copy, Debug, Eq, PartialEq)]
		#[repr(u8)]
		$vis enum $name {
			Off = 0x0,
			On  = 0x1,
		}

		impl $name {
			/// Checks if the flag is on.
			#[inline(always)]
			#[must_use]
			pub const fn is_on(self) -> bool { self as u8 == Self::On as u8 }

			/// Checks if the flag is off.
			#[inline(always)]
			#[must_use]
			pub const fn is_off(self) -> bool { self as u8 == Self::Off as u8 }
		}

		impl Display for $name {
			fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
				const SYMBOL: &str = $symbol;

				if self.is_on() { write!(f, "{SYMBOL}")? };

				Ok(())
			}
		}

		impl From<bool> for $name {
			#[inline(always)]
			fn from(value: bool) -> Self {
				if value {
					Self::On
				} else {
					Self::Off
				}
			}
		}

		impl From<$name> for bool {
			#[inline(always)]
			fn from(value: $name) -> Self { value.is_on() }
		}

		impl From<$name> for u128 {
			#[inline(always)]
			fn from(value: $name) -> Self { Self::from(value.is_on()) }
		}

		impl From<$name> for u16 {
			#[inline(always)]
			fn from(value: $name) -> Self { Self::from(value.is_on()) }
		}

		impl From<$name> for u32 {
			#[inline(always)]
			fn from(value: $name) -> Self { Self::from(value.is_on()) }
		}

		impl From<$name> for u64 {
			#[inline(always)]
			fn from(value: $name) -> Self { Self::from(value.is_on()) }
		}

		impl From<$name> for u8 {
			#[inline(always)]
			fn from(value: $name) -> Self { Self::from(value.is_on()) }
		}

		impl From<$name> for usize {
			#[inline(always)]
			fn from(value: $name) -> Self { Self::from(value.is_on()) }
		}
	};
}

define_flag! {
	vis:    pub,
	name:   Bflag,
	symbol: "B",
	doc:    "A B flag.\n\nThis indicates binary operations on some memory instructions.\n"
}

define_flag! {
	vis:    pub,
	name:   Sflag,
	symbol: "S",
	doc:    "An S flag.\n\nThis indicates setting the status register on some arithmetic instructions.\n"
}

define_flag! {
	vis:    pub,
	name:   Tflag,
	symbol: "T",
	doc:    "An T flag.\n\nThis indicates translation on some memory instructions.\n"
}
