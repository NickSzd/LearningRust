
fn main()
{
    let mut x = 1;
    while x < 1000
    {
        x*= 2;
        if x > 1000 {continue;} // this will skip all statements after 
        println!("x: {}", x);
    }

    let mut y = 1;
    loop //behaives like a while(true) loop
    {
        y *= 2;
        println!("y = {}", y);
        if y == 1<<10 {break;}
    }

    for x in 1..11 // for x in range
    {
        if x == 3 {continue;}
        println!("x is {}", x);
    }

    for (pos, y) in (30..41).enumerate()
    {
        println!("{}: {}", pos, y);
    }
    /*
    The Enumetate fucntion generates the 0 - 10
    0: 30
    1: 31
    2: 32
    3: 33
    4: 34
    5: 35
    6: 36
    7: 37
    8: 38
    9: 39
    10: 40
    */

    let country_code = 44;

    let country = match country_code {
        44 => "UK",
        46 =>"sweden",
        1..=1000=> "unknown",
        _ => "invalid" // the _ symbol merans all other cases 
    };

    println!("Country with code: {} is {}", country_code, country);

}