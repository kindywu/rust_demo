fn main() {
    // 一个所有权型变量的可变引用与不可变引用的作用域不能交叠，也可以说不能同时存在。
    // 同一个所有权型变量的可变借用之间的作用域也不能交叠。
    let mut s = String::from("hello");
    let s2 = &mut s;
    s2.push_str(" world");
    s.push('!');
    println!("{s}");
}
