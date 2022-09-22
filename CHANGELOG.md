# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2022-09-22

### Added

- Dependencies are partially audited and documented with [cargo-vet]

[cargo-vet]: https://mozilla.github.io/cargo-vet/index.html

### Changed

- Removed cog.toml and adopted Keep a Changelog format.
- Removed unmaintained dependency wee_alloc. Default allocator will be used.
- Added shadow-cljs example

## [0.2.2] - 2021-08-29

### Added

- 40888e add an example with parcel, htm, and preact - Kan-Ru Chen

### Changed

- aae08f optimize new wasm-pack output - Kan-Ru Chen
- 781a44 fix cog pre-bump hook - Kan-Ru Chen
- 622881 update dependencies - Kan-Ru Chen
- f27933 update dependencies - Kan-Ru Chen

### Fixed

- a5c93e use correct return type in type declaration - Kan-Ru Chen

## [0.2.1] - 2021-02-10

### Changed

- 2eb37c minimal CI with github action (#1) - Kan-Ru Chen
- 6ac451 enable panic_immedate_abort when using nightly to get smallest wasm - Kan-Ru Chen
- 21f888 build rlib so we can run tests - Kan-Ru Chen

## [0.2.0] - 2021-02-10

### Added

- d7489f use rollup to bundle generated files - Kan-Ru Chen

### Removed

- 872ead remove unused keygen_from_random_bytes function - Kan-Ru Chen

### Changed

- 014e68 add pre_bump_hooks to cog.toml - Kan-Ru Chen
- 5e7595 update wasm-bindgen - Kan-Ru Chen

## [0.1.4] - 2021-02-03

### Changed

- 95874f switch to rage from git to enable faster passphrase encryption - Kan-Ru Chen
- e82b2f add initial cog.toml for cocogitto - Kan-Ru Chen


[Unreleased]: https://github.com/kanru/rage-wasm/compare/v0.3.0..main
[0.3.0]: https://github.com/kanru/rage-wasm/releases/v0.3.0
[0.2.2]: https://github.com/kanru/rage-wasm/releases/tag/0.2.2
[0.2.1]: https://github.com/kanru/rage-wasm/releases/tag/0.2.1
[0.2.0]: https://github.com/kanru/rage-wasm/releases/tag/0.2.0
[0.1.4]: https://github.com/kanru/rage-wasm/releases/tag/0.1.4