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

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        return (((self.x - other.x).powi(2)) + ((self.y - other.y).powi(2))).sqrt();
    }
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        return Circle {
            center: Point { x: x, y: y },
            radius: radius,
        };
    }

    pub fn diameter(&self) -> f64 {
        return self.radius * 2.0;
    }

    pub fn area(&self) -> f64 {
        return consts::PI * self.radius.powi(2);
    }

    pub fn intersect(&self, other: &Circle) -> bool {
        return (self.radius + other.radius) < self.center.distance(&other.center);
    }
}
