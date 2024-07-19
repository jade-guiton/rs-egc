import os
import urllib.request

from url import get_ucd_url

OUTPUT_PATH = "src/test_data.rs"

test_url = get_ucd_url() + "/ucd/auxiliary/GraphemeBreakTest.txt"
print(f"Fetching {test_url}")
test_path, _ = urllib.request.urlretrieve(test_url)

print("Parsing")
test_cases = []
with open(test_path) as test_file:
	for line_idx, line in enumerate(test_file):
		if "#" in line:
			line = line[:line.index("#")]
		line = line.strip()
		if line == "": continue
		assert line.startswith("÷ ") and line.endswith(" ÷")
		line = line[2:-2]
		test_str = bytearray()
		breaks = []
		for part in line.split():
			if part == "×":
				continue
			elif part == "÷":
				breaks.append(len(test_str))
			else:
				test_str += chr(int(part, 16)).encode("utf-8")
		test_cases.append((line_idx+1, test_str.decode("utf-8"), breaks))

os.remove(test_path)

print(f"Writing {OUTPUT_PATH}")

def escape_str(s):
	return "".join(
		f"\\x{ord(c):02x}" if ord(c) <= 0x7f else
		f"\\u{{{ord(c):04x}}}"
		for c in s
	)

f = open(OUTPUT_PATH, "w")
f.write(f"""\
use crate::tests::TestCase;
pub const TEST_CASES: [TestCase; {len(test_cases)}] = [
""")
for line_no, test_str, breaks in test_cases:
	f.write(
		f"\tTestCase::new({line_no}, \"{escape_str(test_str)}\", &["
		+ ",".join(f"{off}" for off in breaks) + "]),\n"
	)
f.write("];\n")
f.flush()

print("Done.")
