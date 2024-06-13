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

use crate::arm32::Unsigned;

use core::cmp::Ordering;
use core::fmt::{Display, Formatter};
use core::ops::{Add, Div, Mul, Sub};

/// A signed word.
///
/// This type defines the [`Add`], [`Sub`], [`Mul`], and [`Div`] traits for `Self` and [`i32`].
/// Internally, these implementations use wrapping arithemtic.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Signed(i32);

impl Signed {
	/// Constructs a new word.
	#[inline(always)]
	#[must_use]
	pub const fn new(value: i32) -> Self { Self(value) }

	/// Retrieves the word's value.
	#[inline(always)]
	#[must_use]
	pub const fn get(self) -> i32 { self.0 }

	/// Sets the value of the word.
	#[inline(always)]
	pub fn set(&mut self, value: i32) { self.0 = value }

	/// Reinterprets the word as unsigned.
	#[inline(always)]
	#[must_use]
	pub const fn as_unsigned(self) -> Unsigned { Unsigned::new(self.0 as u32) }
}

impl Add<Self> for Signed {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output { self + rhs.get() }
}

impl Add<i32> for Signed {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: i32) -> Self::Output { Self(self.0.wrapping_add(rhs)) }
}

impl Display for Signed {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		write!(f, "#{:+}", self.0)
	}
}

impl Div<Self> for Signed {
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: Self) -> Self::Output { self / rhs.get() }
}

impl Div<i32> for Signed {
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: i32) -> Self::Output { Self(self.0.wrapping_div(rhs)) }
}

impl Mul<Self> for Signed {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self::Output { self * rhs.get() }
}

impl Mul<i32> for Signed {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: i32) -> Self::Output { Self(self.0.wrapping_mul(rhs)) }
}

impl Sub<Self> for Signed {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output { self - rhs.get() }
}

impl Sub<i32> for Signed {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: i32) -> Self::Output { Self(self.0.wrapping_sub(rhs)) }
}

impl PartialEq<i32> for Signed {
	#[inline(always)]
	fn eq(&self, other: &i32) -> bool { self.0 == *other }
}

impl PartialOrd<i32> for Signed {
	#[inline(always)]
	fn partial_cmp(&self, other: &i32) -> Option<Ordering> { self.0.partial_cmp(other) }
}
