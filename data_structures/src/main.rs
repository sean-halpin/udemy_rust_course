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

enum Colour {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    Cmyk {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // struct
}

fn enums() {
    // let c:Colour = Colour::Red;
    let c: Colour = Colour::Cmyk {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };
    match c {
        Colour::Red => println!("r"),
        Colour::Green => println!("g"),
        Colour::Blue => println!("b"),
        Colour::RgbColor(0, 0, 0) => println!("black"),
        Colour::Cmyk {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("black"),
        Colour::RgbColor(r, g, b) => println!("{},{},{}", r, g, b),
        _ => ()
    }
}

fn main() {
    structures();
    enums();
}
