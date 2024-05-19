use std::f64::consts;
#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point { x, y},
            radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        const PI: f64 = consts::PI;
        PI * self.radius * self.radius
    }

    pub fn intersect(&self, other: &Circle) -> bool {
        let distance = self.center.distance(&other.center);
        distance <= self.radius + other.radius
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        (x_diff * x_diff + y_diff * y_diff).sqrt()
    }
}