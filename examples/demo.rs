fn main() {
    // mem_size();
    let mut v = vec![1, 2, 3];
    let n = &v[0];
    println!("{n}");
    v.push(0);
    // let x = *n;
    println!("{v:?}")
}

#[allow(dead_code)]
fn mem_size() {
    let size_k = 1024;
    let size_m = size_k * 1024;
    let size_g = size_m * 1024;
    println!("{size_k} {size_m} {size_g}");
    println!("{}", 8 * size_m);
    let arr: [u8; 8000000] = [0; 8000000];
    println!("{}", std::mem::size_of_val(&arr))
}
