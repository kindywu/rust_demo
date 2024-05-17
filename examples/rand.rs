use rand::prelude::SliceRandom;
use rand::Rng;

fn main() -> anyhow::Result<()> {
    // generates a boolean
    // Try printing a random unicode code point (probably a bad idea)!
    println!("{}", rand::random::<u16>());

    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen(); // generates a float between 0 and 1
    println!("{y}");

    let nums: Vec<i32> = (1..100).collect();
    let num = nums.choose(&mut rng).unwrap();
    println!("{num:?}");
    Ok(())
}
