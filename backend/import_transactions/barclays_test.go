package import_transactions

import (
	"testing"
)

func TestBarclays(t *testing.T) {
	data := ",,,,,,,,,,,,,\nTransaktionsansicht,,,,,,,,,,,,,\n,,,,,,,,,,,,,\nKontonummer,1234567890,,,,,,,,,,,,\nIBAN,DE12345678910111213141,,,,,,,,,,,,\nKontoname,Barclays Visa,,,,,,,,,,,,\nKontotyp,Kreditkarte,,,,,,,,,,,,\nKreditrahmen,\"1.234,56\",,,,,,,,,,,,\nVerfügungsrahmen,\"1.234,56\",,,,,,,,,,,,\nStatus,Aktiv,,,,,,,,,,,,\nStand,02 Feb 22,,,,,,,,,,,,\n,,,,,,,,,,,,,\nReferenznummer,Buchungsdatum,Buchungsdatum,Betrag,Beschreibung,Typ,Status,Kartennummer,Originalbetrag,Mögliche Zahlpläne,Land,Name des Karteninhabers,Kartennetzwerk,Kontaktlose Bezahlung\n123456789101112,02.02.2022,,\"-42,42 €\",Verwendungszweck,Belastung,vorgemerkt,123456******1234,\"-42,42 €\",Nein,,Mein Name,Visa,Ja\n"
	_, err := Barclays(data)
	if err != nil {
		t.Fatalf(err.Error())
	}
}
