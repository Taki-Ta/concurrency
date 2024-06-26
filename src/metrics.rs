use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Error;

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut data = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!("lock error: {:?}", e))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>, Error> {
        let data = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!("lock error: {:?}", e))?;
        Ok(data.clone())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
