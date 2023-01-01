use std::thread;
use std::time;
use std::collections::HashSet;

fn main()
{
    let mut greeks = HashSet::new();

    greeks.insert("gamma");
    greeks.insert("delta");
    greeks.insert("alpha");
    println!("{:?}", greeks);

    let added_vega = greeks.insert("vega");
    
    if added_vega
    {
        println!("Added Vega");
    }

    //Subset
    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();


    println!("is {:?} a subset of {:?}? {}",
    _1_5, _6_10,
    _1_5.is_subset(&_1_10));

    //is_subset()
    //is_disjoint()
    //union()

}