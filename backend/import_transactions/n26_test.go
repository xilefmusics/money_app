package import_transactions

import (
	"testing"
)

func TestN26(t *testing.T) {
	data := "\"Datum\",\"Empfänger\",\"Kontonummer\",\"Transaktionstyp\",\"Verwendungszweck\",\"Betrag (EUR)\",\"Betrag (Fremdwährung)\",\"Fremdwährung\",\"Wechselkurs\"\n\"2023-01-31\",\"Von A nach B\",\"\",\"Überweisung\",\"Von A nach B\",\"-123.45\",\"\",\"\",\"\"\n\"2023-02-01\",\"Von B nach A\",\"\",\"Gutschrift\",\"Von B nach A\",\"0.01\",\"\",\"\",\"\"\n\"2023-02-01\",\"Someone\",\"DE12345678910111213141\",\"Lastschrift\",\"Das ist ein Verwendungszweck\",\"-12.34\",\"\",\"\",\"\"\n\"2023-02-01\",\"Someone\",\"DE12121212121212121212\",\"Überweisung\",\"Ein anderer Verwendungszweck\",\"-123.0\",\"\",\"\",\"\"\n"
	_, err := N26(data)
	if err != nil {
		t.Fatalf(err.Error())
	}
}
