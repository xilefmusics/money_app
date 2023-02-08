package import_transactions

import (
	"reflect"
	"testing"
)

func TestSplitCSV(t *testing.T) {
	ab := []string{"a,b", "\"a\",\"b\"", "a,b\n", "\"a\",\"b\"\n"}

	for idx, _ := range ab {
		result := SplitCSV(ab[idx])
		if !reflect.DeepEqual(result, []string{"a", "b"}) {
			t.Fatalf("ERROR: ab %d wrong result\n", idx)
		}
	}

	if !reflect.DeepEqual(SplitCSV("a,\"b, c\",d"), []string{"a", "b, c", "d"}) {
		t.Fatalf("ERROR: a,\"b, c\", d\n")
	}
	if !reflect.DeepEqual(SplitCSV(","), []string{"", ""}) {
		t.Fatalf("ERROR: ,\n")
	}
	if !reflect.DeepEqual(SplitCSV(",\"\\\"\""), []string{"", "\""}) {
		t.Fatalf("ERROR: ,\"\\\"\"\n")
	}

}
