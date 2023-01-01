use std::rc::Rc;// Allows use of multiple references
struct Person{
    name: Rc<String>
}

impl Person{
    fn new(name: Rc<String>)->Person{
        Person{name: name}
    }
    fn greet(&self){
        println!("Hi MY name is, What? My name is, who? My Name is {}", self.name);
    }
}

fn rc_demo(){
    let name = Rc::new("Bill".to_string());
    let person = Person::new(name.clone()); // when name is called name is moved from the local scope
    //.clone creates a clone

    person.greet();
    println!("Name = {}", name);
    println!("Name: {}, has {} pointers", name, Rc::strong_count(&name));

}

fn main()
{
    rc_demo();
}