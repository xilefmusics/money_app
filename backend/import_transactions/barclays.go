package import_transactions

import (
	"errors"
	"strings"
	"time"

	"xilefmusics.de/money-app/transaction"
)

func parseBarclaysTransaction(line string) (transaction.Transaction, error) {
	columns := SplitCSV(line)
	if len(columns) < 14 {
		return transaction.Transaction{}, errors.New("Parse N26: Not enough columns")
	}

	transaction := transaction.Transaction{}

	date, err := time.Parse("02.01.2006", columns[1])
	if err != nil {
		return transaction, err
	}
	transaction.Date = date

	amount, err := ParseAmount(columns[3])
	if err != nil {
		return transaction, err
	}
	transaction.Amount = amount

	if columns[5] == "Belastung" {
		transaction.Type = "out"
		transaction.Sender = "Barclays"
	} else {
		transaction.Type = "in"
		transaction.Receiver = "Barclays"
	}

	transaction.Tags = map[string]string{}
	transaction.Tags["bank"] = "Barclays"
	transaction.Tags["beschreibung"] = columns[4]
	transaction.Tags["betrag"] = columns[3]
	transaction.Tags["buchungsdatum_1"] = columns[1]
	transaction.Tags["buchungsdatum_2"] = columns[2]
	transaction.Tags["kartennetzwerk"] = columns[12]
	transaction.Tags["kartennummer"] = columns[7]
	transaction.Tags["kontaktlose_bezahlung"] = columns[13]
	transaction.Tags["land"] = columns[10]
	transaction.Tags["mögliche_zahlpläne"] = columns[9]
	transaction.Tags["name_des_karteninhabers"] = columns[11]
	transaction.Tags["originalbetrag"] = columns[8]
	transaction.Tags["referenznummber"] = columns[0]
	transaction.Tags["status"] = columns[6]
	transaction.Tags["typ"] = columns[5]

	transaction.Budgets = map[string]int{}
	transaction.Inbudgets = map[string]int{}
	transaction.Debts = map[string]int{}

	return transaction, nil
}

func Barclays(data string) ([]transaction.Transaction, error) {
	transactions := []transaction.Transaction{}

	lines := strings.Split(data, "\n")
	for idx, line := range lines {
		if idx < 13 || len(line) == 0 {
			continue
		}
		transaction, err := parseBarclaysTransaction(line)
		if err != nil {
			return transactions, err
		}
		transactions = append(transactions, transaction)
	}

	return transactions, nil
}
