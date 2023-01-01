trait Animal{
    fn create(name: &'static str)-> Self;
    fn name(&self)->&'static str;
    fn talk(&self){
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human{
    fn create(name:&'static str)->Human{
        return Human{name: name};
    }
    fn name(&self) -> &'static str{
        return self.name;
    }

    fn talk(&self) {
        println!("Hello");
    }
}

impl Animal for Cat{
    fn create(name:&'static str)->Cat{
        return Cat{name: name};
    }

    fn name(&self) -> &'static str{
        return self.name;
    }

    fn talk(&self) {
        println!("Meow");
    }
}

trait Summable<T>{
    fn sum(&self)->T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self)->i32{
        let mut result:i32 = 0;
        for x in self{result +=*x;}
        return result;
    }
}
fn traits(){

    let h = Human::create("Jim");
    //let h = Human{name:"Bill"};
    h.talk();

    let c = Cat{name:"Kitty"};
    c.talk();

    let a = vec![1,2,3];
    println!("sum = {}", a.sum());
}
fn main()
{
    traits();
}
