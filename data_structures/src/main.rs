#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

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
        _ => (),
    }
}

union IntOrFloat {
    i: i32,
    f: f32,
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => println!("meaning of life value"),
            IntOrFloat { f } => println!("value = {}", f),
        }
    }
}

fn unions() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 2345;

    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    iof.f = 42.0;
    process_value(iof);
}

fn options() {
    let x = 3.0;
    let y = 0.1;

    //Option -> Some(v) | None
    let result = if y != 0.0 { Some(x / y) } else { None };

    match result {
        Some(z) => println!("{}", z),
        None => println!("Cannot divide by Zero"),
    }

    if let Some(z) = result {
        println!("Got a result {}", z);
    }

    // while let Some(z) = result {
    //     println!("Got a result {}", z);
    // }
}

fn arrays() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    // let mut a = [1,2,3,4,5];
    println!("a has {} elements, first elem is {}", a.len(), a[0]);
    a[0] = 321;
    println!("a has {} elements, first elem is {}", a.len(), a[0]);

    println!("{:?}", a); // :? output debug info

    if a != [1, 2, 3, 4, 5] {
        println!("Arrays do not match");
    }
    if a != [321, 2, 3, 4, 5] {
        println!("Arrays do not match");
    }

    // error[E0277]: can't compare `[i32; 5]` with `[{integer}; 10]`
    // if a != [321, 2, 3, 4, 5, 5, 5, 5, 5, 5] {
    //     println!("Arrays do not match");
    // }

    let b = [1u8; 10];
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("array takes {} bytes", mem::size_of_val(&b));

    // matrix array of arrays
    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [3.0, 4.0, 0.0]];
    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            println!("{}", mtx[i][j]);
        }
    }
}

fn main() {
    structures();
    enums();
    unions();
    options();
    arrays();
}
