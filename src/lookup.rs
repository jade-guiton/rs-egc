/// Enum summarizing the three character properties relevant for EGC segmentation.
/// 
/// - Grapheme_Cluster_Break (GCB)
/// - Indic_Conjunct_Break (InCB)
/// - Extended_Pictographic (ExtPict)
/// 
/// To keep the data tables light, we only have enum variants for the
/// combinations of property values that actually appear in the Unicode Character Database.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum EgcProps {
	/// Default
	XX,
	
	/// GCB=LF  (line feed)
	LF,
	/// GCB=CR  (carriage return)
	CR,
	/// GCB=Control
	CN,
	/// GCB=L   (hangul leading consonant)
	
	L,
	/// GCB=V   (hangul vowel)
	V,
	/// GCB=T   (hangul trailing consonant)
	T,
	/// GCB=LV  (hangul precombined leading + vowel)
	LV,
	/// GCB=LVT (hangul precombined leading + vowel + trailing)
	LVT,

	/// GCB=SpacingMark
	SM,
	/// GCB=Prepend
	PP,
	
	/// InCB=Consonant
	IN_CO,
	/// GCB=ZWJ, InCB=Extend (zero-width joiner)
	ZWJ,
	/// GCB=Extend, InCB=Extend
	IN_EX,
	/// GCB=Extend, InCB=Linker
	IN_LI,
	/// GCB=Extend, InCB=None
	EX,

	/// ExtPict=Yes
	EP,
	
	/// GCB=Regional_Indicator
	RI,
}

impl EgcProps {
	/// Is character a control character (LF, CR, CN)
	pub fn is_control(self) -> bool {
		EgcProps::LF <= self && self <= EgcProps::CN
	}
	/// Is character hangul (L, V, T, LV, LVT)
	pub fn is_hangul(self) -> bool {
		EgcProps::L <= self && self <= EgcProps::LVT
	}

	/// Does the character have InCB=Extend (ZWJ, IN_EX)
	pub fn is_incb_ex(self) -> bool {
		EgcProps::ZWJ <= self && self <= EgcProps::IN_EX
	}
	/// Does the character have GCB=Extend (IN_EX, IN_LI, EX)
	pub fn is_gcb_ex(self) -> bool {
		EgcProps::IN_EX <= self && self <= EgcProps::EX
	}
}

/// A range of Unicode codepoints, and the associated EGC-related properties.
#[derive(Clone, Copy)]
pub struct CharRange {
	pub start: u32,
	pub count: u16,
	pub kind: EgcProps,
}
pub(crate) const fn ran(start: u32, count: u16, kind: EgcProps) -> CharRange {
	CharRange { start, count, kind }
}

/// The main data table.
pub use crate::data::RANGES;

/// Looks up the character properties of `c` that are relevant to EGCs.
pub fn lookup_egc_props(c: char) -> EgcProps {
	let cp = c as u32;
	if (cp >= 0x20 && cp < 0x7f) || (cp >= 0x3300 && cp < 0xa000) {
		// fast path for printable ASCII and CJK characters
		return EgcProps::XX;
	}
	if cp >= 0xac00 && cp <= 0xd7a3 {
		// precomposed hangul makes up most of the data but is very predictable: don't store it
		return if (cp - 0xac00) % 28 == 0 { EgcProps::LV } else { EgcProps::LVT };
	}
	let mut start = 0usize;
	let mut end = RANGES.len();
	while end > start {
		let pivot_idx = (start + end) / 2;
		let pivot = RANGES[pivot_idx];
		if cp < pivot.start {
			end = pivot_idx;
		} else if cp >= pivot.start + pivot.count as u32 {
			start = pivot_idx + 1;
		} else {
			return pivot.kind;
		}
	}
	return EgcProps::XX;
}
