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

//! Arm instruction manipulator.
//!
//! This library is meant to be used by assemblers and disassembler, emulators, etc.
//! That is to leverage encoding and decoding of instructions.
//!
//! The goal of this project is to fully implement all Arm32 instruction set architectures, and, hopefully, Arm64 architectures as well.

#![no_std]

extern crate alloc;

pub mod arm32;
pub mod arm64;

macro_rules! use_mod {
	($vis:vis $name:ident) => {
		mod $name;
		$vis use $name::*;
	};
}
pub(in crate) use use_mod;

use_mod!(pub error);

#[macro_export]
macro_rules! assert_or_err {
	($predicate:expr, $error:expr) => {{
		if !($predicate) { return Err($error) };
	}};
}
