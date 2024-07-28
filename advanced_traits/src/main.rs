// ==============================
//        Associated Types
// ==============================

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator<u16> for Counter {
    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}

impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

// =============================================
//        Default Generic Type Parameters
// =============================================

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Add trait looks like below:
// trait Add<Rhs = Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// ================================================
//        Calling Methods With The Same Name
// ================================================

trait Pilot {
    fn fly(&self);
}

trait AssociatedPilot {
    fn fly();
}

trait Wizard {
    fn fly(&self);
}

trait AssociatedWizard {
    fn fly();
}

struct Human;
struct AssociatedHuman;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl AssociatedHuman {
    fn fly() {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking...");
    }
}

impl AssociatedPilot for AssociatedHuman {
    fn fly() {
        println!("This is your captain speaking...");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl AssociatedWizard for AssociatedHuman {
    fn fly() {
        println!("Up!");
    }
}

// =========================
//        Supertraits
// =========================

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // To use 'to_string()' fn we need to point that OutlinePrint depends on Display trait -> 'OutlinePrint: fmt::Display'
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("* {} *", output);
        println!("{}", "*".repeat(len + 4));
    }
}

struct Coordinates {
    x: i32,
    y: i32,
}

impl OutlinePrint for Coordinates {}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    // =============================================
    //        Default Generic Type Parameters
    // =============================================

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(Millimeters(1000) + Meters(1), Millimeters(2000));

    // ================================================
    //        Calling Methods With The Same Name
    // ================================================

    let person = Human;

    // Below comes from Human struct
    person.fly();

    // Below comes from Pilot trait for Human
    Pilot::fly(&person);

    // Below comes from Wizard trait for Human
    Wizard::fly(&person);

    //////////////////////////
    // ASSOCIATED FUNCTIONS //
    //////////////////////////

    // Below comes from Human struct
    AssociatedHuman::fly();

    // Below comes from Pilot trait for Human
    <Human as Pilot>::fly(&person);

    // Below comes from Wizard trait for Human
    <Human as Wizard>::fly(&person);

    // =========================
    //        Supertraits
    // =========================

    OutlinePrint::outline_print(&Coordinates { x: 666, y: 666 });
}
