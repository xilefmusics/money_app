use surrealdb::opt::RecordId;

pub fn record2string(record: &RecordId) -> String {
    format!("{}:{}", record.tb, record.id.to_string())
}
