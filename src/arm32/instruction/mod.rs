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

mod display;
mod from_str;

use crate::arm32::{
	Address,
	Bflag,
	Predicate,
	Sflag,
	Register,
	Shifter,
	Tflag
};

/// An Arm32 instruction.
///
/// An instruction must be encoded before it can be used by a processor.
/// This can be done using the [`InstructionCodec`](crate::arm32::InstructionCodec) type.
///
/// Do note that these enumerations do not exactly match instructions used in assembly.
/// For example, the following two lines are completely identical (with respect to the final binary, disregarding optimisations):
///
/// ```as
/// CPY r1, r0
/// MOV r1, r0
/// ```
///
/// Yet only `MOV` ([`Move`](Instruction::Move)) is provided as a variant.
/// Similarly, some combinations of operands yield the same results as multiple instructions.
/// See [`Shifter`] for more information.
///
/// Also note that not all operands can be encoded in Arm instruction sets.
/// Even the largest immediates usually have a limit at `24` significant figures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
	Add {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Shifter,
		s:           Sflag,
	},

	AddCarry {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Shifter,
		s:           Sflag,
	},

	And {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Shifter,
		s:           Sflag,
	},

	BitClear {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Shifter,
		s:           Sflag,
	},

	Branch {
		predicate: Predicate,
		immediate: u32,
	},

	BranchExchange {
		predicate: Predicate,
		source:    Register,
	},

	BranchLink {
		predicate: Predicate,
		source:    u32,
	},

	BranchLinkExchange {
		predicate: Predicate,
		source:    Shifter,
	},

	Breakpoint {
		immediate: u32,
	},

	CountLeadingZeroes {
		predicate:   Predicate,
		destination: Register,
		source:      Register,
	},

	Compare {
		predicate: Predicate,
		lhs:       Register,
		rhs:       Shifter,
	},

	CompareNegated {
		predicate: Predicate,
		lhs:       Register,
		rhs:       Shifter,
	},

	ExclusiveOr {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Shifter,
		s:           Sflag,
	},

	InclusiveOr {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Shifter,
		s:           Sflag,
	},

	Load {
		predicate: Predicate,
		register:  Register,
		address:   Address,
		b:         Bflag,
		t:         Tflag,
	},

	Move {
		predicate:   Predicate,
		destination: Register,
		source:      Shifter,
		s:           Sflag,
	},

	MoveNot {
		predicate:   Predicate,
		destination: Register,
		source:      Shifter,
		s:           Sflag,
	},

	Multiply {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Register,
		s:           Sflag,
	},

	MultiplyAccumulate {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Register,
		shift:       Register,
		s:           Sflag,
	},

	Reverse {
		predicate:   Predicate,
		destination: Register,
		source:      Register,
	},

	ReverseSubtract {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Shifter,
		s:           Sflag,
	},

	ReverseSubtractCarry {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Shifter,
		s:           Sflag,
	},

	SaturatingAdd {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Register,
	},

	SaturatingSubtract {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Register,
	},

	SoftwareInterrupt {
		predicate: Predicate,
		immediate: u32,
	},

	Store {
		predicate: Predicate,
		register:  Register,
		address:   Address,
		b:         Bflag,
		t:         Tflag,
	},

	Subtract {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Shifter,
		s:           Sflag,
	},

	SubtractCarry {
		predicate:   Predicate,
		destination: Register,
		base:        Register,
		source:      Shifter,
		s:           Sflag,
	},

	Swap {
		predicate: Predicate,
		register:  Register,
		address:   Address,
		b:         Bflag,
	},

	UnsignedSaturate {
		predicate:   Predicate,
		destination: Register,
		immediate:   u32,
		source:      Shifter,
	},

	Test {
		predicate: Predicate,
		lhs:       Register,
		rhs:       Shifter,
	},

	TestEquivalence {
		predicate: Predicate,
		lhs:       Register,
		rhs:       Shifter,
	},
}
