use std::ops::Add;
use std::ops::AddAssign;

#[derive(Debug)]
struct Complex<T> {
    real: T,
    i: T,
}

impl<T> Complex<T> {
    fn new(real: T, i: T) -> Complex<T> {
        Complex::<T> { real, i }
    }
}

// impl Add for Complex<i32> {
//     type Output = Complex<i32>;
//     fn add(self, rhs: Self) -> Self::Output {
//         Complex{
//             real: self.real + rhs.real,
//             i: self.i + rhs.i,
//         }
//     }
// }

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            i: self.i + rhs.i,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.i += rhs.i;
    }
}

pub fn op_overloading() {
    let mut a = Complex::new(1, 2);
    let mut b = Complex::new(3, 4);
    // let c = a + b;
    // println!("{:?}", c);
    a += b;
    println!("{:?}", a);
}
