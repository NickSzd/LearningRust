fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("{:?}",a);
    let mut idx:usize = 0;
    println!("a[0] = {}", a[idx]);
    //idx = 111;
    //println!("a[0] = {}", a[idx]);//index out of bounds crash
    match a.get(6)
    {
        Some(x)=>println!("a[6] = {}", x),
        None => println!("Error out of Bounds"),
    }

    for i in 0..100
    {
        a.push(i);
    }
    println!("{:?}",a);
    //let mut last_elem;
    match a.pop()
    {
        Some(x)=>println!("Poped item = {}", x),
        None => println!("Empty Vector"),
    }


}

fn main()
{
    vectors();
}