package import_transactions

import (
	"encoding/json"
	"errors"
	"strings"

	"xilefmusics.de/money-app/transaction"
)

func Import(data string) ([]transaction.Transaction, error) {
	lines := strings.Split(data, "\n")

	var transactions []transaction.Transaction
	var err error

	if len(lines) > 1 && lines[0] == "\"Datum\",\"Empfänger\",\"Kontonummer\",\"Transaktionstyp\",\"Verwendungszweck\",\"Betrag (EUR)\",\"Betrag (Fremdwährung)\",\"Fremdwährung\",\"Wechselkurs\"" {
		transactions, err = N26(data)
		if err != nil {
			return nil, err
		}
	} else if len(lines) > 13 && lines[12] == "Referenznummer,Buchungsdatum,Buchungsdatum,Betrag,Beschreibung,Typ,Status,Kartennummer,Originalbetrag,Mögliche Zahlpläne,Land,Name des Karteninhabers,Kartennetzwerk,Kontaktlose Bezahlung" {
		transactions, err = Barclays(data)
		if err != nil {
			return nil, err
		}
	} else if data[0] == '[' {
		err = json.Unmarshal([]byte(data), &transactions)
	} else {
		return nil, errors.New("Error: Unknown data type in import transactions")
	}

	return transactions, err
}
