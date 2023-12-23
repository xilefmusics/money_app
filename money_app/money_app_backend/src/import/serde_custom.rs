pub mod yyyy_mm_dd {
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

pub mod amount {
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
