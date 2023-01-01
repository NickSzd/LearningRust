trait Animal{
    fn create(name: &'static str)->Self;
    fn name(&self)->&'static str;
    fn talk(&self){
        println!("{} cannot talk", self.name());
    }
}

struct Human{
    name: &'static str
}

struct Cat{
    name:&'static str
}
impl Animal for Cat{
    fn create(name:&'static str)->Cat{
        return Cat{name:name};
    }
    fn name(&self)->&'static str{
        return self.name;
    }
    fn talk(&self)
    {
        println!("Meow");
    }
}

impl Animal for Human{
    fn create(name:&'static str)->Human{
        return Human{name:name};
    }

    fn name(&self)->&'static str
    {
        return self.name;
    }
    fn talk(&self)
    {
        println!("Hello");
    }
}

fn traits(){
    let h = Human{name:"Jim"};
    h.talk();
}

fn main()
{
    traits();
}