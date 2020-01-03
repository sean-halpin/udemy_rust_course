pub fn borrowing() {
    let print_vector = |x: &Vec<i32>| {
        println!("{:?}", x);
    };

    let v = vec![3, 2, 1];
    print_vector(&v);
    // this works since the ref was borrowed
    println!("{:?}", v);

    let mut a = 40;
    {
        // note: there may be only 1 mutable ref to a variable
        let b = &mut a;
        *b += 2;
    }
    println!("{:?}", a);

    let mut z = vec![3, 2, 1];
    for i in &z {
        println!("i = {}", i);
        // Cannot change object we are iterating - undefined behaviour
        // z.push(5);
    }
}
