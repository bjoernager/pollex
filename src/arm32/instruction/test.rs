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

use crate::arm32::{Condition, Instruction};

use alloc::format;

#[test]
fn test_arm32_instruction() {
	let assert_display = |instruction: Instruction, display: &str| {
		assert_eq!(format!("{instruction}"), display);
	};

	assert_display(
		Instruction::BranchLink {
			condition: Condition::HigherOrSame,
			immediate: 0xF,
		},
		"BLHS <#15>",
	);
}
