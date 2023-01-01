fn array()
{
    // groups of 5 32 bit ints
    //Static Size
    let mut a:[i32; 5] = [0,1,2,3,4];

    println!("a has {} elements, index 0 = {}", a.len(), a[0]);

    for i in 0 .. a.len()
    {
        println!("a[{}] = {}", i, a[i]);
    }

    let b = [0;10]; // array size 10 with 0s
    for i in 0 .. b.len()
    {
        println!("b[{}] = {}", i, b[i]);
    }

    let mtx:[[f32;3]; 2] = 
    [
        [1.0, 1.0, 1.0],
        [0.0,2.0,0.0],
    ];
    println!("{:?}", mtx);

    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len()
        {
            println!("{}", mtx[i][j]);
        }
    }

}

fn main()
{
    array();
}