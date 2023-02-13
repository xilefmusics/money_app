package data

import (
	"errors"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	"xilefmusics.de/money-app/event"
	"xilefmusics.de/money-app/transaction"
)

type Data struct {
	dataPath          string
	transactionsPath  string
	eventsPath        string
	transactions      map[string][]transaction.Transaction
	transactionsMutex map[string]*sync.Mutex
	events            map[string][]event.Event
	eventsMutex       map[string]*sync.Mutex
}

func New(dataPath string) (Data, error) {
	transactionsPath := filepath.Join(dataPath, "transactions")
	if _, err := os.Stat(transactionsPath); err != nil {
		if os.IsNotExist(err) {
			if err := os.Mkdir(transactionsPath, os.ModePerm); err != nil {
				return Data{}, err
			}
		}
	}

	eventsPath := filepath.Join(dataPath, "events")
	if _, err := os.Stat(eventsPath); err != nil {
		if os.IsNotExist(err) {
			if err := os.Mkdir(eventsPath, os.ModePerm); err != nil {
				return Data{}, err
			}
		}
	}

	transactions := make(map[string][]transaction.Transaction)
	transactionsMutex := make(map[string]*sync.Mutex)

	err := filepath.Walk(transactionsPath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if filepath.Ext(info.Name()) == ".json" {
			user := strings.TrimSuffix(info.Name(), filepath.Ext(info.Name()))
			transactions[user], _ = transaction.Load(path)
			transactionsMutex[user] = &sync.Mutex{}
			log.Printf("INFO: Load transactions of user %s from path %s", user, path)
		}

		return nil
	})

	events := make(map[string][]event.Event)
	eventsMutex := make(map[string]*sync.Mutex)

	err = filepath.Walk(eventsPath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if filepath.Ext(info.Name()) == ".json" {
			user := strings.TrimSuffix(info.Name(), filepath.Ext(info.Name()))
			events[user], _ = event.Load(path)
			eventsMutex[user] = &sync.Mutex{}
			log.Printf("INFO: Load events of user %s from path %s", user, path)
		}

		return nil
	})

	return Data{dataPath, transactionsPath, eventsPath, transactions, transactionsMutex, events, eventsMutex}, err
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

	oldTransactions := transactions[:]

	transaction.SortByDate(transactions)

	for idx, _ := range transactions {
		transactions[idx].ID = uint(idx)
	}

	data.transactions[user] = transactions
	transaction.Save(fmt.Sprint("%s/%s.json", data.transactionsPath, user), transactions)

	data.AddEvent(user, event.Event{0, time.Now(), "default", "reindex", oldTransactions, transactions})
}

func (data *Data) AddTransactions(user string, newTransactions []transaction.Transaction, addEvent bool) []transaction.Transaction {
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
	transaction.Save(fmt.Sprintf("%s/%s.json", data.transactionsPath, user), transactions)

	if addEvent {
		data.AddEvent(user, event.Event{0, time.Now(), "default", "addTransactions", newTransactions, createdTransactions})
	}

	return createdTransactions
}

func (data *Data) UpdateTransactions(user string, newTransactions []transaction.Transaction, addEvent bool) error {
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

	var oldTransactions []transaction.Transaction

	for idx, transaction := range transactions {
		for _, newTransaction := range newTransactions {
			if transaction.ID == newTransaction.ID {
				transactions[idx] = newTransaction
			}
		}
	}

	data.transactions[user] = transactions
	transaction.Save(fmt.Sprintf("%s/%s.json", data.transactionsPath, user), transactions)

	if addEvent {
		data.AddEvent(user, event.Event{0, time.Now(), "default", "updateTransactions", oldTransactions, newTransactions})
	}

	return nil
}

func (data *Data) DeleteTransactions(user string, transactionsToDelete []transaction.Transaction, addEvent bool) []transaction.Transaction {
	// todo error handling

	var deletedTransactions []transaction.Transaction

	mutex, ok := data.transactionsMutex[user]
	if !ok {
		return deletedTransactions
	}
	mutex.Lock()
	defer mutex.Unlock()

	transactions, ok := data.transactions[user]
	if !ok {
		return deletedTransactions
	}

	for _, transactionToDelete := range transactionsToDelete {
		for idx, transaction := range transactions {
			if transaction.ID == transactionToDelete.ID {
				transactions = append(transactions[:idx], transactions[idx+1:]...)
				deletedTransactions = append(deletedTransactions, transaction)
				break
			}
		}
	}

	data.transactions[user] = transactions
	transaction.Save(fmt.Sprintf("%s/%s.json", data.transactionsPath, user), transactions)

	if addEvent {
		data.AddEvent(user, event.Event{0, time.Now(), "default", "deleteTransactions", transactionsToDelete, deletedTransactions})
	}

	return deletedTransactions
}

func (data *Data) AddEvent(user string, newEvent event.Event) {
	// todo error handling

	mutex, ok := data.eventsMutex[user]
	if !ok {
		mutex = &sync.Mutex{}
		data.eventsMutex[user] = mutex
	}
	mutex.Lock()
	defer mutex.Unlock()

	events, ok := data.events[user]
	if !ok {
		events = []event.Event{}
	}

	var id uint
	id = 0
	for _, event := range events {
		if event.ID > id {
			id = event.ID
		}
	}
	if len(events) > 0 {
		id++
	}

	newEvent.ID = id
	events = append(events, newEvent)

	data.events[user] = events
	event.Save(fmt.Sprintf("%s/%s.json", data.eventsPath, user), events)
}

func (data *Data) Undo(user string) {
	// todo error handling

	mutex, ok := data.eventsMutex[user]
	if !ok {
		mutex = &sync.Mutex{}
		data.eventsMutex[user] = mutex
	}
	mutex.Lock()
	defer mutex.Unlock()

	events, ok := data.events[user]
	if !ok {
		events = []event.Event{}
	}

	var eventVar event.Event
	var eventIdx int
	found := false
	event.SortByDateReverse(events)
	for idx, e := range events {
		if e.Reason == "default" {
			eventVar = e
			eventIdx = idx
			eventVar.Reason = "rolledBack"
			found = true
		}
	}

	if !found {
		return
	}

	if eventVar.Type == "addTransactions" {
		data.DeleteTransactions(user, eventVar.TransactionsNew, false)
	} else if eventVar.Type == "updateTransactions" {
		data.UpdateTransactions(user, eventVar.TransactionsOld, false)
	} else if eventVar.Type == "deleteTransactions" {
		data.AddTransactions(user, eventVar.TransactionsNew, false)
	} else {
		eventVar.Reason = "default"
	}

	events[eventIdx] = eventVar
	data.events[user] = events
	event.Save(fmt.Sprintf("%s/%s.json", data.eventsPath, user), events)
}
