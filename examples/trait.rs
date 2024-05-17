trait Size {
    fn area(&self) -> f64;
}

struct Rectangle {
    x: f64,
    y: f64,
}

impl Size for Rectangle {
    fn area(&self) -> f64 {
        self.x * self.y
    }
}

struct Circle {
    radius: f64,
}

impl Size for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn print_area(shape: &dyn Size) {
    println!("{} {}", stringify!(shape), shape.area())
}

fn print_area_2(shapes: Vec<&dyn Size>) {
    for shape in shapes {
        println!("{} {}", stringify!(shape), shape.area())
    }
}
fn print_area_3(shapes: &[&dyn Size]) {
    for shape in shapes {
        println!("{} {}", stringify!(shape), shape.area())
    }
}

fn print_area2(shape: &impl Size) {
    println!("{} {}", stringify!(shape), shape.area())
}

#[allow(dead_code)]
fn print_area2_2(shapes: &[impl Size]) {
    for shape in shapes {
        println!("{} {}", stringify!(shape), shape.area())
    }
}

fn print_area3<T>(shape: &T)
where
    T: Size,
{
    println!("{} {}", stringify!(shape), shape.area())
}

#[allow(dead_code)]
fn print_area3_2<T>(shapes: &[T])
where
    T: Size,
{
    for shape in shapes {
        println!("{} {}", stringify!(shape), shape.area())
    }
}

fn main() {
    let rect = Rectangle { x: 10.0, y: 20.0 };
    let circle = Circle { radius: 5.0 };
    print_area(&rect);
    print_area(&circle);

    print_area2(&rect);
    print_area2(&circle);

    print_area3(&rect);
    print_area3(&circle);

    print_area_2(vec![&rect, &circle]);

    print_area_3(&[&rect, &circle]);

    print_area2_2(&[circle]);

    print_area3_2(&[rect]);
}
