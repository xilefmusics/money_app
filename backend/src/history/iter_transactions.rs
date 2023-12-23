use super::{DateIterator, WealthIterator};

use crate::transaction::Transaction;

use chrono::{DateTime, Local};

pub struct TransactionsIterator<'a> {
    date_iterator: DateIterator,
    transactions: &'a [Transaction],
}

impl<'a> TransactionsIterator<'a> {
    pub fn new(date_iterator: DateIterator, transactions: &'a [Transaction]) -> Self {
        Self {
            date_iterator,
            transactions,
        }
    }

    pub fn into_wealth_iter(self) -> WealthIterator<'a> {
        WealthIterator::new(self)
    }
}

impl<'a> Iterator for TransactionsIterator<'a> {
    type Item = (&'a [Transaction], DateTime<Local>);

    fn next(&mut self) -> Option<Self::Item> {
        let date = self.date_iterator.next()?;

        let (transactions, rest) = self.transactions.split_at(
            self.transactions
                .iter()
                .position(|transaction| transaction.date > date)
                .unwrap_or(self.transactions.len()),
        );
        self.transactions = rest;

        Some((transactions, date))
    }
}
