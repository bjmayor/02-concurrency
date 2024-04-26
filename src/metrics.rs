// metrics data structure
// 基本功能: inc/dec/snashot

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Result;
#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Metrics {
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        let data = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        Ok(data.clone())
    }
}
