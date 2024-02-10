# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1](https://github.com/zusi/zusi-rs/compare/zusi-protocol-v0.2.0...zusi-protocol-v0.2.1) - 2024-02-10

### Other
- shadow derive package

## [0.2.0](https://github.com/zusi/zusi-rs/compare/zusi-protocol-v0.1.2...zusi-protocol-v0.2.0) - 2024-02-10

### Fixed
- *(deps)* update rust crate either to 1.10

### Other
- promote parser to directory
- refactor number serialization
- Zusi Protocol Parser refactor

## [0.1.2](https://github.com/zusi/zusi-rs/compare/zusi-protocol-v0.1.1...zusi-protocol-v0.1.2) - 2024-02-06

### Other
- cleanup features

## [0.1.1](https://github.com/zusi/zusi-rs/compare/zusi-protocol-v0.1.0...zusi-protocol-v0.1.1) - 2024-02-06

### Other
- updated the following local packages: zusi-protocol-derive

## [0.1.0](https://github.com/zusi/zusi-rs/releases/tag/zusi-protocol-v0.1.0) - 2023-12-02

### Other
- update authors
- reimplement codec
- zwischenstand
- zusi-async crate ([#17](https://github.com/zusi/zusi-rs/pull/17))
- migrate to rust 2021
- remove nightly clippy warnings
- test wrong length for field reported
- error if receive_unknown_msg is called with struct end
- use own error type
- handle unknown structs and fields gracefully
- update cargo.tomls to have readmes
- add doc
- document deserialize vec<t>
- refactor serialize number
- remove unused import
- use exact length buffer
- refactor serialize for String
- remove serialize_struct method
- add repository
- rename directorys to match crate name
