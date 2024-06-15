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

use core::fmt::{Display, Formatter};

/// Wrapping of [`core::result::Result`].
pub type Result<T> = core::result::Result<T, Error>;

/// A crate error.
#[derive(Clone, Debug)]
pub enum Error {
	/// The given immediate cannot be encoded.
	IllegalImmediate,

	/// The given instruction cannot be encoded.
	///
	/// Even valid instructions are not valid in *every* case.
	/// For example, most shifter operands and instruction predicates are **not** allowed in a Thumb context (with exceptions).
	IllegalInstruction,

	/// The given instruction predicate cannot be encoded.
	IllegalPredicate,

	/// The given register can currently not be used.
	IllegalRegister,

	/// Register name is not known.
	UnknownRegister,
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		use Error::*;

		let message = match *self {
			IllegalImmediate => "illegal immediate value",

			IllegalInstruction => "illegal instruction",

			IllegalPredicate => "illegal instruction predicate",

			IllegalRegister => "register not permitted here",

			UnknownRegister => "unknown register",
		};

		f.write_str(message)
	}
}

impl core::error::Error for Error { }
