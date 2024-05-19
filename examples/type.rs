use std::collections::HashMap;

type List<T> = Vec<T>;
type MapList<T> = HashMap<u32, List<T>>;

fn main() {
    let mut list = List::new();
    for i in 1..=10 {
        list.push(i)
    }
    println!("{list:?}");
    let key = 1;
    let mut map_list = MapList::new();
    // map_list.entry(key).or_insert(list);
    map_list.insert(key, list);
    println!("{map_list:?}");
}
