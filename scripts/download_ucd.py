from dataclasses import dataclass
import io
import os
from typing import Callable
import urllib.request
import xml.etree.ElementTree as ElementTree
import zipfile

from url import get_ucd_url

@dataclass
class UcdRange[T]:
	start: int
	end: int
	val: T

def download_ucd_data(get_range_value: Callable[[dict[str,str], int, int], any]) -> list[UcdRange]:
	ucd_zip_url = get_ucd_url() + "/ucdxml/ucd.nounihan.flat.zip"
	print(f"Fetching {ucd_zip_url}")
	ucd_zip_path, _ = urllib.request.urlretrieve(ucd_zip_url)

	print("Parsing")
	ranges = []
	xml_ns = "{http://www.unicode.org/ns/2003/ucd/1.0}"
	ucd_file = open(ucd_zip_path, "rb")
	ucd_zip = zipfile.ZipFile(ucd_file, "r")
	file_bin = ucd_zip.open("ucd.nounihan.flat.xml")
	#total_size = ucd_zip.getinfo("ucd.nounihan.flat.xml").file_size
	file = io.TextIOWrapper(file_bin, encoding="utf-8")
	last_cp = -1
	for ev, elem in ElementTree.iterparse(file, events=("start", "end")):
		if ev == "end" and elem.tag == xml_ns + "description":
			print(f"Description: {elem.text}")
		if ev != "start" or elem.tag not in (xml_ns + "char", xml_ns + "reserved", xml_ns + "noncharacter", xml_ns + "surrogate"):
			continue
		if "cp" in elem.attrib:
			start, end = elem.attrib["cp"], elem.attrib["cp"]
		else:
			start, end = elem.attrib["first-cp"], elem.attrib["last-cp"]
		start, end = int(start, 16), int(end, 16)
		if start != last_cp + 1:
			print(f"Warning: Codepoint range U+{last_cp+1:06x} to U+{start-1:06x} is missing")
		last_cp = end

		val = get_range_value(elem.attrib, start, end)
		if val is None: continue
		
		if len(ranges) > 0 and (prev := ranges[-1]).end == start-1 and prev.val == val:
			prev.end = end
		else:
			ranges.append(UcdRange(start, end, val))
		
	if last_cp != 0x10ffff:
		print(f"Warning: Codepoint range U+{last_cp+1:06x} to U+10ffff is missing")
	
	os.remove(ucd_zip_path)

	return ranges