#[derive(Debug, PartialEq)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Del { key: String },
    Expire { key: String, seconds: i64 },
    Ttl { key: String },
}