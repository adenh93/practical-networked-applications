use std::collections::HashMap;

#[derive(Debug)]
pub struct KvStore(HashMap<String, String>);

impl KvStore {
    /// Instantiates a new Key/Value store.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Sets the value of a `String` key to a `String`.
    pub fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_owned(), value.to_owned());
    }

    /// Gets the `String` value of a `String` key. If the key does not exist,
    // returns `None`.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|v| v.as_str())
    }

    /// Removes a given key.
    pub fn remove(&mut self, key: &str) {
        self.0.remove(key);
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
