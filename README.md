# rs-egc

A simple Rust crate for iterating forward and backward over the extended grapheme clusters (EGC) of a string,
as specified in [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/).

This crate should be up-to-date with Unicode 15.1.0.

This was meant as an exercice, and is not necessarily more efficient, complete, or ergonomic
than existing crates providing similar functionality. However, it should at least be accurate.

- `cargo build` to build the library.
- `cargo doc` to build documentation.
- `cargo test` to run tests extracted from the `GraphemeBreakTest.txt` file in the Unicode Character Database (UCD).
- `python3 scripts/download_egc_data.py` to download up-to-date character data from the UCD and regenerate `src/data.rs`.
- `python3 scripts/download_test_data.py` to download up-to-date test data from the UCD and regenerate `src/test_data.rs`.
