// use tokio::sync::{Semaphore, TryAcquireError};

// #[tokio::main]
// async fn main() {
//     let semaphore = Semaphore::new(3);

//     let _a_permit = semaphore.acquire().await.unwrap();
//     let _two_permits = semaphore.acquire_many(2).await.unwrap();

//     assert_eq!(semaphore.available_permits(), 0);

//     let permit_attempt = semaphore.try_acquire();
//     assert_eq!(permit_attempt.err(), Some(TryAcquireError::NoPermits));
// }

use std::{sync::Arc, time::Duration};
use tokio::{sync::Semaphore, time::sleep};

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = Vec::new();

    for _ in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            // perform task...
            // explicitly own `permit` in the task
            sleep(Duration::from_secs(1)).await;
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}
