package data

import (
	"errors"
	"sync"
	"time"

	"xilefmusics.de/money-app/event"
	"xilefmusics.de/money-app/helper"
	"xilefmusics.de/money-app/transaction"
)

type UserData struct {
	username         string
	mutex            sync.Mutex
	transactions     []transaction.Transaction
	transactionsPath string
	events           []event.Event
	eventsPath       string
}

func new(username, transactionsPath, eventsPath string) (*UserData, error) {
	mutex := sync.Mutex{}

	transactions := []transaction.Transaction{}
	transactionsPathExists, err := helper.OsPathExists(transactionsPath)
	if err != nil {
		return nil, err
	}
	if transactionsPathExists {
		transactions, err = transaction.Load(transactionsPath)
		if err != nil {
			return nil, err
		}
		transaction.SortByDate(transactions)
	}

	events := []event.Event{}
	eventsPathExists, err := helper.OsPathExists(eventsPath)
	if err != nil {
		return nil, err
	}
	if eventsPathExists {
		events, err = event.Load(eventsPath)
		if err != nil {
			return nil, err
		}
	}

	return &UserData{username, mutex, transactions, transactionsPath, events, eventsPath}, nil
}

func (self *UserData) lock() {
	self.mutex.Lock()
}

func (self *UserData) unlock() {
	self.mutex.Unlock()
}

func (self *UserData) save() error {
	err := transaction.Save(self.transactionsPath, self.transactions)
	if err != nil {
		return err
	}
	return event.Save(self.eventsPath, self.events)
}

func (self *UserData) getTransactions() []transaction.Transaction {
	self.lock()
	defer self.unlock()

	transaction.SortByDate(self.transactions)

	return self.transactions
}

func (self *UserData) addTransactions(transactions []transaction.Transaction, rollback bool) ([]transaction.Transaction, error) {
	if !rollback {
		self.lock()
		defer self.unlock()
	}

	transaction.SortByDate(transactions)
	current, new := transaction.Add(self.transactions, transactions)
	if !rollback {
		self.events = event.Add(self.events, event.Event{0, time.Now(), "default", "addTransactions", transactions, new})
	}
	transaction.SortByDate(current)
	self.transactions = current

	return new, self.save()
}

func (self *UserData) updateTransactions(transactions []transaction.Transaction, rollback bool) ([]transaction.Transaction, error) {
	if !rollback {
		self.lock()
		defer self.unlock()
	}

	current, new, old := transaction.Update(self.transactions, transactions)
	if !rollback {
		self.events = event.Add(self.events, event.Event{0, time.Now(), "default", "updateTransactions", old, new})
	}
	transaction.SortByDate(current)
	self.transactions = current

	return new, self.save()
}

func (self *UserData) deleteTransactions(transactions []transaction.Transaction, rollback bool) ([]transaction.Transaction, error) {
	if !rollback {
		self.lock()
		defer self.unlock()
	}

	current, deleted := transaction.Delete(self.transactions, transactions)
	if !rollback {
		self.events = event.Add(self.events, event.Event{0, time.Now(), "default", "deleteTransactions", transactions, deleted})
	}
	self.transactions = current

	return deleted, self.save()
}

func (self *UserData) reindexTransactions() error {
	self.lock()
	defer self.unlock()

	reindexed := transaction.Reindex(self.transactions)
	self.events = event.Add(self.events, event.Event{0, time.Now(), "default", "reindex", self.transactions, reindexed})
	self.transactions = reindexed

	return self.save()
}

func (self *UserData) undo() (event.Event, error) {
	self.lock()
	defer self.unlock()

	idx := event.Find(self.events)
	if idx < 0 {
		return event.Event{}, errors.New("no event to rollback found")
	}

	if self.events[idx].Type == "addTransactions" {
		_, err := self.deleteTransactions(self.events[idx].TransactionsNew, true)
		if err != nil {
			return event.Event{}, err
		}
	} else if self.events[idx].Type == "updateTransactions" {
		_, err := self.updateTransactions(self.events[idx].TransactionsOld, true)
		if err != nil {
			return event.Event{}, err
		}
	} else if self.events[idx].Type == "deleteTransactions" {
		_, err := self.addTransactions(self.events[idx].TransactionsNew, true)
		if err != nil {
			return event.Event{}, err
		}
	}

	self.events[idx].Reason = "rolledBack"

	return self.events[idx], self.save()
}
