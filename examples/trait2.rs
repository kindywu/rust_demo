trait Add<T: Sized> {
    type Output;
    fn add(self, rhs: T) -> Self::Output;
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<i32> for Point {
    type Output = Point;
    fn add(self, rhs: i32) -> Self {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Add<u32> for Point {
    type Output = u32;
    fn add(self, rhs: u32) -> u32 {
        (self.x + self.y) as u32 + rhs
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
