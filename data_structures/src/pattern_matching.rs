// & = passing a ref to the object
// 'static = longest possible lifetime
// &str = string slice or string literal
fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "a couple",
        z @ 9..=11 => "just shy of a dozen",
        12 => "a dozen",
        _ if (x % 2 == 0) => "an even number of",
        _ => "many",
    }
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

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (3, 4);
    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (_, y) => println!("(?,y:{})", y),
    }

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
        // ,.. we don't care about the other values
        Colour::Cmyk { black: 255, .. } => println!("black"),
        Colour::RgbColor(r, g, b) => println!("{},{},{}", r, g, b),
        _ => (),
    }
}
