package import_transactions

func Split(line string, separator rune, quote rune) []string {
	if line[len(line)-1] == '\n' {
		line = line[:(len(line) - 1)]
	}

	result := []string{}

	inner := false
	escape := false
	current := ""
	for _, char := range line {
		if !escape && inner && char == quote {
			inner = false
		} else if !escape && !inner && char == rune(separator) {
			result = append(result, current)
			current = ""
		} else if !escape && !inner && char == quote {
			inner = true
		} else if inner && char == '\\' {
			escape = true
		} else {
			current += string(char)
			escape = false
		}
	}

	result = append(result, current)

	return result
}

func SplitCSV(line string) []string {
	return Split(line, ',', '"')
}
