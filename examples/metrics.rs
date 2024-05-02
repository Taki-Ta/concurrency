use anyhow::{Ok, Result};
use concurrency::Metrics;
use rand::Rng;
use std::{thread, time::Duration};

const N: usize = 2;
const M: usize = 4;

fn main() -> anyhow::Result<()> {
    let metrics = Metrics::new();
    println!("{:?}", metrics.snapshot());

    for i in 0..N {
        task_worker(i, metrics.clone())?;
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        std::thread::sleep(Duration::from_secs(5));
        println!("metrics={:?}", metrics.snapshot());
    }
}

fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        std::thread::sleep(Duration::from_millis(rng.gen_range(1000..1500)));
        metrics.inc(format!("task-{}", idx)).unwrap();
    });
    Ok(())
}

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(1000..1500)));
            let page = rng.gen_range(1..256);
            metrics.inc(format!("req.page-{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });
    Ok(())
}
