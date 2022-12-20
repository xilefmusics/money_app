package series

import "xilefmusics.de/money-app/transaction"

func GenIntSeriesFromTransactions(timeSeries TimeSeries, transactions []transaction.Transaction, key string, f func(transaction.Transaction, string) int) []int {
	var result = []int{}
	var value int = 0
	timeSeriesIdx := 0
	for _, transaction := range transactions {
		if timeSeries[timeSeriesIdx].Before(transaction.Date) {
			result = append(result, value)
			timeSeriesIdx += 1
			if timeSeriesIdx >= len(timeSeries) {
				return result
			}
		}
		value += f(transaction, key)
	}
	result = append(result, value)
	return result
}

func GenValueDiffSeriesFromTransactions(timeSeries TimeSeries, transactions []transaction.Transaction, key string, f func(transaction.Transaction, string) int) ValueDiffSeries {
	return GenValueDiffSeries(GenIntSeriesFromTransactions(timeSeries, transactions, key, f))
}
