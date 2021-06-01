pub struct Circle {
    pub r: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.r * self.r
    }
}

pub struct Square {
    pub s: f64
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.s * self.s
    }
}


pub trait Shape {
    fn area(&self) -> f64;
}

pub fn print_area<T: Shape>(s: &T) {
    println!("{}", s.area());
}