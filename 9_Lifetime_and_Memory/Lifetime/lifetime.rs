struct Person<'p>{
    name: &'p str
}

impl<'p> Person<'p>{
    fn get_ref_name(&self){
        //
    }
}

struct Company <'z> //<'whatever> // is the lifetime
{
    name: String,
    ceo: &'z Person<'z>
}

fn main()
{
    //&'static str
    //'static is the lifetime of the variable


    //let boss = Person{name: String::from("BIll Gates")};
    //let microsoft = Company{name: String::from("Microsoft"), ceo: &boss};

    let person = Person{name : "Dudde"};
}