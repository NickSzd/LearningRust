//& means we are  taking a chunk of data
fn use_slice(slice: &mut[i32])
{
    println!("first Elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 1;
}

fn slices()
{
    let mut data = [0,1,2,3,4];
    //use_slice(&mut data[1..4]);
    use_slice(&mut data);
    println!("{:?}", data);
}

fn main()
{
    slices();
}