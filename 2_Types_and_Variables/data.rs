#[allow(dead_code)]
#[allow(unused_variables)]
use std::mem;

fn main()
{

    //INTEGERS

    let a: u8 = 123; //u = unsigned, 8 bits, 0-255
    println!("a = {}", a); // {} will be replaced by the value of a

    //u = UNSIGNED 0 - (2^n)-1
    //i = SIGNED -2^(N-1) - 2 ^(N-1)
    let mut b: i8 = 0;//mutable variable ie the value can be changed
    println!("b = {}", b);
    b = 6;
    println!("b is now {}", b);
    
    let c = 123456789;
    println!("c = {}, takign up {} bytes", c, mem::size_of_val(&c)); //mem::size_of_val(&c) allows the user to dispaly the size of the variable in bytes requires a pointer
    // u8, u16, u32, u64, i8, i16, ...

    //usize isize
    let z: isize = 123; // uses size of machine
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}. taking up {} bytes, {} bit OS",
    z, size_of_z, size_of_z*8);

    //CHARACTERS
    let d: char = 'x';
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    //FLOATING POINT NUMBERS
    //f32, f64

    let e:f32 = 2.5; // note the compiler will assume f64 if not specified
    println!("e = {}, takign up {} bytes", e, mem::size_of_val(&e)); 

    //boolean
    let g:bool = false;
    println!("g = {}, takign up {} bytes", g, mem::size_of_val(&g)); 
    

}