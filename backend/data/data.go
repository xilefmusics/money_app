package data

import (
	"log"
	"os"
	"path/filepath"
	"strings"
	"sync"

	"xilefmusics.de/money-app/transaction"
)

type Data struct {
	transactions      map[string][]transaction.Transaction
	transactionsMutex map[string]*sync.Mutex
}

func New(dataPath string) (Data, error) {
	transactions := make(map[string][]transaction.Transaction)
	transactionsMutex := make(map[string]*sync.Mutex)

	err := filepath.Walk(dataPath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if filepath.Ext(info.Name()) == ".json" {
			user := strings.TrimSuffix(info.Name(), filepath.Ext(info.Name()))
			transactions[user], _ = transaction.Load(path)
			transactionsMutex[user] = &sync.Mutex{}
			log.Printf("INFO: Load data of user %s from path %s", user, path)
		}

		return nil
	})

	return Data{transactions, transactionsMutex}, err
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

func (data *Data) Reindex(user string) {
	mutex, ok := data.transactionsMutex[user]
	if !ok {
		return
	}
	mutex.Lock()
	defer mutex.Unlock()

	transactions, ok := data.transactions[user]
	if !ok {
		return
	}

	transaction.SortByDate(transactions)

	for idx, _ := range transactions {
		transactions[idx].ID = uint(idx)
	}

	data.transactions[user] = transactions

	// TODO: change loading and add errorhandling
	transaction.Save("../frontend/static/transactions.json", transactions)
}
