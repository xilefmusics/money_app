package transaction

import (
	"encoding/json"
	"io/ioutil"
	"os"
	"sort"
	"time"

	"golang.org/x/exp/slices"
)

type Transaction struct {
	ID        uint              `json:"id"`
	Type      string            `json:"type"`
	Date      time.Time         `json:"date"`
	Amount    int               `json:"amount"`
	Sender    string            `json:"sender"`
	Receiver  string            `json:"receiver"`
	Budgets   map[string]int    `json:"budgets"`
	Inbudgets map[string]int    `json:"inbudgets"`
	Debts     map[string]int    `json:"debts"`
	Tags      map[string]string `json:"tags"`
}

func (self Transaction) In() bool {
	return self.Type == "in"
}

func (self Transaction) Out() bool {
	return self.Type == "out"
}

func (self Transaction) SignAmount() int {
	if self.In() {
		return self.Amount
	}
	if self.Out() {
		return -self.Amount
	}
	return 0
}

func (self Transaction) DebtSum(name string) int {
	result := 0
	for debtName, debt := range self.Debts {
		if name == "*" || name == debtName {
			result += debt
		}
	}
	return result
}

func (self Transaction) SignDebtSum(name string) int {
	if self.In() {
		return self.DebtSum(name)
	}
	if self.Out() {
		return -self.DebtSum(name)
	}
	return 0
}

func (self Transaction) InbudgetSum(name string) int {
	result := 0
	for inbudgetName, inbudget := range self.Inbudgets {
		if name == "*" || name == inbudgetName {
			result += inbudget
		}
	}
	return result
}

func (self Transaction) BudgetSum(name string) int {
	result := 0
	for budgetName, budget := range self.Budgets {
		if name == "*" || name == budgetName {
			result += budget
		}
	}
	return result
}

func (self Transaction) Before(other Transaction) bool {
	return self.Date.Before(other.Date)
}

func SortByDate(transactions []Transaction) {
	sort.Slice(transactions, func(i, j int) bool {
		return transactions[i].Before(transactions[j])
	})
}

func GetDebts(transactions []Transaction) []string {
	result := []string{}
	for _, transaction := range transactions {
		for debt := range transaction.Debts {
			if !slices.Contains(result, debt) {
				result = append(result, debt)
			}
		}
	}
	sort.Strings(result)
	return result
}

func GetBudgets(transactions []Transaction) []string {
	result := []string{}
	for _, transaction := range transactions {
		for budget := range transaction.Budgets {
			if !slices.Contains(result, budget) {
				result = append(result, budget)
			}
		}
	}
	sort.Strings(result)
	return result
}

func GetInbudgets(transactions []Transaction) []string {
	result := []string{}
	for _, transaction := range transactions {
		for inbudget := range transaction.Inbudgets {
			if !slices.Contains(result, inbudget) {
				result = append(result, inbudget)
			}
		}
	}
	sort.Strings(result)
	return result
}

func GetTags(transactions []Transaction) []string {
	result := []string{}
	for _, transaction := range transactions {
		for tag := range transaction.Tags {
			if !slices.Contains(result, tag) {
				result = append(result, tag)
			}
		}
	}
	sort.Strings(result)
	return result
}

func GetPods(transactions []Transaction) []string {
	result := []string{}
	for _, transaction := range transactions {
		if transaction.In() {
			if !slices.Contains(result, transaction.Receiver) {
				result = append(result, transaction.Receiver)
			}
		} else if transaction.Out() {
			if !slices.Contains(result, transaction.Sender) {
				result = append(result, transaction.Sender)
			}
		} else {
			if !slices.Contains(result, transaction.Receiver) {
				result = append(result, transaction.Receiver)
			}
			if !slices.Contains(result, transaction.Sender) {
				result = append(result, transaction.Sender)
			}
		}
	}
	sort.Strings(result)
	return result
}

func Load(path string) ([]Transaction, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	bytes, err := ioutil.ReadAll(file)
	if err != nil {
		return nil, err
	}

	var transactions []Transaction
	err = json.Unmarshal(bytes, &transactions)
	if err != nil {
		return nil, err
	}

	return transactions, nil
}

func Save(path string, transactions []Transaction) error {
	bytes, err := json.MarshalIndent(transactions, "", "  ")
	if err != nil {
		return err
	}

	ioutil.WriteFile(path, bytes, 0644)
	return nil
}

func GetNextID(transactions []Transaction) (id uint) {
	id = 0
	for _, transaction := range transactions {
		if transaction.ID > id {
			id = transaction.ID
		}
	}
	if len(transactions) > 0 {
		id++
	}
	return
}

func Add(current, new []Transaction) ([]Transaction, []Transaction) {
	created := []Transaction{}

	id := GetNextID(current)

	for _, transaction := range new {
		transaction.ID = id
		current = append(current, transaction)
		created = append(created, transaction)
		id++
	}

	return current, created
}

func Update(current, new []Transaction) ([]Transaction, []Transaction, []Transaction) {
	updated := []Transaction{}
	old := []Transaction{}

	for _, newTransaction := range new {
		for idx, transaction := range current {
			if transaction.ID == newTransaction.ID {
				old = append(old, transaction)
				current[idx] = newTransaction
				updated = append(updated, newTransaction)
				break
			}
		}
	}

	return current, updated, old
}

func Delete(current, toDelete []Transaction) ([]Transaction, []Transaction) {
	deleted := []Transaction{}

	for _, transactionToDelete := range toDelete {
		for idx, transaction := range current {
			if transaction.ID == transactionToDelete.ID {
				current = append(current[:idx], current[idx+1:]...)
				deleted = append(deleted, transaction)
				break
			}
		}
	}

	return current, deleted
}

func Reindex(transactions []Transaction) []Transaction {
	SortByDate(transactions)
	for idx, _ := range transactions {
		transactions[idx].ID = uint(idx)
	}
	return transactions
}
