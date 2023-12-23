use crate::error::AppError;
use crate::transaction::{Transaction, Type as TType};

use std::collections::HashMap;

use chrono::{DateTime, Local};
use csv;
use serde::Deserialize;

mod yyyy_mm_dd {
    use chrono::{DateTime, Local, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = format!("{} 0:0:0", String::deserialize(deserializer)?);
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Local>::from_naive_utc_and_offset(
            dt,
            Local::now().offset().clone(),
        ))
    }
}

mod amount {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(amount: usize, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}.{}", (amount / 100) as usize, amount % 100))
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<usize, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut iter = s.split(".");
        let before = iter.next().unwrap().parse::<i64>().unwrap();
        let after = iter.next().unwrap().parse::<i64>().unwrap();
        Ok((before.abs() * 100 + after) as usize)
    }
}

#[derive(Deserialize, Debug)]
pub enum Type {
    #[serde(rename = "Überweisung")]
    Transfer,
    #[serde(rename = "Gutschrift")]
    CreditNote,
    #[serde(rename = "Lastschrift")]
    Debit,
    #[serde(rename = "MasterCardZahlung")]
    CardPayment,
}

#[derive(Deserialize, Debug)]
pub struct N26 {
    #[serde(rename = "Datum", with = "yyyy_mm_dd")]
    date: DateTime<Local>,
    #[serde(rename = "Empfänger")]
    receiver: String,
    #[serde(rename = "Kontonummer")]
    account_number: String,
    #[serde(rename = "Transaktionstyp")]
    ttype: Type,
    #[serde(rename = "Verwendungszweck")]
    purpose: String,
    #[serde(rename = "Betrag (EUR)", with = "amount")]
    amount: usize,
    #[serde(rename = "Betrag (Fremdwährung)")]
    foreign_amount: Option<f64>,
    #[serde(rename = "Fremdwährung")]
    foreign_currency: Option<String>,
    #[serde(rename = "Wechselkurs")]
    exchange_rate: Option<f64>,
}

impl N26 {
    fn from_str(s: &str) -> Result<Vec<Self>, AppError> {
        Ok(csv::Reader::from_reader(s.as_bytes())
            .deserialize()
            .collect::<Result<Vec<Self>, csv::Error>>()?)
    }

    pub fn to_tags(self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("bank".into(), "n26".into());
        map.insert("iban".into(), self.account_number);
        map.insert("purpose_of_use".into(), self.purpose);
        map.insert("receiver".into(), self.receiver);
        map.insert(
            "foreign_currency".into(),
            self.foreign_currency.unwrap_or("".into()),
        );
        map.insert(
            "foreign_amount".into(),
            self.foreign_amount
                .map(|a| a.to_string())
                .unwrap_or("".into()),
        );
        map.insert(
            "exchange_rate".into(),
            self.exchange_rate
                .map(|a| a.to_string())
                .unwrap_or("".into()),
        );
        map
    }

    pub fn to_transaction(self) -> Result<Transaction, AppError> {
        let (ttype, sender, receiver) = if self.receiver.starts_with("Von") {
            let mut iter = self.receiver.split(" ");
            let von = iter
                .next()
                .ok_or(AppError::Import("Parse N26: no Von".to_string()))?;
            let sender = iter
                .next()
                .ok_or(AppError::Import("Parse N26: no sender".to_string()))?;
            let nach = iter
                .next()
                .ok_or(AppError::Import("Parse N26: no nach".to_string()))?;

            let receiver = iter
                .next()
                .ok_or(AppError::Import("Parse N26: no receiver".to_string()))?;

            Ok::<(TType, Option<String>, Option<String>), AppError>((
                TType::Move,
                Some(sender.to_string()),
                Some(receiver.to_string()),
            ))
        } else {
            Ok(match self.ttype {
                Type::CreditNote => (TType::In, None, Some("N26 Hauptkonto".to_string())),
                _ => (TType::Out, Some("N26 Hauptkonto".to_string()), None),
            })
        }?;

        Ok(Transaction {
            id: None,
            ttype,
            date: self.date,
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

    fn parse_transactions(s: &str) -> Result<Vec<Transaction>, AppError> {
        Self::from_str(s)?
            .into_iter()
            .map(|n26| n26.to_transaction())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let input = "\"Datum\",\"Empfänger\",\"Kontonummer\",\"Transaktionstyp\",\"Verwendungszweck\",\"Betrag (EUR)\",\"Betrag (Fremdwährung)\",\"Fremdwährung\",\"Wechselkurs\"
\"2023-12-01\",\"Receiver 1\",\"DE123...456\",\"Überweisung\",\"Transaction 1\",\"-123.4\",\"\",\"\",\"\"
\"2023-12-02\",\"Receiver 2\",\"DE123...456\",\"Lastschrift\",\"Transaction 2\",\"-8.99\",\"\",\"\",\"\"";
        dbg!(N26::parse_transactions(input));
    }
}
