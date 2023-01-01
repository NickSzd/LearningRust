use std::mem;

trait Printable{
    fn foramt(&self)->String;
}

impl Printable for i32{
    fn format(&self)->String{
        format!("i32: {}", *self)
    }
}

impl Printable for String{
    fn format(&self)->String{
        format("string: {}", *self);
    }
}

fn main()
{

}
