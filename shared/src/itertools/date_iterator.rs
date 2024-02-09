use chrono::{DateTime, Datelike, Days, Local, Months, TimeZone};

pub struct DateIterator {
    current: DateTime<Local>,
    year: u32,
    month: u32,
    day: u64,
    left: u32,
}

impl DateIterator {
    pub fn new(mut current: DateTime<Local>, year: u32, month: u32, day: u64, len: u32) -> Self {
        if day == 0 && month == 0 {
            current = Local
                .with_ymd_and_hms(current.year() + 1, 1, 1, 0, 0, 0)
                .unwrap();
        } else if day == 0 {
            current = Local
                .with_ymd_and_hms(current.year(), current.month(), 1, 0, 0, 0)
                .unwrap()
                + Months::new(1);
        } else {
            current = Local
                .with_ymd_and_hms(current.year(), current.month(), current.day(), 0, 0, 0)
                .unwrap()
                + Days::new(1);
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
}

impl Iterator for DateIterator {
    type Item = DateTime<Local>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left == 0 {
            None
        } else {
            self.left -= 1;
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
    fn date_iterator_year() {
        assert_eq!(
            DateIterator::new(
                Local.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
                1,
                0,
                0,
                3
            )
            .collect::<Vec<DateTime<Local>>>(),
            vec![
                Local.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
                Local.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
                Local.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            ]
        );
    }

    #[test]
    fn date_iterator_month() {
        assert_eq!(
            DateIterator::new(
                Local.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
                0,
                3,
                0,
                3
            )
            .collect::<Vec<DateTime<Local>>>(),
            vec![
                Local.with_ymd_and_hms(2023, 8, 1, 0, 0, 0).unwrap(),
                Local.with_ymd_and_hms(2023, 11, 1, 0, 0, 0).unwrap(),
                Local.with_ymd_and_hms(2024, 2, 1, 0, 0, 0).unwrap(),
            ]
        );
    }

    #[test]
    fn date_iterator_day() {
        assert_eq!(
            DateIterator::new(
                Local.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
                0,
                0,
                5,
                3
            )
            .collect::<Vec<DateTime<Local>>>(),
            vec![
                Local.with_ymd_and_hms(2023, 12, 23, 0, 0, 0).unwrap(),
                Local.with_ymd_and_hms(2023, 12, 28, 0, 0, 0).unwrap(),
                Local.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap(),
            ]
        );
    }
}
