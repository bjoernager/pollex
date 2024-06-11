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

use crate::arm32::Instruction;

use core::fmt::Display;

impl Display for Instruction {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use Instruction::*;

		match *self {
			Branch { condition, immediate } => {
				write!(f, "B{condition} <#{immediate}>")
			},

			BranchLink { condition, immediate } => {
				write!(f, "BL{condition} <#{immediate}>")
			},
		}
	}
}
