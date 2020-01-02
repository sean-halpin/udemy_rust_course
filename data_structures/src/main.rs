#![allow(dead_code)]
#![allow(unused_variables)]

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("Point p is at ({},{})", p.x, p.y);
    let p2 = Point { x: 13.0, y: 14.0 };
    println!("Point p2 is at ({},{})", p2.x, p2.y);
    let this_line = Line { start: p, end: p2 };
}

fn main() {
    structures();
}
