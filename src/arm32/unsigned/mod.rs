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

use crate::arm32::Signed;

use core::cmp::Ordering;
use core::fmt::{Display, Formatter};
use core::ops::{Add, Div, Mul, Sub};

/// An unsigned word.
///
/// This type defines the [`Add`], [`Sub`], [`Mul`], and [`Div`] traits for `Self` and [`u32`].
/// Internally, these implementations use wrapping arithemtic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Unsigned(u32);

impl Unsigned {
	/// Constructs a new word.
	#[inline(always)]
	#[must_use]
	pub const fn new(value: u32) -> Self { Self(value) }

	/// Retrieves the word's value.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> u32 { self.0 }

	/// Sets the value of the word.
	#[inline(always)]
	pub fn set(&mut self, value: u32) { self.0 = value }

	/// Reinterprets the word as signed.
	#[allow(clippy::cast_possible_wrap)]
	#[inline(always)]
	#[must_use]
	pub const fn as_signed(self) -> Signed { Signed::new(self.0 as i32) }

	/// Adds a signed offset to the word.
	#[inline(always)]
	#[must_use]
	pub const fn offset(self, value: i32) -> Self { Self(self.get().wrapping_add_signed(value)) }
}

impl Add<Self> for Unsigned {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output { self + rhs.get() }
}

impl Add<u32> for Unsigned {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: u32) -> Self::Output { Self(self.0.wrapping_add(rhs)) }
}

impl Display for Unsigned {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		write!(f, "#{}", self.0)
	}
}

impl Div<Self> for Unsigned {
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: Self) -> Self::Output { self / rhs.get() }
}

impl Div<u32> for Unsigned {
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: u32) -> Self::Output { Self(self.0.wrapping_div(rhs)) }
}

impl Mul<Self> for Unsigned {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self::Output { self * rhs.get() }
}

impl Mul<u32> for Unsigned {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: u32) -> Self::Output { Self(self.0.wrapping_mul(rhs)) }
}

impl PartialEq<u32> for Unsigned {
	#[inline(always)]
	fn eq(&self, other: &u32) -> bool { self.0 == *other }
}

impl PartialOrd<u32> for Unsigned {
	#[inline(always)]
	fn partial_cmp(&self, other: &u32) -> Option<Ordering> { self.0.partial_cmp(other) }
}

impl Sub<Self> for Unsigned {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output { self - rhs.get() }
}

impl Sub<u32> for Unsigned {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: u32) -> Self::Output { Self(self.0.wrapping_sub(rhs)) }
}
