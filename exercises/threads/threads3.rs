use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(tx: mpsc::Sender<u32>) -> thread::JoinHandle<()> {
    let handle = thread::spawn(move || {
        let vals = vec![&1, &2, &3, &4, &5];
        for val in vals {
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    handle
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();


    let _ = send_tx(tx.clone());


    let _ = send_tx(tx);


    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }


    assert_eq!(total_received, queue.length);
    println!("total numbers received: {}", total_received);
}