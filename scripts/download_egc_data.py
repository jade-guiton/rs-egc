from download_ucd import download_ucd_data

OUTPUT_PATH = "src/data.rs"

GCB_DEPR = ["EB","EBG","EM","GAZ"]
ENUM_MAP = {
	("XX", "None", "Y"): "EP",
	("XX", "Consonant", "N"): "IN_CO",
	("EX", "Extend", "N"): "IN_EX",
	("EX", "Linker", "N"): "IN_LI",
	("ZWJ", "Extend", "N"): "ZWJ",
}
for gcb in ["XX", "LF","CR","CN", "L","V","T","LV","LVT", "SM","PP","EX","RI"]:
	ENUM_MAP[(gcb, "None", "N")] = gcb

def get_egc_enum(attrib: dict[str, str], start: int, end: int) -> str:
	gcb, incb, ext_pict = attrib["GCB"], attrib["InCB"], attrib["ExtPict"]
	if gcb in GCB_DEPR: gcb = "XX"
	if (gcb, incb, ext_pict) not in ENUM_MAP:
		print(f"Unexpected combination: U+{start:06x} to U+{end:06x} has GCB={gcb}, InCB={incb}, ExtPict={ext_pict}")
		exit(1)
	val = ENUM_MAP[(gcb, incb, ext_pict)]
	if val == "XX": return None
	return val


ranges = download_ucd_data(get_egc_enum)

# remove precomposed hangul: handled in software to save space
ranges = [r for r in ranges if r.end < 0xac00 or r.start > 0xd7af]

print(f"Writing {OUTPUT_PATH}")
f = open(OUTPUT_PATH, "w")
f.write(f"""\
use crate::lookup::{{EgcProps as P, CharRange, ran}};
pub const RANGES: [CharRange; {len(ranges)}] = [
{
	"".join(f"\tran(0x{r.start:05x}, {r.end-r.start+1: >4}, P::{r.val}),\n" for r in ranges)
}];
""")
f.flush()

print("Done.")
