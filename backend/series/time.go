package series

import "time"

type TimeSeries = []time.Time

func genTimeSeriesHelper(begin, end time.Time, year, month, day int, oneMore bool) TimeSeries {
	var result = TimeSeries{}
	for current := begin; !end.Before(current); current = current.AddDate(year, month, day) {
		result = append(result, current)
	}

	if oneMore {
		if len(result) > 0 {
			result = append(result, result[len(result)-1].AddDate(year, month, day))
		} else {
			result = append(result, begin)
		}
	}
	return result
}

func genTimeSeries(year, month, day, n int, now time.Time) TimeSeries {
	if n < 1 {
		return TimeSeries{}
	}
	currenYear, currentMonth, currentDay := now.Date()
	if year != 0 || month != 0 {
		currentDay = 1
	}
	if year != 0 {
		currentMonth = 1
	}

	normalized := time.Date(currenYear, currentMonth, currentDay, 0, 0, 0, 0, now.Location())
	return genTimeSeriesHelper(normalized.AddDate((2-n)*year, (2-n)*month, (2-n)*day), normalized, year, month, day, true)
}

func GenTimeSeries(year, month, day, n int) TimeSeries {
	return genTimeSeries(year, month, day, n, time.Now())
}
