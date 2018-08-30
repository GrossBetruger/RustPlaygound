use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {
    let handle = thread::spawn(||
    for i in 1..11 {
        println!("hello number: {} from spawned thread", i);
        thread::sleep(Duration::from_millis(100));
    });

    for i in 1..6 {
        println!("hello number: {} from main thread", i);
        thread::sleep(Duration::from_millis(100));
    }

    handle.join().unwrap(); // main thread will wait for handle to finish

    let v = vec![1, 2, 3];

    // passing ownership of a vector to a clojure with 'move'
    thread::spawn(move || {
        println!("here's a nice vector: {:?}", v);
    }).join().expect("vec printing job failed");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let message = String::from("yo yo!");
        tx.send(message).unwrap();
    });

    let received_msg = rx.recv()
        .expect("failed to retrieve message from channel");

    println!("message: '{}' received loud and clear!", received_msg);


    let (tx, rx) = mpsc::channel(); // creating new transmitter and receiver

    thread::spawn(move || {
        for i in 1..6 {
            tx.send(format!("msg {}", i)).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    for rec in rx {
        println!("postponed message received: '{}'", rec);
    }

    let (tx, rx) = mpsc::channel(); // creating new transmitter and receiver


    thread::sleep(Duration::from_secs(1));

    // using the multi-producer single consumer property:
    let tx1 = tx.clone();

    thread::spawn(move || {
        let messages = vec!["hello", "from"];

        for msg in messages {
            tx.send(msg).unwrap(); // first producer
            thread::sleep(Duration::from_millis(700));
        }
    });

    thread::sleep(Duration::from_secs(3));

    thread::spawn(move || {
        let messages = vec!["the", "other", "side"];

        for msg in messages {
            tx1.send(msg).unwrap(); // second producer
            thread::sleep(Duration::from_millis(700))
        }
    });

    for rec in rx {
        println!("{}", rec);
    }

}
