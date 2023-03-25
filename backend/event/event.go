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
	return ioutil.WriteFile(path, bytes, 0644)
}

func GetNextID(events []Event) (id uint) {
	id = 0
	for _, event := range events {
		if event.ID > id {
			id = event.ID
		}
	}
	if len(events) > 0 {
		id++
	}
	return
}

func Add(current []Event, event Event) []Event {
	event.ID = GetNextID(current)
	return append(current, event)
}

func Find(events []Event) int {
	SortByDateReverse(events)
	for idx, e := range events {
		if e.Reason == "default" {
			return idx
		}
	}
	return -1
}
