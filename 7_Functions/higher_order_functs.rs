fn is_even(x: u32)->bool{
    return x%2 == 0;
}



fn main()
{
    //Fucntions that take functions
    //Fucntions that return functions

    //sum of all even squares < 500

    let limit = 500;
    let mut sum = 0;
    let above_limit = |y| y > limit;

    for i in 0..{
        let isq = i*i;
        if above_limit(isq) {break;}
        else if is_even(isq){
            sum += isq;
        }
    }
    println!("sum = {}", sum);
}