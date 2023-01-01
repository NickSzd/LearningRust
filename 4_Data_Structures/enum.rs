
//use std::mem;

enum Color{
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), //tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8}, //struct
}

fn enums()
{
    //let c:Color = Color::Red;
    //let c:Color = Color::RgbColor(1,2,3);
    let c:Color = Color::CmykColor{cyan:5, magenta:6, yellow: 7, black: 8};


    //Switch 
    match c{
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RgbColor(0,0,0) =>println!("Black"),
        Color::RgbColor(r,g,b) =>println!("rgb({},{},{})", r,g,b),
        Color::CmykColor{cyan:_,magenta:_,yellow:_,black:_} =>println!("cmyk"),


    }
}

fn main()
{
    enums();
}