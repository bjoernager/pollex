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
	Instruction,
	InstructionCodec,
	Predicate,
	Register,
	Sflag,
	Shifter,
};

use alloc::vec::Vec;

#[test]
fn test_thumb_encode() {
	let programme = [
		Instruction::BranchLink {
			predicate: Predicate::Always,
			source:    0x08000044,
		},

		Instruction::And {
			predicate:   Predicate::Always,
			destination: Register::R0,
			base:        Register::R0,
			source:      Shifter::LogicalShiftLeftImmediate { source: Register::R7, shift: 0x0 },
			s:           Sflag::On,
		},

		Instruction::Branch {
			predicate: Predicate::Always,
			immediate: 0x08000008,
		},
	];

	let mut codec = InstructionCodec::new_at(0x08000000);

	let mut opcodes = Vec::new();
	for instruction in programme {
		let opcode = codec.encode_thumb(instruction).unwrap();

		opcodes.push(opcode.0);
		if let Some(opcode) = opcode.1 { opcodes.push(opcode) };
	}

	assert_eq!(
		opcodes,
		[
			0b11110000_00000000,
			0b11111000_00100000,
			0b01000000_00111000,
			0b11100000_00000000,
		],
	)
}
