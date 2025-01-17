#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Method that borrows immutably
    fn display(&self) { // here receiver is the immutable reference to Point
        println!("Point({}, {})", self.x, self.y);
    }

    // Method that borrows mutably
    fn move_point(&mut self, dx: i32, dy: i32) { // here receiver is the mutable reference to Point
        self.x += dx;
        self.y += dy;
    }

    // Method that consumes the instance
    fn into_tuple(self) -> (i32, i32) { // here receiver is the Point itself
        (self.x, self.y)
    }
}

fn main() {
    let mut p = Point { x: 3, y: 4 };

    // Immutable borrow
    p.display();

    // Mutable borrow
    p.move_point(1, 2);
    p.display();

    // Ownership transfer
    let tuple = p.into_tuple();
    println!("{:?}", tuple);

    // dbg!(&p); 
    // dbg!(p);
}
/*
    How the automatic referencing and dereferencing works :
    Given the receiver(function params) and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self).

*/