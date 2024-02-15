pub mod yyyy_mm_dd_option {
    use chrono::{DateTime, Local, TimeZone};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Local>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer);
        if s.is_err() {
            return Ok(None);
        }
        let s = s.unwrap();

        let mut iter = s.split("-");

        let year = iter.next();
        if year.is_none() {
            return Ok(None);
        }
        let year = year.unwrap().parse::<i32>();
        if year.is_err() {
            return Ok(None);
        }
        let year = year.unwrap();
        let month = iter.next().unwrap_or("1").parse::<u32>().unwrap_or(1);
        let day = iter.next().unwrap_or("1").parse::<u32>().unwrap_or(1);

        Ok(Some(
            Local.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap(),
        ))
    }
}

pub mod yyyy_mm_dd {
    use chrono::{DateTime, Local, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = format!("{} 0:0:0", String::deserialize(deserializer)?);
        let dt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Local>::from_naive_utc_and_offset(
            dt,
            Local::now().offset().clone(),
        ))
    }
}

pub mod dd_mm_yyyy_option {
    use chrono::{DateTime, Local, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Local>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.len() == 0 {
            return Ok(None);
        }
        let s = format!("{} 0:0:0", s);
        let dt = NaiveDateTime::parse_from_str(&s, "%d.%m.%Y %H:%M:%S")
            .map_err(serde::de::Error::custom)?;
        Ok(Some(DateTime::<Local>::from_naive_utc_and_offset(
            dt,
            Local::now().offset().clone(),
        )))
    }
}

pub mod amount {
    use serde::{self, Deserialize, Deserializer};

    fn trim(s: &str) -> &str {
        let remove_first = s.starts_with("+");
        let remove_last = s.ends_with("€");
        let mut chars = s.chars();
        if remove_first {
            chars.next();
        }
        if remove_last {
            chars.next_back();
        }
        chars.as_str().trim()
    }

    fn handle_comma_dot(s: &str) -> String {
        if let Some(_) = s.find(",") {
            s.to_string().replace(".", "").replace(",", ".")
        } else {
            s.to_string()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<usize, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = handle_comma_dot(trim(&String::deserialize(deserializer)?));

        let mut iter = s.split(".");
        let before = iter.next().unwrap().parse::<i64>().unwrap();
        let after = iter.next().unwrap().parse::<i64>().unwrap();
        Ok((before.abs() * 100 + after) as usize)
    }
}
