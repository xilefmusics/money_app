use crate::error::AppError;
use crate::transaction::Transaction;

use super::Barclays;
use super::N26;

pub fn import(s: &str) -> Result<Vec<Transaction>, AppError> {
    if let Some(_) = s.find("Referenznummer,Buchungsdatum,Buchungsdatum,Betrag,Beschreibung,Typ,Status,Kartennummer,Originalbetrag,Mögliche Zahlpläne,Land,Name des Karteninhabers,Kartennetzwerk,Kontaktlose Bezahlung") {
        Barclays::parse_transactions(s)
    } else {
        N26::parse_transactions(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n26() {
        let input = "\"Datum\",\"Empfänger\",\"Kontonummer\",\"Transaktionstyp\",\"Verwendungszweck\",\"Betrag (EUR)\",\"Betrag (Fremdwährung)\",\"Fremdwährung\",\"Wechselkurs\"
\"2023-12-01\",\"Receiver 1\",\"DE123...456\",\"Überweisung\",\"Transaction 1\",\"-123.4\",\"\",\"\",\"\"
\"2023-12-02\",\"Receiver 2\",\"DE123...456\",\"Lastschrift\",\"Transaction 2\",\"-8.99\",\"\",\"\",\"\"
";
        dbg!(import(input));
    }

    #[test]
    fn test_barclays() {
        let input = "
,,,,,,,,,,,,,
Transaktionsansicht,,,,,,,,,,,,,
,,,,,,,,,,,,,
Kontonummer,123456,,,,,,,,,,,,
IBAN,DE123456,,,,,,,,,,,,
Kontoname,Barclays Visa,,,,,,,,,,,,
Kontotyp,Kreditkarte,,,,,,,,,,,,
Kreditrahmen,\"1.2345,00\",,,,,,,,,,,,
Verfügungsrahmen,\"1.000,00\",,,,,,,,,,,,
Status,Aktiv,,,,,,,,,,,,
Stand,23 Dez 23,,,,,,,,,,,,
,,,,,,,,,,,,,
Referenznummer,Buchungsdatum,Buchungsdatum,Betrag,Beschreibung,Typ,Status,Kartennummer,Originalbetrag,Mögliche Zahlpläne,Land,Name des Karteninhabers,Kartennetzwerk,Kontaktlose Bezahlung
123456,01.12.2023,,\"+1,23 €\",Transaction 1,Überweisung,vorgemerkt,9876,\"+1,23 €\",Nein,DE,Name,Visa,Ja
123457,02.12.2023,03.12.2023,\"-1,23 €\",Transaction2,Belastung,noch nicht abgerechnet,5432,\"-1,23 €\",Nein,DE,Name,Visa,Nein
";
        dbg!(import(input));
    }
}
