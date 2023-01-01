fn main()
{
    let name = "Nick";
    let greeting = format!("hi , i'm {}", name);
    println!("{}", greeting);

    let hello = "Hello";
    let rust = "Rust!";

    let hello_rust = format!("{}, {}", hello, rust);
    println!("{}", hello_rust);

    let run = "run";
    let forrest = "Forrest";
    let rfr = format!("{0}, {1}, {0}", run, forrest);
    println!("{}", rfr);
}