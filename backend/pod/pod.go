package pod

import (
	"log"
	"sort"

	"golang.org/x/exp/slices"

	"xilefmusics.de/money-app/transaction"
)

type Pod struct {
	Name  string   `json:"name"`
	Ibans []string `json:"iban"`
	Bank  string   `json:"bank"`
}

func SortByName(pods []Pod) {
	sort.Slice(pods, func(i, j int) bool {
		return pods[i].Name < pods[j].Name
	})
}

func ToNames(pods []Pod) []string {
	names := []string{}
	for _, pod := range pods {
		names = append(names, pod.Name)
	}
	return names
}

func fromTransactions(pods []Pod, name string, transaction transaction.Transaction) []Pod {
	if !slices.Contains(ToNames(pods), name) {
		pods = append(pods, Pod{name, []string{}, ""})
	}

	foundIdx := -1
	for idx, pod := range pods {
		if pod.Name == name {
			foundIdx = idx
			break
		}
	}

	iban, ok := transaction.Tags["iban"]
	if ok && !slices.Contains(pods[foundIdx].Ibans, iban) && transaction.Move() && name == transaction.Sender {
		pods[foundIdx].Ibans = append(pods[foundIdx].Ibans, iban)
	}

	bank, ok := transaction.Tags["bank"]
	if ok {
		pods[foundIdx].Bank = bank
	}

	return pods
}

func FromTransactions(transactions []transaction.Transaction) []Pod {
	pods := []Pod{}
	for _, transaction := range transactions {
		if !transaction.In() {
			pods = fromTransactions(pods, transaction.Sender, transaction)
		}
		if !transaction.Out() {
			pods = fromTransactions(pods, transaction.Receiver, transaction)
		}
	}
	SortByName(pods)
	log.Println(pods)
	return pods
}
