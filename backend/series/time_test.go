package series

import (
	"reflect"
	"testing"
	"time"
)

func TestGenTimeSeries(t *testing.T) {
	// len 1 single
	if !reflect.DeepEqual(genTimeSeries(1, 0, 0, 1, time.Date(2022, 05, 07, 0, 0, 0, 0, time.Now().Location())), TimeSeries{time.Date(2023, 1, 1, 0, 0, 0, 0, time.Now().Location())}) {
		t.Fatalf("1, 0, 0, 1")
	}

	if !reflect.DeepEqual(genTimeSeries(0, 1, 0, 1, time.Date(2022, 05, 07, 0, 0, 0, 0, time.Now().Location())), TimeSeries{time.Date(2022, 6, 1, 0, 0, 0, 0, time.Now().Location())}) {
		t.Fatalf("0, 1, 0, 1")
	}

	if !reflect.DeepEqual(genTimeSeries(0, 0, 1, 1, time.Date(2022, 05, 07, 0, 0, 0, 0, time.Now().Location())), TimeSeries{time.Date(2022, 5, 8, 0, 0, 0, 0, time.Now().Location())}) {
		t.Fatalf("0, 0, 1, 1")
	}

	// len 1 multi
	if !reflect.DeepEqual(genTimeSeries(0, 1, 1, 1, time.Date(2022, 05, 07, 0, 0, 0, 0, time.Now().Location())), TimeSeries{time.Date(2022, 6, 2, 0, 0, 0, 0, time.Now().Location())}) {
		t.Fatalf("0, 1, 1, 1")
	}

	if !reflect.DeepEqual(genTimeSeries(1, 0, 1, 1, time.Date(2022, 05, 07, 0, 0, 0, 0, time.Now().Location())), TimeSeries{time.Date(2023, 1, 2, 0, 0, 0, 0, time.Now().Location())}) {
		t.Fatalf("1, 0, 1, 1")
	}

	if !reflect.DeepEqual(genTimeSeries(1, 1, 0, 1, time.Date(2022, 05, 07, 0, 0, 0, 0, time.Now().Location())), TimeSeries{time.Date(2023, 2, 1, 0, 0, 0, 0, time.Now().Location())}) {
		t.Fatalf("1, 1, 0, 1")
	}

	if !reflect.DeepEqual(genTimeSeries(1, 1, 1, 1, time.Date(2022, 05, 07, 0, 0, 0, 0, time.Now().Location())), TimeSeries{time.Date(2023, 2, 2, 0, 0, 0, 0, time.Now().Location())}) {
		t.Fatalf("1, 1, 1, 1")
	}

	// len 0 and negative
	if !reflect.DeepEqual(genTimeSeries(1, 0, 0, 0, time.Date(2022, 05, 07, 0, 0, 0, 0, time.Now().Location())), TimeSeries{}) {
	}

	if !reflect.DeepEqual(genTimeSeries(1, 0, 0, -5, time.Date(2022, 05, 07, 0, 0, 0, 0, time.Now().Location())), TimeSeries{}) {
		t.Fatalf("1, 0, 0, 0")
	}

	// len 2 single
	if !reflect.DeepEqual(genTimeSeries(1, 0, 0, 2, time.Date(2022, 05, 07, 0, 0, 0, 0, time.Now().Location())), TimeSeries{time.Date(2022, 1, 1, 0, 0, 0, 0, time.Now().Location()), time.Date(2023, 1, 1, 0, 0, 0, 0, time.Now().Location())}) {
		t.Fatalf("1, 0, 0, 2")
	}

	if !reflect.DeepEqual(genTimeSeries(0, 1, 0, 2, time.Date(2022, 05, 07, 0, 0, 0, 0, time.Now().Location())), TimeSeries{time.Date(2022, 5, 1, 0, 0, 0, 0, time.Now().Location()), time.Date(2022, 6, 1, 0, 0, 0, 0, time.Now().Location())}) {
		t.Fatalf("0, 1, 0, 2")
	}

	if !reflect.DeepEqual(genTimeSeries(0, 0, 1, 2, time.Date(2022, 05, 07, 0, 0, 0, 0, time.Now().Location())), TimeSeries{time.Date(2022, 5, 7, 0, 0, 0, 0, time.Now().Location()), time.Date(2022, 5, 8, 0, 0, 0, 0, time.Now().Location())}) {
		t.Fatalf("0, 0, 1, 2")
	}

	// trans
	if !reflect.DeepEqual(genTimeSeries(0, 1, 0, 1, time.Date(2022, 12, 07, 0, 0, 0, 0, time.Now().Location())), TimeSeries{time.Date(2023, 1, 1, 0, 0, 0, 0, time.Now().Location())}) {
		t.Fatalf("trans 0, 1, 0, 1")
	}

	if !reflect.DeepEqual(genTimeSeries(0, 0, 1, 1, time.Date(2022, 12, 31, 0, 0, 0, 0, time.Now().Location())), TimeSeries{time.Date(2023, 1, 1, 0, 0, 0, 0, time.Now().Location())}) {
		t.Fatalf("trans 0, 0, 1, 1")
	}
}
