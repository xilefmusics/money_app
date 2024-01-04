use chrono::{DateTime, Datelike, Local, Months, TimeZone};

pub struct DateIterator {
    current: DateTime<Local>,
}

impl DateIterator {
    pub fn new() -> Self {
        let current = Local::now();
        let current = Local
            .with_ymd_and_hms(current.year(), current.month(), 1, 0, 0, 0)
            .unwrap();

        Self { current }
    }
}

impl Iterator for DateIterator {
    type Item = DateTime<Local>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current + Months::new(1);
        if next.year() > self.current.year() {
            None
        } else {
            self.current = next;
            Some(next)
        }
    }
}
