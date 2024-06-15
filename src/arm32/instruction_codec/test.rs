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
	Flag,
	Instruction,
	InstructionCodec,
	Predicate,
	Register,
	Shifter,
};

use alloc::vec::Vec;

#[test]
fn test_arm32_encoder() {
	let tree = [
		Instruction::BranchLink {
			predicate: Predicate::HigherOrSame,
			immediate: 0x1F,
		},

		Instruction::Breakpoint {
			immediate: 0x45,
		},

		Instruction::SoftwareInterrupt {
			predicate: Predicate::Always,
			immediate: 0x54,
		},

		Instruction::Move {
			predicate:   Predicate::Plus,
			destination: Register::Pc,
			source:      Shifter::ArithmeticShiftRightImmediate { source: Register::R3, shift: 0x20 },
			s:           Flag::On,
		},
	];

	let mut codec = InstructionCodec::new_at(0x08000000);

	let mut opcodes = Vec::new();
	for instruction in tree { opcodes.push(codec.encode_arm(instruction).unwrap()) }

	assert_eq!(
		opcodes,
		[
			0b00101010_00000000_00000000_00000000,
			0b11100001_00100000_00000100_01110101,
			0b11101111_00000000_00000000_01010100,
			0b01010001_10110000_11110000_01000011,
		],
	)
}
