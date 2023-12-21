use chrono::{Duration, Local, TimeZone};

#[derive(Default)]
pub struct Filter<'a> {
    year: Option<i32>,
    month: Option<u32>,
    pod: Option<&'a str>,
    debt: Option<&'a str>,
    budget: Option<&'a str>,
    inbudget: Option<&'a str>,
    ttype: Option<&'a str>,
}

impl<'a> Filter<'a> {
    pub fn new(
        year: Option<i32>,
        month: Option<u32>,
        pod: Option<&'a str>,
        debt: Option<&'a str>,
        budget: Option<&'a str>,
        inbudget: Option<&'a str>,
        ttype: Option<&'a str>,
    ) -> Self {
        Self {
            year,
            month,
            pod,
            debt,
            budget,
            inbudget,
            ttype,
        }
    }

    pub fn year(mut self, year: i32) -> Self {
        self.year = Some(year);
        self
    }

    pub fn month(mut self, month: u32) -> Self {
        self.month = Some(month);
        self
    }

    pub fn pod(mut self, pod: &'a str) -> Self {
        self.pod = Some(pod);
        self
    }

    pub fn debt(mut self, debt: &'a str) -> Self {
        self.debt = Some(debt);
        self
    }

    pub fn budget(mut self, budget: &'a str) -> Self {
        self.budget = Some(budget);
        self
    }

    pub fn inbudget(mut self, inbudget: &'a str) -> Self {
        self.inbudget = Some(inbudget);
        self
    }

    pub fn ttype(mut self, ttype: &'a str) -> Self {
        self.ttype = Some(ttype);
        self
    }

    pub fn conditions(&self) -> Vec<String> {
        let mut conditions = Vec::new();

        if let Some(year) = self.year {
            let from = if let Some(month) = self.month {
                Local.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap()
            } else {
                Local.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap()
            };
            let to = if let Some(month) = self.month {
                Local.with_ymd_and_hms(year, month + 1, 1, 0, 0, 0).unwrap()
            } else {
                Local.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0).unwrap()
            };
            conditions.push(format!(
                "content.date > \"{}\" AND content.date < \"{}\"",
                from, to
            ))
        }

        if let Some(pod) = self.pod {
            conditions.push(format!(
                "content.sender == \"{}\" OR content.receiver == \"{}\"",
                pod, pod
            ))
        }
        if let Some(debt) = self.debt {
            conditions.push(format!("content.debts[\"{}\"]", debt))
        }
        if let Some(budget) = self.budget {
            conditions.push(format!("content.budgets[\"{}\"]", budget))
        }
        if let Some(inbudget) = self.inbudget {
            conditions.push(format!("content.inbudget[\"{}\"]", inbudget))
        }
        if let Some(ttype) = self.ttype {
            conditions.push(format!("content.type == \"{}\"", ttype))
        }

        conditions
    }
}
