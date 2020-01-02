mod closures;
mod higher_order_functions;
mod methods;

fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) {
    *x += 1;
}

// lack of semi colon implies return statement
fn product(x: i32, y: i32) -> i32 {
    x * y
}

fn functions() {
    print_value(34 as i32);
    print_value(34);

    let mut z = 1;
    increase(&mut z);
    print_value(z);

    let a = 2;
    let b = 44;
    let p = product(a, b);
    println!("{}", p);
}

fn main() {
    functions();
    methods::methods();
    closures::closures();
    higher_order_functions::higher_order_functions();
}
