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
};

use alloc::vec::Vec;

#[test]
fn test_thumb_decode() {
	let binary = [
		0b11011111_10101010,
		0b01000111_01110000,
	];

	let mut codec = InstructionCodec::new();

	let mut programme = Vec::new();
	for opcode in binary { programme.push(codec.decode_thumb(opcode.into()).unwrap()) }

	assert_eq!(
		programme,
		[
			Instruction::SoftwareInterrupt {
				predicate: Predicate::Always,
				immediate: 0b10101010,
			},

			Instruction::BranchExchange {
				predicate: Predicate::Always,
				source:    Register::Lr,
			},
		],
	)
}
