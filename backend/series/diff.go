package series

type ValueDiff struct {
	Value int `json:"value"`
	Diff  int `json:"diff"`
}

type ValueDiffSeries = []ValueDiff

func GenValueDiffSeries(in []int) ValueDiffSeries {
	result := ValueDiffSeries{}
	for idx, item := range in {
		if idx >= len(in)-2 {
			idx = len(in) - 2
		}
		result = append(result, ValueDiff{item, in[idx+1] - item})
	}
	return result
}

func AddValueDiffSeries(s1, s2 ValueDiffSeries) ValueDiffSeries {
	result := ValueDiffSeries{}
	for idx, item := range s1 {
		result = append(result, ValueDiff{item.Value + s2[idx].Value, item.Diff + s2[idx].Diff})
	}
	return result
}

func ScaleValueDiffSeries(s ValueDiffSeries, n int) ValueDiffSeries {
	result := ValueDiffSeries{}
	for _, item := range s {
		result = append(result, ValueDiff{item.Value * n, item.Diff * n})
	}
	return result
}
