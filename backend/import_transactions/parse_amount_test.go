package import_transactions

import (
	"testing"
)

func TestParseAmount(t *testing.T) {
	testCasesIn := []string{"42.00", "-42.00", "+42.00", "42,00", "-42,00", "+42,00", "42.42", "42.02", "0.99", "-0.99", "+0.99", "1,000.00", "1.000,00", "42,00 €", "42", "42.", ".42"}
	testCasesOut := []int{4200, 4200, 4200, 4200, 4200, 4200, 4242, 4202, 99, 99, 99, 100000, 100000, 4200, 0, 0, 0}
	testCasesErr := []bool{false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true}

	for idx, _ := range testCasesIn {
		result, err := ParseAmount(testCasesIn[idx])
		if testCasesErr[idx] && err == nil {
			t.Fatalf("ERROR: test case %d didn't throw an error", idx)
		}
		if result != testCasesOut[idx] {
			t.Fatalf("ERROR: test case %d wrong result (expected %d but was %d)", idx, testCasesOut[idx], result)
		}
	}

}
