// https://riptutorial.com/rust/example/15356/ownership-and-the-copy-trait

pub fn ownership() {
    // v on stack
    // data is on heap
    // v owns pointer & data it points to
    let v = vec![1, 2, 3];
    // ownership of pointer & data transferred to v2
    let v2 = v;
    // will not compile - moved value
    // v no longer usable
    // println!("{:?}", v);

    let u = 1; // i32
    let u2 = u;
    // this will work because u is a primitive type on the stack and is copied by value
    // most primitive types implement the Copy trait
    println!("{:?}", u);

    let bu = Box::new(1); // Box<i32>
    let bu2 = bu;
    // this will not work because bu is a boxed type on the heap and ownership is transferred to bu2
    // We could implement the Copy trait for an i32 here in order to allow this behaviour
    // println!("{:?}", bu);

    // This would work. Kind of inconvenient -> Reason for concept of Borrowing
    let print_vector = |x: Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };
    let vv = print_vector(v2);
}
