// A trait is a common interface that a group of types can implement.
trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

/*
 To implement a trait for a type, we use the keywords impl Trait for Type,
 where Trait is the name of the trait being implemented and Type is the name of the implementor struct or the enum.
*/

impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Use the derive trait
/* 
The Debug and PartialEq traits can be automatically implemented for us by the Rust compiler 
by using the #[derive(Trait)] attribute, if each of its fields implements the trait:
*/
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 10.0, height: 20.0 };

    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());

    // Derive Trait
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: -3 };

    if p1 == p2 { // can't compare two Point values!
        println!("equal!");
    } else {
        println!("not equal!");
    }

    println!("{}", p1); // can't print using the '{}' format specifier!
    println!("{:?}", p1); //  can't print using the '{:?}' format specifier!
}
