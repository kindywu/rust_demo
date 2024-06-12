use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(1), Condvar::new()));
    let pair_2 = Arc::clone(&pair);
    let pair_3 = Arc::clone(&pair);

    // Inside of our lock, spawn a new thread, and then wait for it to start.
    thread::spawn(move || {
        for i in 1..=3 {
            let (lock, cvar) = &*pair_3;
            let mut started = lock.lock().unwrap();

            sleep(Duration::from_secs(1));
            println!("{i}: working");

            *started = i;
            cvar.notify_one();
            println!("notify_one, start= {}", *started);

            drop(started); // Explicitly drop the lock before the next iteration

            sleep(Duration::from_secs(1));
        }

        let (lock, cvar) = &*pair_2;
        let mut started = lock.lock().unwrap();
        *started = -1;
        cvar.notify_one();
        println!("notify_one, start= {}", *started);
    });

    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;

    loop {
        let mut started = lock.lock().unwrap();
        println!("waiting");
        started = cvar.wait(started).unwrap();
        println!("started: {}", *started);

        if *started < 0 {
            break;
        }
        drop(started);
    }
    println!("worked");
}
