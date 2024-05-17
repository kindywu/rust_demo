#[derive(Debug)]
enum Value {
    Null,
    Bool(bool),
}

fn main() {
    println!("{:?}", Value::Null);
    let value = Value::Bool(true);
    match value {
        Value::Null => todo!(),
        Value::Bool(b) => println!("{b}"),
    }
}
