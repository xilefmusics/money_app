use crate::error::AppError;
use crate::transaction::{Transaction, Type as TType};

use std::collections::HashMap;

use chrono::{DateTime, Local};
use csv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Barclays {
    #[serde(rename = "Referenznummer")]
    reference_number: String,
    #[serde(
        rename = "Buchungsdatum",
        with = "super::serde_custom::dd_mm_yyyy_option"
    )]
    date: Option<DateTime<Local>>,
    #[serde(
        rename = "Buchungsdatum2",
        with = "super::serde_custom::dd_mm_yyyy_option"
    )]
    date2: Option<DateTime<Local>>,
    #[serde(rename = "Betrag", with = "super::serde_custom::amount")]
    amount: usize,
    #[serde(rename = "Beschreibung")]
    description: String,
    #[serde(rename = "Typ")]
    typ: String,
    #[serde(rename = "Status")]
    state: String,
    #[serde(rename = "Kartennummer")]
    card_number: String,
    #[serde(rename = "Originalbetrag", with = "super::serde_custom::amount")]
    original_amount: usize,
    #[serde(rename = "Mögliche Zahlpläne")]
    possible_payments: String,
    #[serde(rename = "Land")]
    country: String,
    #[serde(rename = "Name des Karteninhabers")]
    card_holder: String,
    #[serde(rename = "Kartennetzwerk")]
    card_network: String,
    #[serde(rename = "Kontaktlose Bezahlung")]
    contactless: String,
}

impl Barclays {
    fn from_str(s: &str) -> Result<Vec<Self>, AppError> {
        let (_, data) = s.split_at(
            s.find("\nReferenznummer")
                .ok_or(AppError::Import("Parse Barclays: No Referenzummer".into()))?
                + 1,
        );
        let (first, second) = data.split_at(
            data.find(",Betrag")
                .ok_or(AppError::Import("Parse Barclays: No Betrag".into()))?,
        );
        let data = format!("{}2{}", first, second);

        Ok(csv::Reader::from_reader(data.as_bytes())
            .deserialize()
            .collect::<Result<Vec<Self>, csv::Error>>()?)
    }

    pub fn to_tags(self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("bank".into(), "Barclays".into());
        map.insert("beschreibung".into(), self.description);
        map.insert(
            "buchungsdatum_1".into(),
            self.date.map(|d| d.to_string()).unwrap_or("".into()),
        );
        map.insert(
            "buchungsdatum_2".into(),
            self.date2.map(|d| d.to_string()).unwrap_or("".into()),
        );
        map.insert("kartennetzwerk".into(), self.card_network);
        map.insert("kartennummer".into(), self.card_number);
        map.insert("kontaktlose_bezahlung".into(), self.contactless);
        map.insert("land".into(), self.country);
        map.insert("mögliche_zahlpläne".into(), self.possible_payments);
        map.insert("name_des_karteninhabers".into(), self.card_holder);
        map.insert("originalbetrag".into(), self.original_amount.to_string());
        map.insert("referenznummber".into(), self.reference_number);
        map.insert("typ".into(), self.typ);
        map
    }

    pub fn to_transaction(self) -> Result<Transaction, AppError> {
        let (ttype, sender, receiver) = if self.typ == "Belastung" {
            (TType::Out, Some("Barclays".into()), None)
        } else {
            (TType::In, None, Some("Barclays".into()))
        };

        Ok(Transaction {
            id: None,
            ttype,
            date: self
                .date
                .ok_or(AppError::Import("Parse Barclays: No date".into()))?,
            amount: self.amount,
            sender,
            receiver,
            budgets: HashMap::new(),
            inbudgets: HashMap::new(),
            debts: HashMap::new(),
            tags: self.to_tags(),
            attachment: None,
        })
    }

    pub fn parse_transactions(s: &str) -> Result<Vec<Transaction>, AppError> {
        Self::from_str(s)?
            .into_iter()
            .map(|barclays| barclays.to_transaction())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
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
        dbg!(Barclays::parse_transactions(input));
        panic!("Hello World")
    }
}
