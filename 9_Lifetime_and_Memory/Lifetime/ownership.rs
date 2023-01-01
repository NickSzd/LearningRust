
fn main(){
    println!("Hello World");

    let v = vec![1,2,3];
    //v is on the stack, its data is on the heap
    //let v2 = v;//v2 is basically a pointer to v

    //println!("{:?}", v);//Thsi will not compile since v2 is now
    //the owner of the data from v
    /*
        the Most recently declared variable is the owner of the data
        so let v = vec![...]
        let v1 = v;
        let v2 = v1;
        let v_n = v_n-1
        etc
        let v_i = v_n
        //v_i is now ht owner of the data fron v and ve can 
        no longer access that data
    */

    // we can borrow an item like so
    let print_vector = |x:Vec<i32>|->Vec<i32>{
        println!("{:?}",x);
        return x; // by returning x we are returning the ownership
    };
    let vv =print_vector(v);
    
}