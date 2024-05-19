type List<T> = Vec<T>;

fn main() {
    let mut list = List::new();
    for i in 1..=10 {
        list.push(i)
    }

    println!("{list:?}")
}
