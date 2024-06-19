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

mod decode_thumb;
mod encode_arm;
mod encode_thumb;

use core::num::Wrapping;

/// Codec for encoding and decoding instruction.
///
/// Arm instructions can be encoded/decoded using the [`encode_arm`](InstructionCodec::encode_arm) and (soon) `decode_arm`.
/// Thumb instruction will similarly be manipulated using [`encode_thumb`](InstructionCodec::encode_thumb) and [`decode_thumb`](InstructionCodec::decode_thumb).
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
	/// Constructs a new codec at the origin.
	#[inline(always)]
	#[must_use]
	pub const fn new() -> Self { Self::new_at(0x00000000) }

	/// Constructs a new codec with a given starting address.
	#[inline(always)]
	#[must_use]
	pub const fn new_at(address: u32) -> Self {
		Self { address: Wrapping(address) }
	}

	/// Sets the internal address to the provided one.
	#[inline(always)]
	pub fn seek_to(&mut self, address: u32) { self.address = Wrapping(address) }

	/// Skips the given ammount of bytes.
	#[inline(always)]
	pub fn skip_bytes(&mut self, count: u32) { self.address += Wrapping(count) }

	/// Skips the given ammount of halfwords.
	#[inline(always)]
	pub fn skip_halfwords(&mut self, count: u32) { self.address += Wrapping(count) * Wrapping(0x2) }

	/// Skips the given ammount of words.
	#[inline(always)]
	pub fn skip_words(&mut self, count: u32) { self.address += Wrapping(count) * Wrapping(0x4) }
}

impl Default for InstructionCodec {
	#[inline(always)]
	fn default() -> Self { Self::new() }
}
