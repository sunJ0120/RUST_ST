use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct Store {
    data: Arc<Mutex<HashMap<String, String>>>,
    expiry: Arc<Mutex<HashMap<String, Instant>>>,
}

impl Store{
    pub fn new() -> Self {
        Store {
            data: Arc::new(Mutex::new(HashMap::new())),
            expiry: Arc::new(Mutex::new(HashMap::new())),
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

    // DEL 명령어
    pub fn del(&self, key: &str) -> i64 {
        let mut data = self.data
            .lock()
            .expect("🦀 락을 얻는데 실패하였습니다.");

        if data.remove(key).is_some(){
            1
        } else {
            0
        }
    }

    // EXPIRE
    pub fn expire(&self, key: &str, seconds: i64) -> i64 {
        let mut data = self.data
            .lock()
            .expect("🦀 락을 얻는데 실패하였습니다.");

        if !data.contains_key(key) {
            return 0;
        }

        drop(data); // lock 임시 해제

        if seconds <= 0 {
            self.del(key);
            return 1;
        }

        let mut expiry = self.expiry
            .lock()
            .expect("🦀 락을 얻는데 실패하였습니다.");

        let expire_time = Instant::now() + Duration::from_secs(seconds as u64);
        expiry.insert(key.to_string(), expire_time);

        1    // 성공
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

    #[test]
    fn test_del_existing_key() {
        let store = Store::new();
        store.set("key", "rudis");

        let result = store.del("key");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_del_nonexistent_key() {
        let store = Store::new();
        let result = store.del("not_exist");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_expire_basic(){
        let store = Store::new();
        store.set("key", "rudis");

        assert_eq!(store.expire("key", 10), 1);
    }

    #[test]
    fn test_expire_nonexistent(){
        let store = Store::new();
        assert_eq!(store.expire("not_exist", 10), 0);
    }
}