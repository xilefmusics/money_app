package import_transactions

import (
	"errors"
	"strings"
	"time"

	"xilefmusics.de/money-app/transaction"
)

func parseN26Transaction(line string) (transaction.Transaction, error) {
	columns := SplitCSV(line)
	if len(columns) < 9 {
		return transaction.Transaction{}, errors.New("Parse N26: Not enough columns")
	}

	transaction := transaction.Transaction{}

	date, err := time.Parse("2006-01-02", columns[0])
	if err != nil {
		return transaction, err
	}
	transaction.Date = date

	amount, err := ParseAmount(columns[5])
	if err != nil {
		return transaction, err
	}
	transaction.Amount = amount

	if columns[1][0:3] == "Von" {
		transaction.Type = "move"
		transaction.Sender = strings.Split(columns[1], " ")[1]
		transaction.Receiver = strings.Split(columns[1], " ")[3]
	} else if columns[3] == "Lastschrift" || columns[3] == "Überweisung" || columns[3] == "MasterCard Zahlung" {
		transaction.Type = "out"
		transaction.Sender = "N26 Hauptkonto"
	} else if columns[3] == "Gutschrift" {
		transaction.Type = "in"
		transaction.Receiver = "N26 Hauptkonto"
	} else {
		return transaction, errors.New("Parse N26: Unknown type")
	}

	transaction.Tags = map[string]string{}
	transaction.Tags["amount"] = columns[5]
	transaction.Tags["amount_foreign_currency"] = columns[6]
	transaction.Tags["bank"] = "N26"
	transaction.Tags["date"] = columns[0]
	transaction.Tags["exchange_rate"] = columns[8]
	transaction.Tags["foreign_currency"] = columns[7]
	transaction.Tags["iban"] = columns[2]
	transaction.Tags["purpos_of_use"] = columns[4]
	transaction.Tags["receiver"] = columns[1]
	transaction.Tags["transaction_type"] = columns[3]

	return transaction, nil
}

func N26(data string) ([]transaction.Transaction, error) {
	transactions := []transaction.Transaction{}

	lines := strings.Split(data, "\n")
	for idx, line := range lines {
		if idx == 0 || len(line) == 0 {
			continue
		}
		transaction, err := parseN26Transaction(line)
		if err != nil {
			return transactions, err
		}
		transactions = append(transactions, transaction)
	}

	return transactions, nil
}
