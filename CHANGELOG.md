# Changelog

This is the changelog of Pollex.
See `"README.md"` for more information.

## 0.5.0

* Document `assert_or_err`
* Add docs logo
* Restructure tests
* Bump minor
* Update lints
* Add new errors: `IllegalFlag`, `IllegalShifter`, `InvalidOpcode`, `UnknownMnemonic`
* Update readme
* Update documentation
* Improve diagnostics

### Arm32

* Encode more instructions for Thumb
* Make `InstructionCodec::encode_thumb` return tuple
* Add new instructions: `Test`, `TestEquivalence`, `UnsignedSaturate`, `PreloadData`, `BranchLinkExchange`, `Swap`, `Load`, `Store`
* Rework `Flag`
* Add `Address` type
* Add `as_register` method to `Shifter`
* Add `skip_words`, `skip_halfwords`, and `skib_bytes` methods to `InstructionCodec`
* Add `from_register` method to `Shifter`
* Add Thumb decoding to `InstructionCodec`
* Add `from_u8` method to `Predicate`
* Add `from_u8` method to `Register`
* Implement `FromStr` for `Instruction`
* Add new `Sflag`, `Bflag`, and `Tflag` flags
* Add `seek_to` method to `InstructionCodec`

## 0.4.0

* Bump minor version
* Add logo
* Add more errors
* Add macro `assert_or_err`
* Document errors
* Remove `Arch`

### Arm32

* Add `encode_thumb` method to `InstructionCodec`
* Add `is_low` and `is_high` methods to `Register`
* Implement `FromStr` for `Register`
* Rename `Sb`, `Sl`, and `Ip` in `Register` to `R9`, `R10`, and `R12`
* Display instruction synonyms
* Document `Flag`

### Arm64

* Add module

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
