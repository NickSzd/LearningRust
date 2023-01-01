use std::thread;
use std::rc::Rc;// Allows use of multiple references
use std::sync::Arc;
struct Person{
    name: Arc<String>
}

impl Person{
    fn new(name: Arc<String>)->Person{
        Person{name: name}
    }
    fn greet(&self){
        println!("Hi MY name is, What? My name is, who? My Name is {}", self.name);
    }
}

fn rc_demo(){
    let name = Arc::new("Bill".to_string());
    let person = Person::new(name.clone()); // when name is called name is moved from the local scope
    //.clone creates a clone

    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}",name);
    t.join().unwrap();
}

fn main()
{
    rc_demo();
}