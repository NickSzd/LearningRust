fn sum_and_product(x:i32, y:i32) -> (i32, i32)
{
    return (x+y, x*y);
}

fn tuples()
{
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);
    println!("{:?}", sp);
    println!("({}, {})", sp.0, sp.1);

    //destructuring
    let (a,b) = sp;
    println!("a: {}, b: {}", a,b);

    let sp1 = sum_and_product(9,8);
    let combined = (sp, sp1);
    println!("{:?}", combined);
    println!("sp({}, {}), sp1({}, {})", (combined.0).0, (combined.0).1, (combined.1).0, (combined.1).1);

}

fn main()
{
 tuples();
}