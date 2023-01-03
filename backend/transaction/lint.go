package transaction

import (
	"fmt"
)

func LintTransaction(transaction Transaction) (string, bool) {
	if transaction.Out() {
		budgetSum := transaction.BudgetSum("*")
		inbudgetSum := transaction.InbudgetSum("*")
		debtSum := transaction.DebtSum("*")

		if budgetSum+debtSum != transaction.Amount {
			return fmt.Sprintf("%d: Invalid amounts: budgetSum (%d) + debtSum (%d) != amount (%d)", transaction.ID, budgetSum, debtSum, transaction.Amount), false
		}

		if inbudgetSum != 0 {
			return fmt.Sprintf("%d: Invalid amounts: inbudgetSum (%d) != 0", transaction.ID, inbudgetSum), false
		}
	}

	if transaction.In() {
		budgetSum := transaction.BudgetSum("*")
		inbudgetSum := transaction.InbudgetSum("*")
		debtSum := transaction.DebtSum("*")

		if inbudgetSum+debtSum != transaction.Amount {
			return fmt.Sprintf("%d: Invalid amounts: inbudgetSum (%d) + debtSum (%d) != amount (%d)", transaction.ID, inbudgetSum, debtSum, transaction.Amount), false
		}

		if budgetSum != 0 {
			return fmt.Sprintf("%d: Invalid amounts: budgetSum (%d) != 0", transaction.ID, budgetSum), false
		}
	}

	if transaction.Type == "move" {
		budgetSum := transaction.BudgetSum("*")
		inbudgetSum := transaction.InbudgetSum("*")

		if budgetSum != 0 {
			return fmt.Sprintf("%d: Invalid amounts: budgetSum (%d) != 0", transaction.ID, budgetSum), false
		}

		if inbudgetSum != 0 {
			return fmt.Sprintf("%d: Invalid amounts: inbudgetSum (%d) != 0", transaction.ID, inbudgetSum), false
		}
	}

	return "Ok", true
}

func LintTransactions(transactions []Transaction) []string {
	result := []string{}
	for _, transaction := range transactions {
		msg, ok := LintTransaction(transaction)
		if !ok {
			result = append(result, msg)
		}
	}
	return result
}
