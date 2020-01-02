// functions that take functions as args
// functions that return functions

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    move |y| y > limit
    // we use 'move' because without it,
    // error[E0597]: `limit` does not live long enough
    // https://doc.rust-lang.org/std/keyword.move.html
}

pub fn higher_order_functions() {
    let limit = 500;
    let mut sum = 0;

    let above_limit = greater_than(limit);

    for i in 0.. {
        let isq = i * i;
        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }

    println!("loop sum = {}", sum);

    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x: &u32| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("hof sum = {}", sum2);

    let sum3 = (0..)
        .map(|x| x * x)
        .take_while(|x| x < &limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("hof sum = {}", sum3);
}
