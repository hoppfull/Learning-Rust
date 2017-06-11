fn main() {
    let v0: Vec2 = Vec2 { x: 4., y: 3. };
    let v1: Vec2 = Vec2 { x: 8., y: -2. };
    let v2: Vec2 = v0 + v1;
    let v3: Vec2 = 3. * v2;
    let v4: Vec2 = v3 * 0.5;
    println!("Vec2 {{x:{}, y:{}}}", v0.x, v0.y);
    println!("Vec2 {{x:{}, y:{}}}", v1.x, v1.y);
    println!("Vec2 {{x:{}, y:{}}}", v2.x, v2.y);
    println!("Vec2 {{x:{}, y:{}}}", v3.x, v3.y);
    println!("Vec2 {{x:{}, y:{}}}", v4.x, v4.y);
}

#[derive(Clone, Copy)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, _rhs: Vec2) -> Vec2 {
        Vec2 { x: _rhs.x * self, y: _rhs.y * self }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, _rhs: f32) -> Vec2 {
        Vec2 { x: self.x * _rhs, y: self.y * _rhs }
    }
}
