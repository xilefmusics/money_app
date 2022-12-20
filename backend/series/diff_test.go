package series

import (
	"reflect"
	"testing"
)

func TestGenValueDiffSeries(t *testing.T) {
	if !reflect.DeepEqual(GenValueDiffSeries([]int{1, 2, -5, -2, 0, 3}), ValueDiffSeries{ValueDiff{1, 1}, ValueDiff{2, -7}, ValueDiff{-5, 3}, ValueDiff{-2, 2}, ValueDiff{0, 3}, ValueDiff{3, 0}}) {
		t.Failed()
	}
}

func TestAddValueDiffSeries(t *testing.T) {
	if !reflect.DeepEqual(AddValueDiffSeries(ValueDiffSeries{ValueDiff{1, -1}, ValueDiff{-1, 1}}, ValueDiffSeries{ValueDiff{-1, 1}, ValueDiff{1, -1}}), ValueDiffSeries{ValueDiff{0, 0}, ValueDiff{0, 0}}) {
		t.Failed()
	}
}

func TestScaleValueDiffSeries(t *testing.T) {
	if !reflect.DeepEqual(ScaleValueDiffSeries(ValueDiffSeries{ValueDiff{1, -1}, ValueDiff{-1, 1}}, -1), ValueDiffSeries{ValueDiff{-1, 1}, ValueDiff{1, -1}}) {
		t.Failed()
	}
}
