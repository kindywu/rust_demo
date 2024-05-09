fn main() {
    demo1();
    demo2();
    demo3();
    demo4();
    demo5();
}

fn demo1() {
    let a = String::from("hello");
    let b = a;
    // println!("{} {}", a, b) //borrow of moved value: `a` value borrowed here after move
    println!("demo1 => {}", b)
}

fn demo2() {
    let a = 1024;
    let b = a;
    println!("demo2 => {} {}", a, b);
}

#[derive(Debug, Clone, Copy)]
struct MyData {
    value: i32,
}

fn demo3() {
    let a = MyData { value: 32 };
    let b = a;
    // println!("{:?} {:?}", a, b);
    println!("demo3 => {:?} {:?}", a.value, b.value); // borrow of moved value: `a` value borrowed here after move
}

#[derive(Debug, Clone)]
struct MyData2 {
    value: i32,
    value2: Vec<i32>, //the trait `Copy` cannot be implemented for this type
}

// Vec is not defined in the current crate
// Vec implements Drop
// Copy and Drop are mutually exclusive
// impl Copy for Vec<i32> {}

#[derive(Debug, Clone, Copy)]
struct MyData3<'a> {
    value: i32,
    value2: &'a Vec<i32>, //the trait `Copy` cannot be implemented for this type
}

fn demo4() {
    let a = MyData2 {
        value: 32,
        value2: vec![10, 11],
    };
    let b = a.clone(); // +-
    println!(
        "demo4 => {} {:?} {} {:?}",
        a.value, a.value2, b.value, b.value2
    );
}

fn demo5() {
    let a = MyData3 {
        value: 32,
        value2: &vec![10, 11],
    };
    let b = a.clone(); // +-
    println!(
        "demo5 => {} {:?} {} {:?}",
        a.value, a.value2, b.value, b.value2
    );
}
