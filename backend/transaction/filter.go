package transaction

import (
	"strconv"
)

func filterPod(transaction Transaction, podFilter string) bool {
	return podFilter == "*" || transaction.Sender == podFilter || transaction.Receiver == podFilter
}

func filterDebt(transaction Transaction, debtFilter string) bool {
	if debtFilter == "*" {
		return true
	}
	for debt, _ := range transaction.Debts {
		if debt == debtFilter {
			return true
		}
	}
	return false
}

func filterBudget(transaction Transaction, budgetFilter string) bool {
	if budgetFilter == "*" {
		return true
	}
	for debt, _ := range transaction.Budgets {
		if debt == budgetFilter {
			return true
		}
	}
	return false
}

func filterInbudget(transaction Transaction, inbudgetFilter string) bool {
	if inbudgetFilter == "*" {
		return true
	}
	for debt, _ := range transaction.Inbudgets {
		if debt == inbudgetFilter {
			return true
		}
	}
	return false
}

func filterType(transaction Transaction, typeFilter string) bool {
	return typeFilter == "*" || transaction.Type == typeFilter
}

func filterId(transaction Transaction, idFilter string) bool {
	if idFilter == "*" {
		return true
	}
	id, err := strconv.ParseUint(idFilter, 10, 32)
	if err != nil {
		return false
	}
	return idFilter == "*" || transaction.ID == uint(id)
}

func Filter(transactions []Transaction, podFilter, debtFilter, budgetFilter, inbudgetFilter, typeFilter, idFilter string) []Transaction {
	result := []Transaction{}
	for _, t := range transactions {
		if filterPod(t, podFilter) && filterDebt(t, debtFilter) && filterBudget(t, budgetFilter) && filterInbudget(t, inbudgetFilter) && filterType(t, typeFilter) && filterId(t, idFilter) {
			result = append(result, t)
		}
	}
	return result
}
