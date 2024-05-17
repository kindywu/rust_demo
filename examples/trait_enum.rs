use enum_dispatch::enum_dispatch;

#[enum_dispatch]
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
#[enum_dispatch(Size)]
enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
}

// impl Size for Shape {
//     fn area(&self) -> f64 {
//         match &self {
//             Shape::Rectangle(rect) => rect.area(),
//             Shape::Circle(circle) => circle.area(),
//         }
//     }
// }

fn print_shape_list(shapes: &[Shape]) {
    for shape in shapes {
        println!("{}", shape.area())
    }
}

// 参数必须在编译时就确定大小
fn print_shape_list_2(shapes: [Shape; 2]) {
    for shape in shapes {
        println!("{}", shape.area())
    }
}

fn main() {
    let rect = Rectangle { x: 10.0, y: 20.0 };
    let circle = Circle { radius: 5.0 };

    let shapes = [Shape::Circle(circle), Shape::Rectangle(rect)];

    print_shape_list(&shapes);
    print_shape_list_2(shapes);
}
