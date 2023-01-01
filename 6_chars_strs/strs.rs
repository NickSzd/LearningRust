fn strings()
{
    // sequence of utf-8 chars
    let s:&'static str = "hello world!"; //&str = string slice
    // Str slices are static and cannot be changed
    for c in s.chars()
    {
        println!("{}", c);
    }
    //Note 
    //println!("{}", s[0]); will not work normally
    
    // To access indexes of a string you must use the String identifier
    let mut letters = String::new();
    //NOTE this is declared on the Heap

    let mut a = 'a' as u8;
    while(a<=('z' as u8))
    {
        letters.push(a as char);
        letters.push_str(","); // thsi will append str chars
        a+=1;
    }
    println!("{}", letters);

    //&str <> String
    let u:&str = &letters;

    //concatenation
    let z = letters + &letters;

    let mut abc = "hello World!".to_string();
    

}

fn main()
{
    strings();
}