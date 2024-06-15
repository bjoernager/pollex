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
	Flag,
	Instruction,
	Register,
	Shifter,
};

use alloc::format;
use alloc::vec::Vec;

#[test]
fn test_arm32_instruction() {
	let tree = [
		Instruction::Add {
			predicate:   Predicate::GreaterThanOrEqual,
			destination: Register::R1,
			base:        Register::R2,
			source:      Shifter::RotateRightImmediate { source: Register::R3, shift: 0x2 },
			s:           Flag::Off,
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
			s:           Flag::On,
		},

		Instruction::MultiplyAccumulate {
			predicate:   Predicate::Equal,
			destination: Register::R0,
			base:        Register::Pc,
			source:      Register::Pc,
			shift:       Register::Lr,
			s:           Flag::Off,
		},

		Instruction::Move {
			predicate:   Predicate::NotEqual,
			destination: Register::R0,
			source:      Shifter::LogicalShiftLeftImmediate { source: Register::Pc, shift: 0x0 },
			s:           Flag::Off,
		},
	];

	let mut displays = Vec::with_capacity(tree.len());
 	for instruction in tree { displays.push(format!("{instruction}")) }

	assert_eq!(
		displays,
		[
			"ADDGE r1, r2, r3, ROR #2",
			"QSUBLT r4, r5, r6",
			"ORRS r7, r8, r9",
			"MLAEQ r0, pc, pc, lr",
			"CPYNE r0, pc"
		],
	);
}
