UNICODE_VERSION = "latest"

def get_ucd_url():
	if UNICODE_VERSION == "latest":
		subdir = "UCD/latest"
	else:
		subdir = UNICODE_VERSION
	return "https://www.unicode.org/Public/" + subdir
