package import_transactions

import (
	"errors"
	"strconv"
	"strings"
)

func isDigit(c byte) bool {
	return c == '1' || c == '2' || c == '3' || c == '4' || c == '5' || c == '6' || c == '7' || c == '8' || c == '9' || c == '0'
}

func ParseAmount(str string) (int, error) {
	for !isDigit(str[0]) {
		str = str[1:]
	}
	for !isDigit(str[len(str)-1]) {
		str = str[:(len(str) - 1)]
	}

	str = strings.Replace(str, ",", ".", -1)
	for strings.Index(str, ".") != strings.LastIndex(str, ".") {
		idx := strings.Index(str, ".")
		str = str[:idx] + str[idx+1:]
	}

	split := strings.Split(str, ".")
	if len(split) != 2 {
		return 0, errors.New("ERROR: No '.' or ',' found in ParseAmount")
	}

	before, err := strconv.Atoi(split[0])
	if err != nil {
		return 0, err
	}

	after, err := strconv.Atoi(split[1])
	if err != nil {
		return 0, err
	}
	if len(split[1]) == 1 {
		after *= 10
	}

	return before*100 + after, nil
}
