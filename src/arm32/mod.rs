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

//! Arm32-related facilities.
//!
//! This includes T variants of Arm32.

use crate::use_mod;
use_mod!(pub arm_opcode);
use_mod!(pub predicate);
use_mod!(pub flag);
use_mod!(pub instruction);
use_mod!(pub register);
use_mod!(pub shifter);
use_mod!(pub signed);
use_mod!(pub thumb_opcode);
use_mod!(pub unsigned);
