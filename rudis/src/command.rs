#[derive(Debug, PartialEq)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Del { key: String },
    Expire { key: String, seconds: i64 },
    Ttl { key: String },
    Exit,
    Unknown,
}

impl Command {
    pub fn parse(input: &str) -> Result<Command, String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.as_slice() {
            ["SET", key, value] => Ok(Command::Set{
                key: key.to_string(),
                value: value.to_string(),
            }),

            ["GET", key] => Ok(Command::Get {
                key: key.to_string(),
            }),

            ["DEL", key] => Ok(Command::Del {
                key: key.to_string(),
            }),

            ["EXPIRE", key, seconds] => {
                match seconds.parse::<i64>() {
                    Ok(sec) => Ok(Command::Expire {
                        key: key.to_string(),
                        seconds: sec,
                    }),
                    Err(_) => Err("올바르지 않은 TTL 값 입니다.".to_string()),
                }
            }

            ["TTL", key] => Ok(Command::Ttl {
                key: key.to_string(),
            }),

            ["EXIT"] | ["exit"] | ["QUIT"] | ["quit"] => Ok(Command::Exit),

            [] => Err("빈 명령어 입니다.".to_string()),

            _ => Ok(Command::Unknown),
        }
    }
}