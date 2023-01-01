struct Point<T,T1>
{
    x: T,
    y: T1
}

fn genarics()
{
    let a:Point<i32, i32> = Point{x:0,y:0}; // Specifing genaric type
    let b = Point{x:1.2, y:3.4};
}

fn main()
{
    genarics();
}