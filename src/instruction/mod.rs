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

use core::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Register {
	R0  = 0x00,
	R1  = 0x01,
	R2  = 0x02,
	R3  = 0x03,
	R4  = 0x04,
	R5  = 0x05,
	R6  = 0x06,
	R7  = 0x07,
	R8  = 0x08,
	Sb  = 0x09, // R9
	Sl  = 0x0A, // R10
	R11 = 0x0B,
	Ip  = 0x0C, // R12
	Sp  = 0x0D, // R13
	Lr  = 0x0E, // R14
	Pc  = 0x0F, // R15
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
			Sb  => write!(f, "sb"),
			Sl  => write!(f, "sl"),
			R11 => write!(f, "r11"),
			Ip  => write!(f, "ip"),
			Sp  => write!(f, "sp"),
			Lr  => write!(f, "lr"),
			Pc  => write!(f, "pc"),
		}
	}
}
