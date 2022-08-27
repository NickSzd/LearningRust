fn if_statement()
{
    let temp = 25;

    if temp > 30
    {
        println!("it is hot");
    }
    else if temp < 10
    {
        println!("It is cold");
    }
    else
    {
        println!("It is nice out");
    }


    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}",day);
    /*
        since in Rust if is a statement we can use it to inintilize variables. In 
        this case day is set to sunny if temp is greater than 20
    */

    println!("it is {}", 
        if temp > 20 {"hot"} 
        else if temp < 10{"cold"} 
        else {"ok"});

    println!("it is {}",
        if temp > 20{
            if temp > 30 {"very hot"} else{"hot"}
            }
        else if temp < 10{"cold"} 
        else {"ok"});
}