// src/storage/cache.rs

use redis::Commands;
use std::time::Duration;

/// Manages cache operations using Redis.
pub struct Cache {
    client: redis::Client,
}

impl Cache {
    /// Initializes a new cache connection.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the Redis server.
    ///
    /// # Returns
    ///
    /// * `Cache` - A new instance of the Cache.
    pub fn new(url: &str) -> Self {
        let client = redis::Client::open(url).expect("Failed to connect to Redis");
        Cache { client }
    }

    /// Sets a value in the cache with a specified key and optional expiration.
    ///
    /// # Arguments
    ///
    /// * `key` - The key under which the value will be stored.
    /// * `value` - The value to store in the cache.
    /// * `expiration` - An optional expiration duration.
    pub fn set_value(&self, key: &str, value: &str, expiration: Option<Duration>) {
        let mut con = self.client.get_connection().expect("Failed to connect to Redis");
        let _: () = con.set(key, value).expect("Failed to set value in Redis");

        if let Some(expire) = expiration {
            let _: () = con.expire(key, expire.as_secs() as usize).expect("Failed to set expiration in Redis");
        }
    }

    /// Retrieves a value from the cache by its key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the value to retrieve.
    ///
    /// # Returns
    ///
    /// * `Option<String>` - The value if found, otherwise `None`.
    pub fn get_value(&self, key: &str) -> Option<String> {
        let mut con = self.client.get_connection().expect("Failed to connect to Redis");
        con.get(key).unwrap_or(None)
    }

    /// Deletes a value from the cache by its key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the value to delete.
    pub fn delete_value(&self, key: &str) {
        let mut con = self.client.get_connection().expect("Failed to connect to Redis");
        let _: () = con.del(key).expect("Failed to delete value in Redis");
    }
}
