// Pattern Matching
fn how_many(x:i32)->&'static str
{
    match x
    {
        0 => "None",
        1 | 2 => "one or two",
        12 => "a dozen",
        (9..=11) => "lots",
        _ if (x%2 == 0) => "some",
        _ => "a few"
    }
}

fn pattern_matching()
{
    for x in 0 .. 13
    {
        println!("Oranges: {}, or {}", x, how_many(x));
    }
}

fn main()
{   
    pattern_matching();
}