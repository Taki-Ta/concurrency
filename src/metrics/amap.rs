use anyhow::{anyhow, Ok};
use core::fmt;
use std::{
    collections::HashMap,
    fmt::{Formatter, Result},
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

//使用atomic实现并发访问
//          读     写
//Atomic  .load() .fetch_XXX()

pub struct AMapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AMapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let data = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        AMapMetrics {
            data: Arc::new(data),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> anyhow::Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow!("key {} not found", key))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl Clone for AMapMetrics {
    fn clone(&self) -> Self {
        AMapMetrics {
            data: self.data.clone(),
        }
    }
}

impl fmt::Display for AMapMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (key, value) in self.data.iter() {
            writeln!(f, "{}:{} ", key, value.load(Ordering::Relaxed))?;
        }
        std::fmt::Result::Ok(())
    }
}
