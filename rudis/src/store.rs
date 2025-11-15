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
}