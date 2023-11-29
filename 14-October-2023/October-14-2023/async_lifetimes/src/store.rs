use async_std::sync::{Arc, RwLock};
use std::{borrow::Cow, collections::HashMap};

use crate::{MemdbError, MemdbResult};

pub struct Store {
    /// Simulate a database
    memdb: Arc<RwLock<HashMap<String, String>>>,
}

impl Store {
    pub fn init() -> Self {
        Self {
            memdb: Arc::new(RwLock::new(HashMap::default())),
        }
    }

    pub async fn create(&mut self, key: &str, value: &str) -> MemdbResult<&str> {
        if self.memdb.read().await.get(key).is_some() {
            return Err(MemdbError::KeyAlreadyExists);
        } else {
            self.memdb
                .write()
                .await
                .insert(key.to_owned(), value.to_owned());

            Ok(value)
        }
    }

    pub async fn read(&self, key: &str) -> MemdbResult<Cow<str>> {
        match self.memdb.read().await.get(key) {
            Some(value) => Ok(Cow::Owned(value.to_owned())),
            None => Err(MemdbError::ValueNotFound),
        }
    }
}
