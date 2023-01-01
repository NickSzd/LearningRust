use std::ops::{Add};

#[derive(Debug)]
struct Complex<T>
{
    re:T,
    im:T
}

impl<T> Complex<T>{
    fn new(re:T, im:T)->Complex<T>{
        Complex::<T> {re, im}
    }
}
impl<T> Add for Complex<T>
    where T: Add<Output = T>{
    type Output = Complex<T>;
    fn add(self, rhs: Self)->Self::Output{
        return Complex{
        re:self.re + rhs.re,
        im:self.im + rhs.im
        }
    }
}

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>{
    fn add_assign(&mut self, rhs: Self){
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Neg for Complex<T>
    where T:Neg<Output = T>
{
    fn neg(self, rhs: Self)->Self::Output{
        return Complex{
            re:-self.re,
            im:-    self.im 
        }
    }
}
fn main()
{
    let mut a = Complex::new(1,2);
    let mut b = Complex::new(3,4);
    println!("{:?}", a);
    println!("{:?}", a + b);
}
