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

use alloc::string::String;
use core::fmt::{Display, Formatter};

/// Wrapping of [`core::result::Result`].
pub type Result<T> = core::result::Result<T, Error>;

/// A crate error.
#[derive(Clone, Debug)]
pub enum Error {
	/// The given flag is currently not support.
	///
	/// Some Thumb instructions require the S flag to be on, for example.
	/// These yield this error if encoded with the S flag is off.
	IllegalFlag { reason: &'static str },

	/// The given immediate cannot be encoded.
	IllegalImmediate { reason: &'static str },

	/// The given instruction cannot be encoded.
	///
	/// Even valid instructions are not valid in *every* case.
	/// For example, most shifter operands and instruction predicates are **not** allowed in a Thumb context (with exceptions).
	IllegalInstruction { reason: &'static str },

	/// The given instruction predicate cannot be encoded.
	IllegalPredicate { reason: &'static str },

	/// The given register can currently not be used.
	IllegalRegister { reason: &'static str },

	/// The given shifter operand cannot be encoded.
	IllegalShifter { reason: &'static str },

	/// The provided opcode could not be decoded.
	InvalidOpcode,

	/// Mnemonic is not known.
	UnknownMnemonic(String),

	/// Register name is not known.
	UnknownRegister(String),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		use Error::*;

		match *self {
			IllegalFlag { reason } => write!(f, "illegal flag value: {reason}"),

			IllegalImmediate { reason } => write!(f, "illegal immediate value: {reason}"),

			IllegalInstruction { reason } => write!(f, "illegal instruction: {reason}"),

			IllegalPredicate { reason } => write!(f, "illegal instruction predicate: {reason}"),

			IllegalRegister { reason } => write!(f, "register not permitted here: {reason}"),

			IllegalShifter { reason } => write!(f, "shifter operand not permitted here: {reason}"),

			InvalidOpcode => write!(f, "invalid opcode"),

			UnknownMnemonic(ref s) => write!(f, "unknown mnemonic `{s}`"),

			UnknownRegister(ref s) => write!(f, "unknown register `{s}`"),
		}
	}
}

impl core::error::Error for Error { }
