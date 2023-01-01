
//This does not take up 64 bits only 32
//Note type is unknwon without context
union IntOrFloat
{
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat){
    unsafe{
        match iof{
            IntOrFloat {i:42} => {
                println!("Meaning of Life");
            }
            IntOrFloat {f} =>{
                println!("Value = {}", f);
            }
        }
    }
}

fn main()
{
    let mut iof = IntOrFloat{i:123};
    iof.i = 234;

    //Since type is unkown an unsafe block must be used
    let value = unsafe {iof.i};
    println!("iof.i = {}", value);
    process_value(IntOrFloat{i:42});
    process_value(IntOrFloat{f:42.0});
    process_value(IntOrFloat{i:8});


}