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

//! [Pollex](https://crates.io/crates/pollex/) is a Rust-written library for manipulating instructions of Arm ISAs.
//!
//! # Support
//!
//! Pollex supports encoding of instructions to both Arm and Thumb on Arm32 targets.
//! Arm64 support is planned.
//!
//! # Usage
//!
//! Instructions can be created directly using the [`Instruction`](arm32::Instruction) type:
//!
//! ```
//! use pollex::arm32::{
//!     Instruction,
//!     Predicate,
//!     Register,
//!     Sflag,
//!     Shifter,
//! };
//!
//! // MOVS r0, r1
//! let instr = Instruction::Move {
//!     predicate:   Predicate::Always,
//!     destination: Register::R0,
//!     source:      Shifter::from_register(Register::R1),
//!     s:           Sflag::On,
//! };
//! ```
//!
//! Instructions can also be parsed from strings:
//!
//! ```
//! use pollex::arm32::{
//!     Instruction,
//!     Predicate,
//!     Register,
//!     Sflag,
//!     Shifter,
//! };
//!
//! let instr: Instruction = "CPY r0, r1".parse()?;
//!
//! // Is equivalent to:
//!
//! let instr = Instruction::Move {
//!     predicate:   Predicate::Always,
//!     destination: Register::R0,
//!     source:      Shifter::from_register(Register::R1),
//!     s:           Sflag::Off,
//! };
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! But do note that the latter is currently **not** implemented.
//!
//! Instructions can be encoded to both Arm and Thumb using the [`InstructionCodec`](arm32::InstructionCodec) type:
//!
//! ```
//! use pollex::arm32::{Instruction, InstructionCodec};
//!
//! let instr: Instruction = "BX lr".parse()?;
//!
//! let mut codec = InstructionCodec::new();
//!
//! let arm_opcode   = codec.encode_arm(instr)?;
//! let thumb_opcode = codec.encode_thumb(instr)?;
//!
//! assert_eq!(arm_opcode, 0b11100001_00101111_11111111_00011110);
//! assert_eq!(thumb_opcode.0, 0b01000111_01110000);
//! assert_eq!(thumb_opcode.1, None);
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Copyright & Licensing
//!
//! Copyright 2024 Gabriel Bjørnager Jensen.
//!
//! This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
//!
//! You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
//!
//! # Trademarks
//!
//! Arm and Thumb are registered trademarks or trademarks of Arm Limited (or its subsidiaries or affiliates).

#![doc(html_logo_url = "https://gitlab.com/bjoernager/pollex/-/raw/master/pollex-monochrome.svg?ref_type=heads")]

#![no_std]

extern crate alloc;

#[cfg(test)]
#[doc(hidden)]
mod test;

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

/// Asserts the given predicate.
///
/// In contrast to [`assert`], this macro makes the caller return an error instead of panicking.
#[macro_export]
macro_rules! assert_or_err {
	($predicate:expr, $error:expr) => {{
		if !($predicate) { return Err($error) };
	}};
}
