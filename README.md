# vimvar-rs: Library to read neovim/vim variables from Rust

[![Build Status][build_img]][build_lnk]
[![Crates.io][crates_img]][crates_lnk]
[![Docs.rs][doc_img]][doc_lnk]

[build_img]: https://github.com/chipsenkbeil/vimvar-rs/workflows/CI/badge.svg
[build_lnk]: https://github.com/chipsenkbeil/vimvar-rs/actions
[crates_img]: https://img.shields.io/crates/v/vimvar.svg
[crates_lnk]: https://crates.io/crates/vimvar
[doc_img]: https://docs.rs/vimvar/badge.svg
[doc_lnk]: https://docs.rs/vimvar

---

### Installation

```toml
# Cargo.toml
[dependencies]
vimvar = "0.1"
```

### Usage

```rust
use vimvar::VimVar;

// Loads g:my_global_var from default vimrc, returning None if it does not
// exist, and casting to type String from the default serde_json::Value
//
// Only fails unwrap if neovim/vim fails to run or type fails to cast
let var: Option<String> = VimVar::load_typed_global_var("my_global_var").unwrap();
println!("Loaded g:my_global_var = {:?}", var);
```

### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in vimvar by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
