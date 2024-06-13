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

#[cfg(test)]
mod test;

mod display;
mod encode_arm;

use crate::arm32::{
	Predicate,
	Flag,
	Register,
	Shifter,
	Signed,
	Unsigned,
};

/// An Arm32 instruction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
	Add {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Shifter,
		s:           Flag<'S'> },

	Branch {
		predicate: Predicate,
		immediate: Signed },

	BranchExchange {
		predicate: Predicate,
		register:  Register },

	BranchLink {
		predicate: Predicate,
		immediate: Signed
	},

	Breakpoint {
		immediate: Unsigned },

	Move {
		predicate:   Predicate,
		destination: Register,
		source:      Shifter,
		s:           Flag<'S'> },

	MoveNegated {
		predicate:   Predicate,
		destination: Register,
		source:      Shifter,
		s:           Flag<'S'> },

	SoftwareInterrupt {
		predicate: Predicate,
		immediate: Unsigned },
}
