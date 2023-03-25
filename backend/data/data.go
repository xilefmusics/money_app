package data

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"xilefmusics.de/money-app/event"
	"xilefmusics.de/money-app/helper"
	"xilefmusics.de/money-app/transaction"
)

type Data struct {
	data             map[string]*UserData
	transactionsPath string
	eventsPath       string
	filesPath        string
}

func getListOfUsers(path string) ([]string, error) {
	var users []string

	err := filepath.Walk(path, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if filepath.Ext(info.Name()) == ".json" {
			users = append(users, strings.TrimSuffix(info.Name(), filepath.Ext(info.Name())))
		}
		return nil
	})

	return users, err
}

func New(dataPath string) (*Data, error) {
	transactionsPath := filepath.Join(dataPath, "transactions")
	transactionsPathExists, err := helper.OsPathExists(transactionsPath)
	if err != nil {
		return nil, err
	}
	if !transactionsPathExists {
		if err := os.Mkdir(transactionsPath, os.ModePerm); err != nil {
			return nil, err
		}
	}

	eventsPath := filepath.Join(dataPath, "events")
	eventsPathExists, err := helper.OsPathExists(eventsPath)
	if err != nil {
		return nil, err
	}
	if !eventsPathExists {
		if err := os.Mkdir(eventsPath, os.ModePerm); err != nil {
			return nil, err
		}
	}

	filesPath := filepath.Join(dataPath, "files")
	filesPathExists, err := helper.OsPathExists(filesPath)
	if err != nil {
		return nil, err
	}
	if !filesPathExists {
		if err := os.Mkdir(filesPath, os.ModePerm); err != nil {
			return nil, err
		}
	}

	transactionsUsers, err := getListOfUsers(transactionsPath)
	if err != nil {
		return nil, err
	}

	eventsUsers, err := getListOfUsers(eventsPath)
	if err != nil {
		return nil, err
	}

	data := map[string]*UserData{}
	for _, user := range helper.StringUniqueMergeSlices(transactionsUsers, eventsUsers) {
		userData, err := new(user, filepath.Join(transactionsPath, fmt.Sprintf("%s.json", user)), filepath.Join(eventsPath, fmt.Sprintf("%s.json", user)), filepath.Join(filesPath, user))
		if err != nil {
			return nil, err
		}
		data[user] = userData
	}

	return &Data{data, transactionsPath, eventsPath, filesPath}, nil
}

func (self *Data) getUserData(user string) (*UserData, error) {
	userData, ok := self.data[user]
	if !ok {
		userData, err := new(user, self.transactionsPath, self.eventsPath, self.filesPath)
		if err != nil {
			return nil, err
		}
		self.data[user] = userData
	}
	return userData, nil
}

func (self *Data) GetTransactions(user string) ([]transaction.Transaction, error) {
	userData, err := self.getUserData(user)
	if err != nil {
		return []transaction.Transaction{}, err
	}
	return userData.getTransactions(), nil
}

func (self *Data) AddTransactions(user string, transactions []transaction.Transaction, addEvent bool) ([]transaction.Transaction, error) {
	userData, err := self.getUserData(user)
	if err != nil {
		return []transaction.Transaction{}, err
	}
	return userData.addTransactions(transactions, false)
}

func (self *Data) UpdateTransactions(user string, transactions []transaction.Transaction, addEvent bool) ([]transaction.Transaction, error) {
	userData, err := self.getUserData(user)
	if err != nil {
		return []transaction.Transaction{}, err
	}
	return userData.updateTransactions(transactions, false)
}

func (self *Data) DeleteTransactions(user string, transactions []transaction.Transaction, addEvent bool) ([]transaction.Transaction, error) {
	userData, err := self.getUserData(user)
	if err != nil {
		return []transaction.Transaction{}, err
	}
	return userData.deleteTransactions(transactions, false)
}

func (self *Data) ReindexTransactions(user string) error {
	userData, err := self.getUserData(user)
	if err != nil {
		return err
	}
	return userData.reindexTransactions()
}

func (self *Data) Undo(user string) (event.Event, error) {
	userData, err := self.getUserData(user)
	if err != nil {
		return event.Event{}, err
	}
	return userData.undo()
}

func (self *Data) GetAttachment(user, fileName string) ([]byte, error) {
	userData, err := self.getUserData(user)
	if err != nil {
		return nil, err
	}
	return userData.getAttachment(fileName)
}

func (self *Data) AddAttachment(user string, bytes []byte, ext string) (string, error) {
	userData, err := self.getUserData(user)
	if err != nil {
		return "", err
	}
	return userData.addAttachment(bytes, ext)
}
