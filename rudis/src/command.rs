use crate::Store;

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
    // CLI에서 입력된 문자열을 Command enum으로 파싱하는 함수
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

    // 명령어를 실행하는 함수
    pub fn execute(&self, store: &Store) -> String {
        match self {
            Command::Set { key, value } => {
                store.set(key.as_str(), value.as_str())
            }

            Command::Get { key} => {
                match store.get(key.as_str()) {
                    Some(value) => value,
                    None => "(nil)".to_string(),
                }
            }

            Command::Del { key} => {
                store.del(key.as_str()).to_string()
            }

            Command::Expire { key, seconds } => {
                store.expire(key.as_str(), *seconds).to_string()
            }

            Command::Ttl { key } => {
                store.ttl(key.as_str()).to_string()
            }

            Command::Exit => {
                "🦀 Rudis를 종료합니다.".to_string()
            }

            Command::Unknown => {
                "👀 알 수 없는 명령어 입니다.".to_string()
            }
        }
    }
}