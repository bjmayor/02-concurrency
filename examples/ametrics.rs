use anyhow::Result;
use concurrency::AmapMetrics;
use rand::Rng;
use std::thread;

const N: usize = 2;
const M: usize = 4;
fn main() -> Result<()> {
    let metrics = AmapMetrics::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "req.page.1",
        "req.page.2",
        "req.page.3",
        "req.page.4",
    ]);
    println!("{}", metrics);
    // start N worker and M requesters
    for idx in 0..N {
        task_work(idx, metrics.clone())?; // Arc::clone(&metrics.data)
    }

    for _ in 0..M {
        request_work(metrics.clone())?;
    }

    loop {
        thread::sleep(std::time::Duration::from_secs(2));
        println!("{}", metrics);
    }
}

fn task_work(idx: usize, metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..5000)));

            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_work(metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..5);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
