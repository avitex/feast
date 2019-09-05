[![Build Status](https://travis-ci.org/avitex/feast.svg)](https://travis-ci.org/avitex/feast)
[![Feast crate](https://img.shields.io/crates/v/feast.svg)](https://crates.io/crates/feast)
[![Docs](https://docs.rs/feast/badge.svg)](https://docs.rs/feast)

# Feast

**[EXPERIMENTAL] Rust parsing library with a focus on bytes.**


| variant  | input  | capture       |
| -------- | ------ | ------------- |
| `item`   | loaded | determinate   |
| `[item]` | loaded | determinate   |
| `item`   | stream | determinate   |
| `[item]` | stream | undeterminate |
|