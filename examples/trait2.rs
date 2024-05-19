trait Add<T> {
    fn add(self, rhs: T) -> Self;
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add<Point> for Point {
    fn add(self, rhs: Point) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<i32> for Point {
    fn add(self, rhs: i32) -> Self {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

fn main() {
    let a = Point { x: 10, y: 20 };
    let b = Point { x: 10, y: 20 };
    let c = a.add(b);
    println!("{c:?}");
    let d = c.add(5);
    println!("{d:?}");
}
