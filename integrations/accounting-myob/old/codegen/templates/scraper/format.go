package scraper

func formatType(t string) string {
	if t == "guid" {
		return "string"
	}

	if t == "integer" {
		return "int64"
	}

	if t == "decimal" {
		return "*decimal.Decimal"
	}

	if t == "datetime" {
		return "*time.Time"
	}

	if t == "UID" {
		return "string"
	}

	return t
}
