use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    m: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self { m: HashMap::new() }
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.m.get(&key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.m.insert(key, value);
    }

    pub fn remove(&mut self, key: String) {
        self.m.remove(&key);
    }
}
