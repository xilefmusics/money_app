package event

import (
	"encoding/json"
	"io/ioutil"
	"os"
	"sort"
	"time"

	"xilefmusics.de/money-app/transaction"
)

type Event struct {
	ID              uint                      `json:"id"`
	Date            time.Time                 `json:"date"`
	Reason          string                    `json:"type"`
	Type            string                    `json:"reason"`
	TransactionsOld []transaction.Transaction `json:"transactionsOld"`
	TransactionsNew []transaction.Transaction `json:"transactionsNew"`
}

func SortByDateReverse(events []Event) {
	sort.Slice(events, func(i, j int) bool {
		return events[i].Date.After(events[j].Date)
	})
}

func Load(path string) ([]Event, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	bytes, err := ioutil.ReadAll(file)
	if err != nil {
		return nil, err
	}

	var events []Event
	err = json.Unmarshal(bytes, &events)
	if err != nil {
		return nil, err
	}

	return events, nil
}

func Save(path string, events []Event) error {
	bytes, err := json.MarshalIndent(events, "", "  ")
	if err != nil {
		return err
	}

	ioutil.WriteFile(path, bytes, 0644)
	return nil
}
