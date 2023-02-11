package data

import (
	"errors"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
	"sync"

	"xilefmusics.de/money-app/transaction"
)

type Data struct {
	dataPath          string
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

	return Data{dataPath, transactions, transactionsMutex}, err
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
	transaction.SortByDate(transactions)
	return transactions
}

func (data *Data) Reindex(user string) {
	// todo: error handling

	mutex, ok := data.transactionsMutex[user]
	if !ok {
		mutex = &sync.Mutex{}
		data.transactionsMutex[user] = mutex
	}
	mutex.Lock()
	defer mutex.Unlock()

	transactions, ok := data.transactions[user]
	if !ok {
		transactions = []transaction.Transaction{}
	}

	transaction.SortByDate(transactions)

	for idx, _ := range transactions {
		transactions[idx].ID = uint(idx)
	}

	data.transactions[user] = transactions
	transaction.Save(fmt.Sprint("%s/%s.json", data.dataPath, user), transactions)
}

func (data *Data) AddTransactions(user string, newTransactions []transaction.Transaction) []transaction.Transaction {
	// todo error handling

	var createdTransactions []transaction.Transaction

	mutex, ok := data.transactionsMutex[user]
	if !ok {
		mutex = &sync.Mutex{}
		data.transactionsMutex[user] = mutex
	}
	mutex.Lock()
	defer mutex.Unlock()

	transactions, ok := data.transactions[user]
	if !ok {
		transactions = []transaction.Transaction{}
	}

	var id uint
	id = 0
	for _, transaction := range transactions {
		if transaction.ID > id {
			id = transaction.ID
		}
	}
	if len(transactions) > 0 {
		id++
	}

	transaction.SortByDate(newTransactions)
	for _, transaction := range newTransactions {
		transaction.ID = id
		transactions = append(transactions, transaction)
		createdTransactions = append(createdTransactions, transaction)
		id++
	}

	data.transactions[user] = transactions
	transaction.Save(fmt.Sprintf("%s/%s.json", data.dataPath, user), transactions)

	return createdTransactions
}

func (data *Data) UpdateTransactions(user string, newTransactions []transaction.Transaction) error {
	// todo error handling

	mutex, ok := data.transactionsMutex[user]
	if !ok {
		return errors.New("Mutex not found")
	}
	mutex.Lock()
	defer mutex.Unlock()

	transactions, ok := data.transactions[user]
	if !ok {
		return errors.New("Transactions not found")
	}

	for idx, transaction := range transactions {
		for _, newTransaction := range newTransactions {
			if transaction.ID == newTransaction.ID {
				transactions[idx] = newTransaction
			}
		}
	}

	data.transactions[user] = transactions
	transaction.Save(fmt.Sprintf("%s/%s.json", data.dataPath, user), transactions)

	return nil
}
