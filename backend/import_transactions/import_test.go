package import_transactions

import (
	"testing"
)

func TestImport(t *testing.T) {
	dataBarclays := ",,,,,,,,,,,,,\nTransaktionsansicht,,,,,,,,,,,,,\n,,,,,,,,,,,,,\nKontonummer,1234567890,,,,,,,,,,,,\nIBAN,DE12345678910111213141,,,,,,,,,,,,\nKontoname,Barclays Visa,,,,,,,,,,,,\nKontotyp,Kreditkarte,,,,,,,,,,,,\nKreditrahmen,\"1.234,56\",,,,,,,,,,,,\nVerfügungsrahmen,\"1.234,56\",,,,,,,,,,,,\nStatus,Aktiv,,,,,,,,,,,,\nStand,02 Feb 22,,,,,,,,,,,,\n,,,,,,,,,,,,,\nReferenznummer,Buchungsdatum,Buchungsdatum,Betrag,Beschreibung,Typ,Status,Kartennummer,Originalbetrag,Mögliche Zahlpläne,Land,Name des Karteninhabers,Kartennetzwerk,Kontaktlose Bezahlung\n123456789101112,02.02.2022,,\"-42,42 €\",Verwendungszweck,Belastung,vorgemerkt,123456******1234,\"-42,42 €\",Nein,,Mein Name,Visa,Ja\n"
	_, err := Import(dataBarclays)
	if err != nil {
		t.Fatalf(err.Error())
	}

	dataN26 := "\"Datum\",\"Empfänger\",\"Kontonummer\",\"Transaktionstyp\",\"Verwendungszweck\",\"Betrag (EUR)\",\"Betrag (Fremdwährung)\",\"Fremdwährung\",\"Wechselkurs\"\n\"2023-01-31\",\"Von A nach B\",\"\",\"Überweisung\",\"Von A nach B\",\"-123.45\",\"\",\"\",\"\"\n\"2023-02-01\",\"Von B nach A\",\"\",\"Gutschrift\",\"Von B nach A\",\"0.01\",\"\",\"\",\"\"\n\"2023-02-01\",\"Someone\",\"DE12345678910111213141\",\"Lastschrift\",\"Das ist ein Verwendungszweck\",\"-12.34\",\"\",\"\",\"\"\n\"2023-02-01\",\"Someone\",\"DE12121212121212121212\",\"Überweisung\",\"Ein anderer Verwendungszweck\",\"-123.0\",\"\",\"\",\"\"\n"
	_, err = Import(dataN26)
	if err != nil {
		t.Fatalf(err.Error())
	}

	dataJSON := "[{\"id\": 0,\"type\": \"in\",\"date\": \"2022-02-02T00:00:00.000Z\",\"amount\": 4200,\"sender\": \"\",\"receiver\": \"Receiver\",\"budgets\": {},\"inbudgets\": {},\"debts\": {},\"tags\": {}},{\"id\": 1,\"type\": \"out\",\"date\": \"2022-02-02T00:00:00.000Z\",\"amount\": 4200,\"sender\": \"Sender\",\"receiver\": \"\",\"budgets\": {},\"inbudgets\": {},\"debts\": {},\"tags\": {}}]"
	_, err = Import(dataJSON)
	if err != nil {
		t.Fatalf(err.Error())
	}

	//t.Fatalf("TODO\n")
}
