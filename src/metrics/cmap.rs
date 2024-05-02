use anyhow::Ok;
use dashmap::DashMap;
use std::{fmt, sync::Arc};

//使用Rwlock Mutex实现并发访问
//          读     写
//Rwlock  .read() .write()
//Mutex   .lock() .lock()
//DashMap封装了对Rwlock的读写操作

#[derive(Debug, Clone)]
pub struct CMapMetrics {
    data: Arc<DashMap<String, i64>>,
}

impl CMapMetrics {
    pub fn new() -> Self {
        CMapMetrics {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    // pub fn snapshot(&self) -> Result<HashMap<String, i64>, Error> {
    //     let data = self
    //         .data
    //         .lock()
    //         .map_err(|e| anyhow::anyhow!("lock error: {:?}", e))?;
    //     Ok(data.clone())
    // }
}

impl Default for CMapMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CMapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}:{} ", entry.key(), entry.value())?;
        }
        std::fmt::Result::Ok(())
    }
}
