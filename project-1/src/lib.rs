use std::collections::HashMap;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Sets the value of a key to the specified value
    Set { key: String, value: String },
    /// Gets the value corresponding to a key
    Get { key: String },
    /// Removes a given key
    Rm { key: String },
}

#[derive(Debug)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Constructs a new KvStore instance.
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    /// Sets the value of a `key` to the provided `value`.
    /// If a record already exists for `key`, it will be overwritten.
    /// Otherwise, it will be newly inserted.
    ///
    /// # Arguments
    ///
    /// * `key`     - The key to either insert a new record against, or
    ///               replace an existing record.
    /// * `value`   - The value to assign to the new or existing key.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut kv_store = KvStore::new();
    ///
    /// // Inserting new record with key "foo" and value "bar".
    /// kv_store.set("foo", "bar");
    /// let foo = kv_store.get("foo").unwrap();
    /// assert_eq!(foo, String::from("bar"));
    ///
    /// // Overwriting "foo" with "other"
    /// kv_store.set("foo", "other");
    /// let foo = kv_store.get("foo").unwrap();
    /// assert_eq!(foo, String::from("other"));
    /// ```
    pub fn set(&mut self, key: &str, value: &str) {
        *self.store.entry(key.into()).or_insert(String::new()) = value.into()
    }

    /// Retrieves a value from the store corresponding to the given `key`.
    /// If no record exists for the given `key`, `None` will be returned.
    ///
    /// # Arguments
    ///
    /// * `key` - The key corresponding to a record to retrieve.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut kv_store = KvStore::new();
    ///
    /// kv_store.set("foo", "bar");
    ///
    /// let foo = kv_store.get("foo");
    /// let bar = kv_store.get("bar");
    ///
    /// assert!(foo.is_some());
    /// assert!(bar.is_none());
    /// ```
    ///
    pub fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).map(|value| value.into())
    }

    /// Removes a key/value pair from the store corresponding to the given `key`.
    /// If no record exists for the given `key`, this method will have no effect.
    ///
    /// # Arguments
    ///
    /// * `key` - The key corresponding to a record to remove.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut kv_store = KvStore::new();
    ///
    /// kv_store.set("foo", "bar");
    ///
    /// let foo = kv_store.get("foo");
    /// assert!(foo.is_some());
    ///
    /// kv_store.remove("foo");
    ///
    /// let foo = kv_store.get("foo");
    /// assert!(foo.is_none());
    /// ```
    pub fn remove(&mut self, key: &str) {
        self.store.remove(key);
    }
}
