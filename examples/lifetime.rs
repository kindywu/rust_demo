struct Person<'a, 'b> {
    name: &'a str,
    desc: &'b str,
    age: u32,
}

fn describe<'a, 'b>(person: &Person<'a, 'b>) {
    println!(
        "Name: {}, Desc: {}, Age: {}",
        person.name, person.desc, person.age
    );
}

fn main() {
    let name = "Alice";
    let person = Person {
        name,
        desc: "Good Girl",
        age: 30,
    };

    describe(&person);
}
