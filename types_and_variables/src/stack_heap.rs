use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    // stack allocated
    let p1 = origin();
    // p2 is stack allocated,
    // pointing to a struct which is heap allocated
    let p2 = Box::new(origin());

    // 16 bytes
    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    // 8 bytes (just a pointer)
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    // unboxing
    let p3 = *p2;
    println!("{},{}", p3.x, p3.y)
}
