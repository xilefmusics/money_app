use super::TransactionsIterator;

use crate::transaction::Transaction;

use chrono::{DateTime, Datelike, Days, Local, Months, TimeZone};

pub struct DateIterator {
    current: DateTime<Local>,
    year: u32,
    month: u32,
    day: u64,
    left: u32,
}

impl DateIterator {
    pub fn new(year: u32, month: u32, day: u64, len: u32) -> Self {
        let mut current = Local::now();

        if day == 0 && year == 0 {
            current = Local
                .with_ymd_and_hms(current.year() + 1, 1, 1, 0, 0, 0)
                .unwrap();
        } else if day == 0 {
            current = Local
                .with_ymd_and_hms(current.year(), current.month(), 1, 0, 0, 0)
                .unwrap()
                + Months::new(1);
        }
        current = current - Months::new(len * (month + 12 * year)) - Days::new(len as u64 * day);

        Self {
            current,
            year,
            month,
            day,
            left: len,
        }
    }

    pub fn into_transactions_iter<'a>(
        self,
        transactions: &'a [Transaction],
    ) -> TransactionsIterator<'a> {
        TransactionsIterator::new(self, transactions)
    }
}

impl Iterator for DateIterator {
    type Item = DateTime<Local>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left == 0 {
            None
        } else {
            self.left -= 1;
            let last = self.current;
            self.current =
                self.current + Months::new(self.month + 12 * self.year) + Days::new(self.day);
            Some(self.current)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year() {
        let year = Local::now().year();
        let mut dates = DateIterator::new(1, 0, 0, 3);
        assert_eq!(
            dates.next(),
            Some(Local.with_ymd_and_hms(year - 1, 1, 1, 0, 0, 0).unwrap()),
        );
        assert_eq!(
            dates.next(),
            Some(Local.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap()),
        );
        assert_eq!(
            dates.next(),
            Some(Local.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap(),),
        );
    }
}
