
fn main(){
    // we can borrow an item like so
    let print_vector = |x:&Vec<i32>|{
        println!("{:?}",x);
    };

    let v = vec![1,2,3];
    print_vector(&v); // Passign by refferance is borrowing
    
    let mut a = 40;
    { 
        //Now since b is temporary once its scoep ends a has ownership again
        let b = &mut a;
        b* += 2;
    }
    // B is now borrowing a and it must be released before we can use a again

    

}