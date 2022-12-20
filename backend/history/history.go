package history

import (
	"xilefmusics.de/money-app/series"
	"xilefmusics.de/money-app/transaction"
)

const (
	Debt     = 0
	Budget   = 1
	Inbudget = 2
	Pod      = 3
	Wealth   = 4
)

type Kind = int

func history(year, month, day, n int, transactions []transaction.Transaction, keys []string, f func(t transaction.Transaction, key string) int) []map[string]interface{} {
	timeSeries := series.GenTimeSeries(year, month, day, n)
	result := []map[string]interface{}{}
	interim := map[string]series.ValueDiffSeries{}
	for _, key := range keys {
		interim[key] = series.GenValueDiffSeriesFromTransactions(timeSeries, transactions, key, f)
	}
	for idx, date := range timeSeries {
		resultItem := map[string]interface{}{}
		resultItem["date"] = date
		for name, history := range interim {
			resultItem[name] = history[idx]
		}
		result = append(result, resultItem)
	}
	return result
}

func History(year, month, day, n int, kind Kind, transactions []transaction.Transaction) []map[string]interface{} {
	switch kind {
	case Debt:
		return history(year, month, day, n, transactions, transaction.GetDebts(transactions), func(t transaction.Transaction, key string) int {
			return t.SignDebtSum(key)
		})
	case Budget:
		return history(year, month, day, n, transactions, transaction.GetBudgets(transactions), func(t transaction.Transaction, key string) int {
			return t.BudgetSum(key)
		})
	case Inbudget:
		return history(year, month, day, n, transactions, transaction.GetInbudgets(transactions), func(t transaction.Transaction, key string) int {
			return t.InbudgetSum(key)
		})
	case Pod:
		return history(year, month, day, n, transactions, transaction.GetPods(transactions), func(t transaction.Transaction, key string) int {
			if t.In() {
				return t.Amount
			}
			if t.Out() {
				return -t.Amount
			}
			return 0
		})
	case Wealth:
		return history(year, month, day, n, transactions, []string{"sum", "debt", "real", "in", "out", "change"}, func(t transaction.Transaction, key string) int {
			switch key {
			case "sum":
				return t.SignAmount()
			case "debt":
				return t.SignDebtSum("*")
			case "real":
				return t.SignAmount() - t.SignDebtSum("*")
			case "in":
				return t.InbudgetSum("*")
			case "out":
				return t.BudgetSum("*")
			case "change":
				return t.InbudgetSum("*") - t.BudgetSum("*")
			}
			return 0
		})
	}
	return nil
}
