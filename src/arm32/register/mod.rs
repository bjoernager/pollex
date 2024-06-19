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

use alloc::borrow::ToOwned;

use crate::Error;

use core::fmt::Display;
use core::mem::transmute;
use core::str::FromStr;

/// An Arm register.
///
/// Some opcodes can only encode specific registers.
/// And yet other opcodes only encode a single register (which is then inferred from the opcode in question).
///
/// # Low registers
///
/// The registers whose identifiers can fit into `3` bits (i.e. registers from `r0` to `r7`, inclusive) are called *low* registers.
///
/// Some instructions (primarily on Thumb) can only encode low registers.
/// In the case of Thumb, a few instructions can be used to move values to and from low registers.
///
/// # High registers
///
/// In contrast to low registers, registers that can only be addressed using `4` bíts are called *high registers*.
///
/// Some instructions are pedantic about combining both low and high registers.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Register {
	R0  = 0b0000,
	R1  = 0b0001,
	R2  = 0b0010,
	R3  = 0b0011,
	R4  = 0b0100,
	R5  = 0b0101,
	R6  = 0b0110,
	R7  = 0b0111,
	R8  = 0b1000,
	R9  = 0b1001, // Sb
	R10 = 0b1010, // Sl
	R11 = 0b1011,
	R12 = 0b1100, // Ip
	Sp  = 0b1101, // R13
	Lr  = 0b1110, // R14
	Pc  = 0b1111, // R15
}

impl Register {
	/// Converts the provided byte into a register identifier.
	/// If the byte's value is not a valid identifier, [`None`] is returned.
	///
	/// This conversion is valid for all `4`-bit values.
	#[inline]
	#[must_use]
	pub const fn from_u8(value: u8) -> Option<Self> {
		if value <= 0b1111 {
			Some(unsafe { transmute::<u8, Self>(value) })
		} else {
			None
		}
	}

	/// Checks if the register is a low register.
	///
	/// That is, it is any of `r0`, `r1`, `r2`, `r3`, `r4`, `r5`, `r6`, or `r7` -- or any alias herof.
	/// See also [`is_high`](Self::is_high).
	#[inline(always)]
	#[must_use]
	pub const fn is_low(self) -> bool { (self as u8) <= 0x7 }

	/// Checks if the register is a high register.
	///
	/// That is, it is any of `r8`, `r9`, `r10`, `r11`, `r12`, `sp`, `lr`, or `pc` -- or any alias herof.
	/// See also [`is_low`](Self::is_low).
	#[inline(always)]
	#[must_use]
	pub const fn is_high(self) -> bool { (self as u8) > 0x7 }
}

impl Display for Register {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use Register::*;

		match *self {
			R0  => write!(f, "r0"),
			R1  => write!(f, "r1"),
			R2  => write!(f, "r2"),
			R3  => write!(f, "r3"),
			R4  => write!(f, "r4"),
			R5  => write!(f, "r5"),
			R6  => write!(f, "r6"),
			R7  => write!(f, "r7"),
			R8  => write!(f, "r8"),
			R9  => write!(f, "r9"),
			R10 => write!(f, "r10"),
			R11 => write!(f, "r11"),
			R12 => write!(f, "r12"),
			Sp  => write!(f, "sp"),
			Lr  => write!(f, "lr"),
			Pc  => write!(f, "pc"),
		}
	}
}

impl FromStr for Register {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use Register::*;

		match s.to_lowercase().as_str() {
			| "r0"
			| "a1"
			=> Ok(R0),

			| "r1"
			| "a2"
			=> Ok(R1),

			| "r2"
			| "a3"
			=> Ok(R2),

			| "r3"
			| "a4"
			=> Ok(R3),

			| "r4"
			| "v1"
			=> Ok(R4),

			| "r5"
			| "v2"
			=> Ok(R5),

			| "r6"
			| "v3"
			=> Ok(R6),

			| "r7"
			| "v4"
			=> Ok(R7),

			| "r8"
			| "v5"
			=> Ok(R8),

			| "r9"
			| "sb"
			| "v6"
			=> Ok(R9),

			| "r10"
			| "sl"
			| "v7"
			=> Ok(R10),

			| "r11"
			| "v8"
			=> Ok(R11),

			| "ip"
			| "r12"
			=> Ok(R12),

			| "r13"
			| "sp"
			=> Ok(Sp),

			| "lr"
			| "r14"
			=> Ok(Lr),

			| "pc"
			| "r15"
			=> Ok(Pc),

			_ => Err(Error::UnknownRegister(s.to_owned()))
		}
	}
}
