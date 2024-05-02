use anyhow::Ok;
use std::{sync::mpsc, thread, time::Duration};

const MUM_PRODUCTERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

#[allow(dead_code)]
impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Msg { idx, value }
    }
}

fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();
    for i in 0..MUM_PRODUCTERS {
        let tx = tx.clone();
        std::thread::spawn(move || {
            producer(i, tx).unwrap();
        });
    }
    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Got: {:?}", msg);
        }
        println!("consumer exit");
        42
    });

    //return value of thread
    let secret = consumer
        .join()
        .map_err(|err| anyhow::anyhow!("thread join error : {:?}", err))?;
    println!("secret: {}", secret);

    Ok(())
}

fn producer(id: usize, tx: mpsc::Sender<Msg>) -> anyhow::Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(id, value)).unwrap();
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 10 == 0 {
            println!("producer {} exit", id);
            break;
        }
    }
    Ok(())
}
