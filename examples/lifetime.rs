struct Person<'a> {
    name: &'a str,
    age: u32,
}

fn describe<'a>(person: &'a Person<'a>) {
    println!("Name: {}, Age: {}", person.name, person.age);
}

fn main() {
    let name = "Alice";
    let person = Person { name, age: 30 };

    describe(&person);
}
