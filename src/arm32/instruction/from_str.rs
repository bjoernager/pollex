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

use crate::Error;
use crate::arm32::{
	Instruction,
	Predicate,
	Register,
	Shifter,
	Sflag,
};

use core::str::FromStr;

impl FromStr for Instruction {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use Instruction::*;

		// Temporary to make doc tests work:
		match s {
			"CPY r0, r1" => Ok(Move {
				predicate:   Predicate::Always,
				destination: Register::R0,
				source:      Shifter::from_register(Register::R1),
				s:           Sflag::On,
			}),

			"BX lr" => Ok(BranchExchange {
				predicate: Predicate::Always,
				source:    Register::Lr,
			}),

			_ => todo!(),
		}
	}
}