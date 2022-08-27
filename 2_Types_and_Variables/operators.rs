#[allow(dead_code)]
#[allow(unused_variables)]
use std::mem;

fn operators()
{
    let mut a = 2+3*4;
    a = a + 1;
    
    a += 1;

    println!("remainder of {} / {} = {}", a, 3,(a%3));

    let a_cubed = i32::pow(a,3);
    println!("a^3 = {}", a);

    let b = 2.5;
    let b_cubed = f64::powi(b,3); // integral power
    println!("b^3 = {}", b_cubed);
    let b_to_pi = f64::powf(b,std::f64::consts::PI); // integral power
    println!("b_to_pi = {}", b_to_pi);

    //BITWISE
    /*
    | OR
    & AND
    ^ XOR
    ! NOR
     << shift left
     >> shift right
    */
    let c = 1 | 2; // 01 OR 10 = 11 = 3
    println!(" 1 | 2  = {}", c);

    //LOGICAL
    let pi_less_4 = std::f64::const::PI < 4.0; // true
    // > <= >= ==

    let x = 5;
    let x_is_5 = x == 5;


}

fn scope_and_shadowing()
{
    let a = 123;
    let a = 1234;
    {
        let b = 456;
        println!("inside, b = {}", b);
        let a = 777;
        println!("inside a = {}", a);
    }

    println!("Outside a = {}", a);

}

fn main()
{
    operators();
}