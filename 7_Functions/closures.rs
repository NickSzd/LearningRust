fn say_hello(){println!("HELLO");}

fn closure()
{
    //ALL OF THESE ARE EQUIVELENT
    let sh = say_hello;
    sh();
    say_hello();
    //--------------------------
    //this creates a functihigher_order_functs.rson in the scope of the closur function
    let plus_one = |x:i32| -> i32 {x+1};
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let plus_two = |x|{
        let mut z = x;
        z+=2;
        return z;
    };
    println!("{} + 2= {}", 3, plus_two(3));

    let plus_three = |x:&mut i32| *x +=3;
    let mut f = 3;
    plus_three(&mut f);
    println!("f= {}", f);

}

fn main()
{
    closure();
}