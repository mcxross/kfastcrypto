<h1 align="center">Kfastcrypto - Kotlin MPP bindings for Fastcrypto</h1>

Kfastcrypto (short for Kotlin for Fastcrypto) is a cryptography library exposing a significant part of Fastcrypto library APIs to Kotlin MPP.

![Platform](https://img.shields.io/badge/platform-Android%20|%20JVM%20|%20JS%20|%20Native-blue.svg)
![Architecture](https://img.shields.io/badge/architecture-amd64%20|%20aarch64-blue.svg)
[![Rust](https://github.com/mcxross/kfastcrypto/actions/workflows/rust.yml/badge.svg)](https://github.com/mcxross/kfastcrypto/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

# Table of contents
- [What's included](#whats-included)
- [Contribution](#contribution)


## What's included

In any FFI-related project, it is natural to have code written in both languages that need to communicate with each other.

| File/Folder      | Description    |
|------------------|----------------|
| [kotlin](kotlin) | Kotlin related |
| [rust](rust)     | Rust related   |


## Contribution

All contributions to Kfastcrypto are welcome. Before opening a PR, please submit an issue detailing the bug or feature. When opening a PR, please ensure that your contribution builds on the KMM and cargo toolchain, has been linted with `ktfmt <GOOGLE (INTERNAL)>` for `kotlin`, `cargo fmt` for `rust`, and contains tests when applicable. For more information, please see the [contribution guidelines](CONTRIBUTING.md).