# Pollex

[Pollex](https://crates.io/crates/pollex/) is a Rust-written library for manipulating instructions of Arm ISAs.

See [Docs.rs](https://docs.rs/pollex/) for documentation.

## Support

Pollex supports encoding of instructions to both Arm and Thumb on Arm32 targets.
Arm64 support is planned.

## Usage

Instructions can be created directly using the `Instruction` type:

```rs
use pollex::arm32::{
    Instruction,
    Predicate,
    Register,
    Sflag,
    Shifter,
};

// MOVS r0, r1
let instr = Instruction::Move {
    predicate:   Predicate::Always,
    destination: Register::R0,
    source:      Shifter::from_register(Register::R1),
    s:           Sflag::On,
};
```

Instructions can also be parsed from strings:

```rs
use pollex::arm32::{
    Instruction,
    Predicate,
    Register,
    Sflag,
    Shifter,
};

let instr: Instruction = "CPY r0, r1".parse()?;

// Is equivalent to:

let instr = Instruction::Move {
    predicate:   Predicate::Always,
    destination: Register::R0,
    source:      Shifter::from_register(Register::R1),
    s:           Sflag::Off,
};

# Ok::<(), Box<dyn std::error::Error>>(())
```

But do note that the latter is currently **not** implemented.

Instructions can be encoded to both Arm and Thumb using the `InstructionCodec` type:

```rs
use pollex::arm32::{Instruction, InstructionCodec};

let instr: Instruction = "BX lr".parse()?;

let mut codec = InstructionCodec::new();

let arm_opcode   = codec.encode_arm(instr)?;
let thumb_opcode = codec.encode_thumb(instr)?;

assert_eq!(arm_opcode, 0b11100001_00101111_11111111_00011110);
assert_eq!(thumb_opcode.0, 0b01000111_01110000);
assert_eq!(thumb_opcode.1, None);

# Ok::<(), Box<dyn std::error::Error>>(())
```

## Copyright & Licensing

Copyright 2024 Gabriel Bjørnager Jensen.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

## Trademarks

Arm and Thumb are registered trademarks or trademarks of Arm Limited (or its subsidiaries or affiliates).
