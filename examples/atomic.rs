use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

#[tokio::main]
async fn main() {
    // let counter = Arc::new(AtomicUsize::new(0));
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = task::spawn(async move {
            for _ in 0..100 {
                // counter_clone.fetch_add(1, Ordering::SeqCst);
                let mut num = counter_clone.lock().await;
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    // println!("Final counter value: {}", counter.load(Ordering::SeqCst));
    println!("Final counter value: {}", counter.lock().await);
}
