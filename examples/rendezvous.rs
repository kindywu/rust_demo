use rendezvous::{Rendezvous, RendezvousGuard};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// A slow worker function. Sleeps, then mutates a value.
fn slow_worker_fn(guard: RendezvousGuard, value: Arc<Mutex<u32>>) {
    thread::sleep(Duration::from_millis(400));
    let mut value = value.lock().unwrap();
    *value = 42;
    guard.completed();
}

fn main() {
    // The guard that ensures synchronization across threads.
    // Rendezvous itself acts as a guard: If not explicitly dropped, it will block the current
    // scope until all rendezvous points are reached.
    let rendezvous = Rendezvous::new();

    // A value to mutate in a different thread.
    let value = Arc::new(Mutex::new(0u32));

    // Run the worker in a thread.
    thread::spawn({
        let guard = rendezvous.fork_guard();
        let value = value.clone();
        move || slow_worker_fn(guard, value)
    });

    // Block until the thread has finished its work.
    rendezvous.rendezvous();

    println!("{}", *(value.lock().unwrap()));
    // The thread finished in time.
    assert_eq!(*(value.lock().unwrap()), 42);
}
