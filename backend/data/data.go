package data

import (
	"sync"

	"xilefmusics.de/money-app/transaction"
)

type Data struct {
	transactions      map[string][]transaction.Transaction
	transactionsMutex map[string]*sync.Mutex
}

func New() (Data, error) {
	transactions := make(map[string][]transaction.Transaction)
	transactionsMutex := make(map[string]*sync.Mutex)

	// TODO: change loading and add errorhandling
	transactions["xilef"], _ = transaction.Load("../frontend/static/transactions.json")
	transactionsMutex["xilef"] = &sync.Mutex{}

	return Data{transactions, transactionsMutex}, nil
}

func (data *Data) GetTransactions(user string) []transaction.Transaction {
	mutex, ok := data.transactionsMutex[user]
	if !ok {
		return []transaction.Transaction{}
	}
	mutex.Lock()
	defer mutex.Unlock()
	transactions, ok := data.transactions[user]
	if !ok {
		return []transaction.Transaction{}
	}
	return transactions
}
