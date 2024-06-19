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

use crate::arm32::{Sflag, Instruction, Shifter};

use core::fmt::Display;

impl Display for Instruction {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use Instruction::*;

		match *self {
			Add {
				predicate,
				destination,
				base,
				source,
				s,
			} => write!(f, "ADD{predicate}{s} {destination}, {base}, {source}"),

			AddCarry {
				predicate,
				destination,
				base,
				source,
				s,
			} => write!(f, "ADC{predicate}{s} {destination}, {base}, {source}"),

			And {
				predicate,
				destination,
				base,
				source,
				s,
			} => write!(f, "AND{predicate}{s} {destination}, {base}, {source}"),

			BitClear {
				predicate,
				destination,
				base,
				source,
				s,
			} => write!(f, "BIC{predicate}{s} {destination}, {base}, {source}"),

			Branch {
				predicate,
				immediate,
			} => write!(f, "B{predicate} #{immediate}"),

			BranchExchange {
				predicate,
				source,
			} => write!(f, "BX{predicate} {source}"),

			BranchLink {
				predicate,
				source,
			} => write!(f, "BL{predicate} #{source}"),

			BranchLinkExchange {
				predicate,
				source,
			} => write!(f, "BLX{predicate} {source}"),

			Breakpoint {
				immediate } => write!(f, "BKPT #{immediate}"),

			CountLeadingZeroes {
				predicate,
				destination,
				source,
			} => write!(f, "CLZ{predicate} {destination}, {source}"),

			Compare {
				predicate,
				lhs,
				rhs,
			} => write!(f, "CMP{predicate} {lhs}, {rhs}"),

			CompareNegated {
				predicate,
				lhs,
				rhs,
			} => write!(f, "CMN{predicate} {lhs}, {rhs}"),

			ExclusiveOr {
				predicate,
				destination,
				base,
				source,
				s,
			} => write!(f, "EOR{predicate}{s} {destination}, {base}, {source}"),

			InclusiveOr {
				predicate,
				destination,
				base,
				source,
				s,
			} => write!(f, "ORR{predicate}{s} {destination}, {base}, {source}"),

			Load {
				predicate,
				register,
				address,
				b,
				t,
			} => write!(f, "LDR{predicate}{b}{t} {register}, {address}"),

			Move {
				predicate,
				destination,
				source:      Shifter::LogicalShiftLeftImmediate { source, shift: 0x0 },
				s:           Sflag::Off,
			} => write!(f, "CPY{predicate} {destination}, {source}"),

			Move {
				predicate,
				destination,
				source:      Shifter::ArithmeticShiftRightImmediate { source, shift },
				s,
			} => write!(f, "ASR{predicate}{s} {destination}, {source}, #{shift}"),

			Move {
				predicate,
				destination,
				source:      Shifter::ArithmeticShiftRightRegister { source, shift },
				s,
			} => write!(f, "ASR{predicate}{s} {destination}, {source}, {shift}"),

			Move {
				predicate,
				destination,
				source:      Shifter::LogicalShiftLeftImmediate { source, shift },
				s,
			} if shift != 0x0 => write!(f, "LSL{predicate}{s} {destination}, {source}, #{shift}"),

			Move {
				predicate,
				destination,
				source:      Shifter::LogicalShiftLeftRegister { source, shift },
				s,
			} => write!(f, "LSL{predicate}{s} {destination}, {source}, {shift}"),

			Move {
				predicate,
				destination,
				source:      Shifter::LogicalShiftRightImmediate { source, shift },
				s,
			} => write!(f, "LSR{predicate}{s} {destination}, {source}, #{shift}"),

			Move {
				predicate,
				destination,
				source:      Shifter::LogicalShiftRightRegister { source, shift },
				s,
			} => write!(f, "LSR{predicate}{s} {destination}, {source}, {shift}"),

			Move {
				predicate,
				destination,
				source:      Shifter::RotateRightImmediate { source, shift },
				s,
			} => write!(f, "ROR{predicate}{s} {destination}, {source}, #{shift}"),

			Move {
				predicate,
				destination,
				source:      Shifter::RotateRightRegister { source, shift },
				s,
			} => write!(f, "ROR{predicate}{s} {destination}, {source}, {shift}"),

			Move {
				predicate,
				destination,
				source,
				s,
			} => write!(f, "MOV{predicate}{s} {destination}, {source}"),

			MoveNot {
				predicate,
				destination,
				source,
				s,
			} => write!(f, "MVN{predicate}{s} {destination}, {source}"),

			Multiply {
				predicate,
				destination,
				base,
				source,
				s,
			} => write!(f, "MUL{predicate}{s} {destination}, {base}, {source}"),

			MultiplyAccumulate {
				predicate,
				destination,
				base,
				source,
				shift,
				s,
			} => write!(f, "MLA{predicate}{s} {destination}, {base}, {source}, {shift}"),

			Reverse {
				predicate,
				destination,
				source,
			} => write!(f, "REV{predicate} {destination}, {source}"),

			ReverseSubtract {
				predicate,
				destination,
				base,
				source: Shifter::Immediate(0x0),
				s,
			} => write!(f, "NEG{predicate}{s} {destination}, {base}"),

			ReverseSubtract {
				predicate,
				destination,
				base,
				source,
				s,
			} => write!(f, "RSB{predicate}{s} {destination}, {base}, {source}"),

			ReverseSubtractCarry {
				predicate,
				destination,
				base,
				source,
				s,
			} => write!(f, "RSC{predicate}{s} {destination}, {base}, {source}"),

			SaturatingAdd {
				predicate,
				destination,
				base,
				source,
			} => write!(f, "QADD{predicate} {destination}, {base}, {source}"),

			SaturatingSubtract {
				predicate,
				destination,
				base,
				source,
			} => write!(f, "QSUB{predicate} {destination}, {base}, {source}"),

			SoftwareInterrupt {
				predicate,
				immediate,
			} => write!(f, "SWI{predicate} #{immediate}"),

			Store {
				predicate,
				register,
				address,
				b,
				t,
			} => write!(f, "STR{predicate}{b}{t} {register}, {address}"),

			Subtract {
				predicate,
				destination,
				base,
				source,
				s,
			} => write!(f, "SUB{predicate}{s} {destination}, {base}, {source}"),

			SubtractCarry {
				predicate,
				destination,
				base,
				source,
				s,
			} => write!(f, "SBC{predicate}{s} {destination}, {base}, {source}"),

			Swap {
				predicate,
				register,
				address,
				b,
			} => write!(f, "SWP{predicate}{b} {register}, {address}"),

			UnsignedSaturate {
				predicate,
				destination,
				immediate,
				source,
			} => write!(f, "USAT{predicate} {destination}, #{immediate}, {source}"),

			Test {
				predicate,
				lhs,
				rhs,
			} => write!(f, "TST{predicate} {lhs}, {rhs}"),

			TestEquivalence {
				predicate,
				lhs,
				rhs,
			} => write!(f, "TEQ{predicate} {lhs}, {rhs}"),
		}
	}
}
