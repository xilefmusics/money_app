use chrono::{DateTime, Datelike, Local, Months, TimeZone};

pub struct DateIterator {
    current: DateTime<Local>,
    year: i32,
}

impl DateIterator {
    pub fn new(year: i32) -> Self {
        let current = Local.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();

        Self { current, year }
    }
}

impl Iterator for DateIterator {
    type Item = DateTime<Local>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.year() > self.year {
            None
        } else {
            let current = self.current;
            self.current = self.current + Months::new(1);
            Some(current)
        }
    }
}
