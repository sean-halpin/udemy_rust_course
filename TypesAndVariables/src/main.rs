use std::mem;

fn main() {

    let a:u8 = 123; // 8bits u8:unsigned/i8:signed
    println!("a = {}", a);

    let mut b:i8 = 0; // mutable
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 123456789; // i32 , signed 32 bit int
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after update", c);

    // i8 u8 i16 u16 i32 u32 i64 u64
    let z:isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {} uses {} bytes, {}-bit", z, size_of_z, size_of_z * 8);

    let d = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // e:f32 e:f64
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));

}
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