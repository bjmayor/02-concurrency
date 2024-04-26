// metrics data structure
// 基本功能: inc/dec/snashot

use std::{
    collections::HashMap,
    fmt,
    fmt::{Debug, Display},
    sync::{Arc, RwLock},
};

use anyhow::Result;
#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Metrics {
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        let data = self
            .data
            .read()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        Ok(data.clone())
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let data = self.data.read().map_err(|_e| fmt::Error {})?;
        for (key, value) in data.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}
