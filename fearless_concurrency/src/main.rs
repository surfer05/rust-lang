use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Creating a New Thread with `spawn`
    // let handle = thread::spawn(|| {
    //     for i in 1..20 {
    //         println!("hi number {} from the spawned thread!", i);
    //         // thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // handle.join().unwrap();

    // for i in 1..10 {
    //     println!("hi number {} from the main thread!", i);
    //     // thread::sleep(Duration::from_millis(1));
    // }

    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {:?}", v);
    // });

    // drop(v);

    // handle.join().unwrap();

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    // let receivd = rx.recv().unwrap();
    // println!("Got : {}", receivd);

    for received in rx {
        println!("Got: {}", received);
    }

    let (t, r) = mpsc::channel();
    thread::spawn(move || {
        let s = String::from("Hello world");
        t.send(s.clone()).unwrap();
        t.send(s.len()).unwrap();
    });

    let s = r.recv().unwrap();
    let n = r.recv().unwrap();
    println!("{s}, {n}");
}
