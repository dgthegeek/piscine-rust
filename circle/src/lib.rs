use std::f64::consts;

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point { x, y },
            radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        consts::PI * self.radius.powf(2.0)
    }

    pub fn intersect(&self, other: &Circle) -> bool {
        let distance_between_centers = self.center.distance(&other.center);
        distance_between_centers < self.radius + other.radius
    }
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
    }
}

