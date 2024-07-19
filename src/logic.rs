use crate::lookup::{lookup_egc_props, EgcProps};
use EgcProps as EP;

/// Context for EGC segmentation.
/// 
/// Implements a state machine which recognizes the patterns
/// necessary to apply the full segmentation rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ctx {
	/// At EGC boundary (including start of string)
	Start,
	/// State machine for rule GB9c
	Indic(bool),
	/// State machine for rule GB11
	Emoji(bool),
	/// State machine for rules GB12/GB13
	Ri,
}

impl Ctx {
	/// Computes the new context after a character with the given EGC properties.
	pub fn step(self, p: EgcProps) -> Ctx {
		// GB9c
		if p == EP::IN_CO {
			return Ctx::Indic(false);
		}
		if let Ctx::Indic(s) = self {
			if p == EP::IN_LI {
				return Ctx::Indic(true);
			} else if p.is_incb_ex() {
				return Ctx::Indic(s);
			}
		}

		// GB11
		if p == EP::EP {
			return Ctx::Emoji(false);
		}
		if let Ctx::Emoji(s) = self {
			match (s, p) {
				(false, _) if p.is_gcb_ex() => return Ctx::Emoji(false),
				(false, EP::ZWJ) => return Ctx::Emoji(true),
				_ => (),
			}
		}

		// GB12/13
		if p == EP::RI {
			return if self == Ctx::Ri { Ctx::Start } else { Ctx::Ri };
		}

		Ctx::Start
	}
}

/// Computes whether an EGC boundary exists between characters with
/// properties `p1` and `p2`, with no further context.
/// 
/// If context is needed to decide, returns None.
pub fn is_local_boundary(p1: EP, p2: EP) -> Option<bool> {
	if p1 == EP::XX && p2 == EP::XX { // Fast path
		return Some(true);
	}
	if p1 == EP::CR && p2 == EP::LF { // GB3
		return Some(false);
	}
	if p1.is_control() || p2.is_control() { // GB4/5
		return Some(true);
	}
	if p1.is_hangul() && p2.is_hangul() {
		let merge =
			(p1 == EP::L && p2 != EP::T) || // GB6
			((p1 == EP::LV || p1 == EP::V) && (p2 == EP::V || p2 == EP::T)) || // GB7
			((p1 == EP::LVT || p1 == EP::T) && p2 == EP::T); // GB8
		return if merge { Some(false) } else { Some(true) };
	}
	if p2.is_gcb_ex() || p2 == EP::ZWJ { // GB9
		return Some(false);
	}

	// Extended rules:
	if p2 == EP::SM || p1 == EP::PP { // GB9a/b
		return Some(false);
	}
	if (p1.is_incb_ex() || p1 == EP::IN_LI) && p2 == EP::IN_CO { // GB9c
		return None;
	}
	
	if p1 == EP::ZWJ && p2 == EP::EP { // GB11
		return None;
	}
	if p1 == EP::RI && p2 == EP::RI { // GB12/13
		return None;
	}
	return Some(true);
}

/// Computes whether an EGC boundary exists between characters with
/// properties `p1` and `p2`, given the context `c` *up to and including* the first character.
pub fn is_boundary(c: Ctx, p1: EP, p2: EP) -> bool {
	if let Some(boundary) = is_local_boundary(p1, p2) {
		return boundary;
	}
	return !(
		(c == Ctx::Indic(true) && p2 == EP::IN_CO) || // GB9c
		(c == Ctx::Emoji(true) && p2 == EP::EP) || // GB11
		(c == Ctx::Ri && p2 == EP::RI) // GB12/13
	);
}

/// Returns the offset of the first EGC boundary in the string,
/// ie. the length of the first EGC.
/// 
/// If the string is empty, returns 0.
pub fn first_boundary(s: &str) -> usize {
	let mut ctx = Ctx::Start;
	let mut p1 = None;
	for (i, c) in s.char_indices() {
		let p2 = lookup_egc_props(c);
		if let Some(p1) = p1 {
			if is_boundary(ctx, p1, p2) {
				return i;
			}
		}
		ctx = ctx.step(p2);
		p1 = Some(p2);
	}
	s.len()
}

/// Returns the offset of the last local EGC boundary in the string,
/// and a flag indicating whether a non-local boundary may have been
/// skipped over.
/// 
/// (A local boundary is one which can be decided by only looking at
/// the two characters surrounding it, with no further context.)
pub fn last_local_boundary(s: &str) -> (usize, bool) {
	let mut p2 = None;
	let mut maybe_skipped = false;
	for (i, c) in s.char_indices().rev() {
		let p1 = lookup_egc_props(c);
		if let Some(p2) = p2 {
			if let Some(boundary) = is_local_boundary(p1, p2) {
				if boundary {
					return (i + c.len_utf8(), maybe_skipped);
				}
			} else {
				maybe_skipped = true;
			}
		}
		p2 = Some(p1);
	}
	(0, maybe_skipped)
}
