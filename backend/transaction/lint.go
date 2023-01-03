package transaction

import (
	"fmt"
)

func LintTransaction(transaction Transaction) (string, bool) {
	// TODO: Check no inbudget if not in, no budget if not out and no debt if not in or out

	if transaction.Out() {
		budgetSum := transaction.BudgetSum("*")
		debtSum := transaction.DebtSum("*")
		schuldenreduzierung := transaction.BudgetSum("Schuldenreduzierung")

		if budgetSum != transaction.Amount {
			return fmt.Sprintf("%d: Invalid amounts: budgetSum (%d) != amount (%d)", transaction.ID, budgetSum, transaction.Amount), false
		}

		if debtSum != schuldenreduzierung {
			return fmt.Sprintf("%d: Invalid amounts: debtSum (%d) != budget Schuldenreduzierung (%d)", transaction.ID, debtSum, schuldenreduzierung), false
		}
	}

	if transaction.In() {
		inbudgetSum := transaction.InbudgetSum("*")
		debtSum := transaction.DebtSum("*")
		schuldenaufbau := transaction.InbudgetSum("Schuldenaufbau")

		if inbudgetSum != transaction.Amount {
			return fmt.Sprintf("%d: Invalid amounts: inbudgetSum (%d) != amount (%d)", transaction.ID, inbudgetSum, transaction.Amount), false
		}

		if debtSum != schuldenaufbau {
			return fmt.Sprintf("%d: Invalid amounts: debtSum (%d) != budget Schuldenaufbau (%d)", transaction.ID, debtSum, schuldenaufbau), false
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
