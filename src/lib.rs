/*!
A simple crate for performing extended grapheme cluster (EGC) segmentation,
as specified in [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/).

This allows identifying (to a close approximation) what users will perceive as
characters if a string is rendered, better than by using Unicode scalar values
(which is what Rust calls a `char`acter).

This crate allows iterating forward and backward through the EGCs of a string.
Note that iterating backward is less efficient, and can (in principle at least)
have problematic time complexity.

Example of use in a simple CLI utility which prints out the codepoints making up
each grapheme in the entered line of text:
```
use std::io::Write;

use egc::Egc;

fn main() {
	let mut line = String::new();
	print!("> ");
	std::io::stdout().flush().unwrap();
	std::io::stdin().read_line(&mut line).unwrap();
	println!("Graphemes:");
	for egc in line.egcs() {
		print!("-");
		for c in egc.chars() {
			print!(" U+{:04x}", c as u32);
		}
		println!()
	}
}
```
*/

pub(crate) mod data;
/// Lookup of basic EGC-related data.
pub mod lookup;
/// Logic for determining EGC boundaries.
pub mod logic;
/// Iterators over EGCs.
pub mod iter;

pub use iter::Egc;

/// Unicode version this library is up-to-date with (major, minor, patch)
pub const UNICODE_VERSION: (u8,u8,u8) = (15, 1, 0);

#[cfg(test)]
mod test_data;

#[cfg(test)]
mod tests {
	pub struct TestCase {
		line: u32,
		str: &'static str,
		breaks: &'static [usize],
	}
	impl TestCase {
		pub const fn new(line: u32, str: &'static str, breaks: &'static [usize]) -> Self {
			TestCase { line, str, breaks }
		}
	}

  use crate::{test_data::TEST_CASES, Egc};

	fn check_breaks(breaks: &[usize], case: &TestCase) {
		if breaks != case.breaks {
			println!("Incorrect grapheme boundaries:");
			println!("Exp: {:?}", case.breaks);
			println!("Got: {:?}", breaks);
			panic!();
		}
	}

	#[test]
	fn ucd_tests() {
		for case in TEST_CASES {
			print!("Line {}:", case.line);
			for c in case.str.chars() {
				print!(" U+{:04x}", c as u32);
			}
			println!();

			let mut breaks: Vec<usize> = case.str.egc_indices().collect();
			let last = breaks.pop().expect("expected at least one grapheme");
			assert_eq!(last, case.str.len(), "last grapheme should end at .len()");
			check_breaks(&breaks, &case);

			let mut breaks: Vec<usize> = case.str.egc_indices().rev().collect();
			let last = breaks.pop().expect("expected at least one grapheme");
			assert_eq!(last, 0, "last grapheme in reverse should start at 0");
			breaks.reverse();
			check_breaks(&breaks, &case);
			
			println!();
		}
	}
}