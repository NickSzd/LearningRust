fn main()
{
    let x = 3.0;
    let y = 0.0;
    // Option<T> 
    //  Two results
    //    Some(v) Operation worked
    //    None Operation Failed
    let result = if y != 0.0{Some(x/y)}else{None};

    match result{
        Some(z) => println!("{}/{}={}", x,y,z),
        None => println!("ERROR DIV BY ZERO ERROR"),
    }

    //let checks if the assignment is possible
    if let Some(z) = result{
        println!("result = {}", z)
    }
}