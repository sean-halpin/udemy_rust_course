#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::mem;
use std::vec::Vec;

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

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);
    a.push(44);
    println!("a = {:?}", a);
    let idx: usize = 0;
    println!("a[0] = {:?}", a[idx]);
    // Option , protect against out of bounds indexing
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("No element"),
    }
    for x in &a {
        println!("{}", x)
    }
    let last_elem = a.pop(); // Option
    println!("last is {:?}", last_elem);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

fn use_slice(slice: &mut [i32]) {
    println!("first elem {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices() {
    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]);
    println!("{:?}", data);
}

fn strings() {
    let s: &'static str = "hello there";
    // s = ""; // error[E0384]: cannot assign twice to immutable variable `s`
    // let h=s[0];
    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("{}", first_char);
    }

    // heap allocated
    // String
    let mut letter = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letter.push(a as char);
        letter.push_str(",");
        a += 1;
    }
    println!("{:?}", letter);

    // &str <> String
    let u: &str = &letter;

    // concat
    // String + str
    let z = letter + "abc";
    println!("{:?}", z);
    // let zz = letter + &letter;
    // println!("{:?}", zz);

    let mut abc = String::from("hello world");
    println!("{}", abc);
    let mut def = "helloo".to_string();
    println!("{}", def);
    def.remove(0);
    println!("{}", def);
    def.push_str("!!!");
    println!("{}", def);
    def = def.replace("!", "xx");
    println!("{}", def);
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y) // implied return
}

fn tuples() {
    let x = 2;
    let y = 3;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);
    // destructuring
    let (a, b) = sp;
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("{}", (combined.1).1);

    let ((c, d), (e, f)) = combined;
    println!("{}", d);

    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    // single element tuple
    let meaning = (42,);
    println!("{:?}", meaning);
}

fn hashmaps() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square"]);
    shapes.insert(String::from("square"), 5);

    for (key, value) in &shapes {
        println!("{}, {}", key, value);
    }

    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}", shapes);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes);
}

fn main() {
    structures();
    enums();
    unions();
    options();
    arrays();
    vectors();
    slices();
    strings();
    tuples();
    hashmaps();
}
