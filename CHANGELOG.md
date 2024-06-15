# Changelog

This is the changelog of Pollex.
See `"README.md"` for more information.

## 0.3.0

* Bump minor
* Add `arm32::InstructionCodec`
* Remove `arm32::Unsigned` and `arm32::Signed`
* Add `SIZE` constant to `arm32::ArmOpcode` and `arm32::ThumbOpcode`
* Implement `From<u32>` and `Into<u32>` for `ArmOpcode`
* Implement `From<u16>` and `Into<u16>` for `ThumbOpcode`
* Update documentation
* Add new Arm32 instructions: `AddCarry`, `And`, `BitClear`, `CountLeadingZeroes`, `CompareNegated` `Compare`, `ExclusiveOr`, `MultiplyAccumulate`, `Multiply`, `InclusiveOr`, `SaturatingAdd`, `SaturatingSubtract`, `Reverse`, `ReverseSubtract`, `ReverseSubtractCarry`, `Subtract`, `SubtractCarry`,
* Rename `MoveNegated` in `arm32::Instruction` to `MoveNot`
* Implement `Into<bool>`, `Into<u8>`, `Into<u16>`, `Into<u32>`, `Into<u64>`, `Into<u128>`, and `Into<usize>` for `Flag`
* Implement `From<bool>` for `Flag`

## 0.2.0

* Bump minor
* Update copyright years
* Add new Arm32 instructions: `SoftwareInterrupt`, `Move`, `BranchExchange`, `Breakpoint`, `MoveNegated`, `Add`
* Derive more
* Add `arm32::Flag`, `arm32::Unsigned`, and `arm32::Signed` types
* Add instruction encoder (currently only for Arm)
* Add `arm32::ArmOpcode` and `arm32::ThumbOpcode` types
* Rename `arm32::Address` to `arm32::Shifter`
* Add `Error` type
* Rework instructions
* Update readme
* Rename `arm32::Condition` to `arm32::Predicate`
* Update documentation

## 0.1.2

* Update project description
* Add documentation

## 0.1.1

* Set documentation link
* Update homepage
* Update readme

## 0.1.0

* Bump minor
* Add `arm32` module
* Add `arm32::Address` type
* Add `arm32::Condition` type
* Add new ARM32 instructions: `BranchLink`
* Add tests
* Use `alloc`
* Add `Arch` type

## 0.0.0

* Fork from Luma and eAS
* Add gitignore
* Add changelog
* Add readme
* License under AGPL
* Add `Instruction` type
* Add `Register` type
