use std::collections::HashMap;
fn main()
{
    let mut shapes = HashMap::new();
    shapes.insert(String::from("Triangle"),3);
    shapes.insert(String::from("Square"),4);

    println!("Square {}", shapes["Square".into()]);

    //If an entry is not present add it
    shapes.entry("Circle".into()).or_insert(0);
    {
        let actual = shapes
        .entry("Cirlce"
        .into())
        .or_insert(360);
        *actual = 360;
    }
    for(key,value) in &shapes{
        println!("{} : {} ", key, value);
    }


}