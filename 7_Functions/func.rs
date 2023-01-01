fn print_val(val: i32){
    println!("Val: {}", val);
}

fn increase(x: &mut i32){
    *x = *x + 1;
    print_val(*x);
}
//fn function_name(param1, param2, ...) -> return_type {...}
fn product(x: i32, y: i32)->i32{
    //Note you can tell rust to return a value by not including a semi colon
    //ie x*y is equivilent to return x*y
    return x*y;
}

fn function(){
    print_val(5);
    let mut z = 1;
    increase(&mut z);
    println!("{}",product(5,5));

}

fn main()
{
    function();
}