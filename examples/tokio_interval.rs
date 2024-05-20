use std::time::Duration;

use tokio::time;

#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(3));
    loop {
        let instant = interval.tick().await;
        println!("{:?}", instant)
    }
}
