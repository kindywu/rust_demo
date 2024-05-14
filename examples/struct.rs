use std::ops::Deref;

// #[derive(Clone, Copy)]
#[derive(Debug, Default)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl Default for Rectangle {
//     fn default() -> Self {
//         Self {
//             width: Default::default(),
//             height: Default::default(),
//         }
//     }
// }

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn area(self) -> u32 {
        self.width * self.height
    }

    fn area_ref(&self) -> u32 {
        self.width * self.height
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

struct PositiveI8(i8);
impl PositiveI8 {
    fn positive(&self) -> bool {
        self.0 > 0
    }
}

impl Deref for PositiveI8 {
    type Target = i8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

trait NewOperation {
    type Target;

    // 新增操作，返回Target类型的值
    fn double(self) -> Self::Target;
}

impl NewOperation for i32 {
    type Target = i32;

    fn double(self) -> Self::Target {
        self + self
    }
}

impl NewOperation for i8 {
    type Target = i8;

    fn double(self) -> Self::Target {
        self + self
    }
}

// trait NewOperation<T> {
//     fn double(self) -> T;
// }

// impl<T> NewOperation<T> for T
// where
//     T: Add<Output = T> + Clone,
// {
//     fn double(self) -> T {
//         self.clone() + self
//     }
// }

fn main() {
    let mut positive = PositiveI8(10);
    println!("{}", positive.positive());
    positive.0 = -1;
    println!("{}", positive.is_positive());
    let i32 = 99;
    println!("{}", i32.double());
    let i8: i8 = 12;
    println!("{}", i8.double());

    let rect: Rectangle = Default::default();
    println!("{rect:?}");
    let rect = Rectangle::default();
    println!("{rect:?}");
    let mut rect = Rectangle::new(10, 20);
    println!("rect size is {}", rect.area_ref());
    rect.resize(15, 25);
    println!("rect size is {}", rect.area());
    // println!("rect size is {}", rect.area()) //must uncomment the #[derive(Debug, Clone, Copy)]
}
