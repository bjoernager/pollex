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

use crate::arm32::{
	Predicate,
	Sflag,
	Instruction,
	Register,
	Shifter,
};

use alloc::string::ToString;
use alloc::vec::Vec;

#[test]
fn test_instruction_display() {
	let tree = [
		Instruction::Add {
			predicate:   Predicate::GreaterThanOrEqual,
			destination: Register::R1,
			base:        Register::R2,
			source:      Shifter::RotateRightImmediate { source: Register::R3, shift: 0x2 },
			s:           Sflag::Off,
		},

		Instruction::SaturatingSubtract {
			predicate:   Predicate::LessThan,
			destination: Register::R4,
			base:        Register::R5,
			source:      Register::R6,
		},

		Instruction::InclusiveOr {
			predicate:   Predicate::Always,
			destination: Register::R7,
			base:        Register::R8,
			source:      Shifter::LogicalShiftLeftImmediate { source: Register::R9, shift: 0x0 },
			s:           Sflag::On,
		},

		Instruction::MultiplyAccumulate {
			predicate:   Predicate::Equal,
			destination: Register::R0,
			base:        Register::Pc,
			source:      Register::Pc,
			shift:       Register::Lr,
			s:           Sflag::Off,
		},

		Instruction::Move {
			predicate:   Predicate::NotEqual,
			destination: Register::R0,
			source:      Shifter::LogicalShiftLeftImmediate { source: Register::Pc, shift: 0x0 },
			s:           Sflag::Off,
		},

		Instruction::ReverseSubtract {
			predicate:   Predicate::Always,
			destination: Register::R0,
			base:        Register::R0,
			source:      Shifter::Immediate(0x0),
			s:           Sflag::On,
		},

		Instruction::Move {
			predicate:   Predicate::GreaterThan,
			destination: Register::R0,
			source:      Shifter::LogicalShiftRightImmediate { source: Register::R7, shift: 0x20 },
			s:           Sflag::On,
		},

		Instruction::Move {
			predicate:   Predicate::Always,
			destination: Register::R0,
			source:      Shifter::LogicalShiftLeftImmediate { source: Register::R0, shift: 0x0 },
			s:           Sflag::On,
		},
	];

	let mut displays = Vec::with_capacity(tree.len());
 	for instruction in tree { displays.push(instruction.to_string()) }

	assert_eq!(
		displays,
		[
			"ADDGE r1, r2, r3, ROR #2",
			"QSUBLT r4, r5, r6",
			"ORRS r7, r8, r9",
			"MLAEQ r0, pc, pc, lr",
			"CPYNE r0, pc",
			"NEGS r0, r0",
			"LSRGTS r0, r7, #32",
			"MOVS r0, r0",
		],
	);
}
