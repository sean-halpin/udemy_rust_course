use std::mem;

fn core_data_types() {
    let a: u8 = 123; // 8bits u8:unsigned/i8:signed
    println!("a = {}", a);

    let mut b: i8 = 0; // mutable
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 123456789; // i32 , signed 32 bit int
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after update", c);

    // i8 u8 i16 u16 i32 u32 i64 u64
    let z: isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {} uses {} bytes, {}-bit", z, size_of_z, size_of_z * 8);

    let d = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // e:f32 e:f64
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
    // Output
    // a = 123
    // b = 0
    // b = 42
    // c = 123456789, size = 4 bytes
    // c = -1 after update
    // z = 123 uses 8 bytes, 64-bit
    // d = x, size = 4 bytes
    // e = 2.5, size = 8 bytes
    // g = false, size = 1 bytes
}

fn operators() {
    // arithmetic
    let mut a = 2 + 3 * 4;
    println!("a = {}", a);
    a = a + 1;
    a -= 2;

    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed = {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed, = {}, {}^pi= {}", b, b_cubed, b, b_to_pi);

    // bitwise operators
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
    println!("1|2 = {}", c);
    let two_to_pow_ten = 1 << 10;
    println!("2^10 = {}", two_to_pow_ten);

    // logical 
    let pi_less_four = std::f64::consts::PI < 4.0; // true
    // < <= >= ==
    println!("pi < 4 ? {}", pi_less_four );

    let x = 5;
    let x_is_5 = x == 5; // true
    println!("5 == 5 ? {}", x_is_5);
}

fn scope_and_shadowing(){
    let a = 123;
    {
        let b = 456;
        println!("inside b = {}", b); // 456
        let a = 666; // this a shadows the a above
        println!("inside a = {}", a); // 666
    }
    println!("a = {}", a); // 123
    
    // Compile time error
    // error[E0425]: cannot find value `b` in this scope -- 'rustc --explain E0425'
    // println!("b = {}", b);
}

fn main() {
    core_data_types();
    operators();
    scope_and_shadowing();
}
