fn say_hello() {
    println!("hello");
}

pub fn closures() {
    let sh = say_hello;
    sh();

    let plus_one = |x: i32| -> i32 { x + 1 };
    let a = plus_one(1);
    println!("{}", a);

    let mut two = 2;
    {
        let plus_two = |x| {
            let mut z = x;
            z += 2;
            z
        };
        println!("{}", plus_two(two));
    } // we close this scope here to ensure the closure returns ownership to variable 'two'

    let _borrow_two = &mut two;

    // T: by value
    // &T
    // &mut &T
    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);
}
