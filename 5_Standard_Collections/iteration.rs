

pub fn main()
{
    let mut vec = vec![3,2,1];
    for x in &vec// Note write for x in &vec in order to have the referanc epointer go back to the beginning
    {
        println!("{}", *x);
    }

    //By referance
    for x in vec.iter()
    {
        println!("{}", x);
    }

    //Note use * to modify values in the vector
    for x in vec.iter_mut()
    {
        *x+=2;
        println!("{}", x);
    }
}