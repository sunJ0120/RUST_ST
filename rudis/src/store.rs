use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Store {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl Store{
    pub fn new() -> Self {
        Store {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // SET 명령어
    pub fn set(&self, key: &str, value: &str) -> String {
        let mut data = self.data
            .lock()
            .expect("🦀 락을 얻는데 실패하였습니다.");
        data.insert(key.to_string(), value.to_string());

        "OK".to_string()
    }

    // GET 명령어
    pub fn get(&self, key: &str) -> Option<String> {
        let data = self.data
            .lock()
            .expect("🦀 락을 얻는데 실패하였습니다.");

        data.get(key).cloned()
    }
}

// 테스트
#[cfg(test)]
mod tests {
    use super::Store;

    #[test]
    fn test_set(){
        let store = Store::new();
        store.set("key", "rudis");
        assert_eq!(store.get("key"), Some("rudis".to_string()));
    }

    #[test]
    fn test_get_nonexistent_key(){
        let store = Store::new();
        let result = store.get("not_exist");
        assert_eq!(result, None);
    }
}