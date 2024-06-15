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

mod encode_arm;
mod encode_thumb;

use core::num::Wrapping;

/// Codec for encoding and decoding instruction.
///
/// Arm instructions can be encoded/decoded using the [`encode_arm`](InstructionCodec::encode_arm) and `decode_arm` (soon).
/// Thumb instruction will similarly be manipulated using [`encode_thumb`](InstructionCodec::encode_thumb) and `decode_thumb`.
///
/// This structure keeps track of the adress at which instructions are to be placed (see *Rationale*).
/// If encoding causes this internal address to go past `0xFFFFFFFF`, the value is safely wrapped to the origin (i.e. `0x00000000`).
///
/// # Rationale
///
/// This structure is useful for the few functions that encode differently according to their position.
///
/// For example `B 0x08000000` encodes the immediate (in this case `0x08000000`) as an offset from `PC`.
/// This is despite taking an address as its operand in assembly.
#[derive(Clone, Debug)]
pub struct InstructionCodec {
	address: Wrapping<u32>,
}

impl InstructionCodec {
	/// Constructs a new codec with a given starting address.
	#[inline(always)]
	#[must_use]
	pub const fn new_at(address: u32) -> Self {
		Self { address: Wrapping(address) }
	}
}

impl Default for InstructionCodec {
	#[inline(always)]
	fn default() -> Self { Self::new_at(0x00000000) }
}
