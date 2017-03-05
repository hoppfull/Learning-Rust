use std::fmt;

fn main() {
    let my_struct = MyStruct { a: 2., b: 79 };
    println!("{}", my_struct);
    println!("mag(&my_struct)={}", mag(&my_struct)); // 9
    let point = Point { x: -1.5, y: 10.5 };
    println!("mag(&point)={}", mag(&point)); // 3
}

fn mag(summable: &Sum) -> f32 {
    summable.sum().sqrt()
}

struct MyStruct {
    a: f32,
    b: i32,
}

struct Point {
    x: f32,
    y: f32,
}

trait Sum {
    fn sum(&self) -> f32;
}

impl Sum for MyStruct {
    fn sum(&self) -> f32 {
        self.a + (self.b as f32)
    }
}

impl Sum for Point {
    fn sum(&self) -> f32 {
        self.x + self.y
    }
}


// implementing trait to allow for printing:
impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyStruct {{ a: {}, b: {} }}", self.a, self.b)
    }
}
