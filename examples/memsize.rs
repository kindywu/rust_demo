use std::{
    io::Write,
    mem::{align_of_val, size_of_val},
    rc::Rc,
};

fn main() {
    let a: (char, u8, i32) = ('a', 7, 356); // 4+1+4=9
    println!("{}", size_of_val(&a)); // 12
    println!("{}", align_of_val(&a)); // 4

    let mut i = 1;
    let mut f = || i += 1;
    f();
    println!("{}", size_of_val(&f));

    let msg = String::from("hello");
    let f = || println!("{msg}");
    println!("{}", size_of_val(&f));

    let msg = String::from("hello");
    let f = || drop(msg);
    println!("{}", size_of_val(&f));

    let s = String::new();
    println!("{}", size_of_val(&s));

    let v = vec!["odin".to_string(), "thor".to_string(), "loki".to_string()];

    println!("{}", size_of_val(&v));

    let v = Rc::new(vec![
        "odin".to_string(),
        "thor".to_string(),
        "loki".to_string(),
    ]);

    println!("{}", size_of_val(&v));

    let mut buffer: Vec<u8> = vec![];
    let w: &mut dyn Write = &mut buffer;
    println!("{}", size_of_val(&w));

    let buffer: Vec<u8> = vec![];
    let w: Box<dyn Write> = Box::new(buffer);
    println!("{}", size_of_val(&w));

    let buffer: Vec<u8> = vec![];
    let w: Rc<dyn Write> = Rc::new(buffer);
    println!("{}", size_of_val(&w));
}
