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

#[cfg(test)]
mod test;

mod display;

use crate::arm32::Condition;

/// An Arm32 instruction.
pub enum Instruction {
	Branch { condition: Condition, immediate: i32 },

	BranchLink { condition: Condition, immediate: i32 },
}
