// metrics data structure
// 基本功能: inc/dec/snashot

use std::{
    fmt,
    fmt::{Debug, Display},
    sync::Arc,
};

use dashmap::DashMap;

use anyhow::Result;
#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }
}

impl Metrics {
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
