use crate::logic::{first_boundary, last_local_boundary};

/// A forward iterator over the end indices of EGCs in a string.
#[derive(Clone, Copy)]
pub struct EgcIndices<'a> {
	str: &'a str,
	offset: usize,
}
impl<'a> Iterator for EgcIndices<'a> {
	type Item = usize;
	fn next(&mut self) -> Option<usize> {
		if self.offset == self.str.len() {
			return None;
		}
		self.offset += first_boundary(&self.str[self.offset..]);
		Some(self.offset)
	}
}

/// A forward iterator over EGCs in a string returned as sub-slices.
#[derive(Clone, Copy)]
pub struct EgcSlices<'a> {
	inner: EgcIndices<'a>,
}
impl<'a> Iterator for EgcSlices<'a> {
	type Item = &'a str;
	fn next(&mut self) -> Option<&'a str> {
		let start = self.inner.offset;
		self.inner.next().map(|i| &self.inner.str[start..i])
	}
}

/// A backward iterator over the start indices of EGCs in a string.
/// 
/// Note that this is not as straightforward as forward iteration:
/// 
/// - An initial backwards pass is made, only looking for local EGC boundaries
/// (ie. those which can be determined without prior context).
/// - Then, if we skipped over any possible non-local boundaries that required
/// more context to determine, a forwards pass is made to identify them.
/// - If any boundaries were skipped, we store them in the iterator for later
/// retrieval. This avoids backtracking multiple times, at the cost of memory.
/// 
/// For example, a very long string full of flag emojis will require
/// backtracking all the way to the start to determine the flag boundaries,
/// even for the very last flag. All the flag boundaries will end up being
/// computed in the first call to [next](Iterator::next), but will not be
/// recomputed in later calls.
#[derive(Clone)]
pub struct EgcRevIndices<'a> {
	str: &'a str,
	offset: usize,
	// If we backtracked too much and skipped over some non-local boundaries,
	// we store them in a stack to output later.
	stack: Vec<usize>,
}
impl<'a> Iterator for EgcRevIndices<'a> {
	type Item = usize;
	fn next(&mut self) -> Option<usize> {
		if self.offset == 0 {
			return None;
		}

		if let Some(i) = self.stack.pop() {
			self.offset = i;
			return Some(i);
		}

		let rest = &self.str[..self.offset];
		let (mut i, maybe_skipped) = last_local_boundary(rest);
		if !maybe_skipped {
			self.offset = i;
			return Some(i);
		}

		let mut it = EgcIndices { str: rest, offset: i };
		while let Some(end) = it.next() {
			if end == self.offset {
				self.offset = i;
				return Some(i);
			} else {
				self.stack.push(i);
			}
			i = end;
		}
		unreachable!()
	}
}

/// A backward iterator over EGCs in a string returned as sub-slices.
/// 
/// Same caveats as [EgcRevIndices].
#[derive(Clone)]
pub struct EgcRevSlices<'a> {
	inner: EgcRevIndices<'a>,
}
impl<'a> Iterator for EgcRevSlices<'a> {
	type Item = &'a str;
	fn next(&mut self) -> Option<&'a str> {
		let end = self.inner.offset;
		self.inner.next().map(|i| &self.inner.str[i..end])
	}
}

impl<'a> EgcIndices<'a> {
	/// Returns a backwards iterator over the indices.
	/// 
	/// Note that this is a different iterator type, and thus
	/// cannot be an implementation of [DoubleEndedIterator].
	/// 
	/// Moreover, the backward iterator returns the _start_
	/// indices of the EGCs, rather than the _end_ indices.
	pub fn rev(self) -> EgcRevIndices<'a> {
		let rest = &self.str[self.offset..];
		EgcRevIndices {
			str: rest,
			offset: rest.len(),
			stack: vec![],
		}
	}
}

impl<'a> EgcSlices<'a> {
	/// Returns a backwards iterator over the EGC slices.
	/// 
	/// Note that this is a different iterator type, and thus
	/// cannot be an implementation of [DoubleEndedIterator].
	pub fn rev(self) -> EgcRevSlices<'a> {
		EgcRevSlices { inner: self.inner.rev() }
	}
}

/// An extension trait which adds EGC-related methods to [str].
pub trait Egc {
	/// Returns an iterator over the indices of the
	/// extended grapheme clusters (EGCs) in the string.
	/// The offset at the _end_ of the EGC is returned,
	/// unlike `char_indices`.
	/// 
	/// A backwards iterator can be obtained with [rev](EgcIndices::rev),
	/// but because it is a different iterator type, [EgcIndices] does
	/// not implement [DoubleEndedIterator].
	fn egc_indices(&self) -> EgcIndices;

	/// Returns an iterator over the extended grapheme clusters
	/// (EGC) in the string, returned as sub-slices.
	/// 
	/// A backwards iterator can be obtained with [rev](EgcSlices::rev),
	/// but because it is a different type, [EgcSlices] does
	/// not implement [DoubleEndedIterator].
	fn egcs(&self) -> EgcSlices;
}

impl Egc for str {
	fn egc_indices(&self) -> EgcIndices {
		EgcIndices { str: self, offset: 0 }
	}
	fn egcs(&self) -> EgcSlices {
		EgcSlices { inner: self.egc_indices() }
	}
}
