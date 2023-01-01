use std::thread;
use std::rc::Rc;// Allows use of multiple references
use std::sync::{Arc, Mutex};
struct Person{
    name: Arc<String>,
    state: Arc<Mutex<String>>
}

impl Person{
    fn new(name: Arc<String>, state: Arc<Mutex<String>>)->Person{
        Person{name: name, state: state}
    }
    fn greet(&self){
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("Pog");

        println!("Hi MY name is, What? My name is, who? My Name is {}, and i am {}", self.name, state.as_str());
    }
}

fn mutex_demo()
{
    let name = Arc::new("Bill".to_string());
    let state = Arc::new(Mutex::new("Bored".to_string()));
    let person = Person::new(name.clone(),state.clone()); // when name is called name is moved from the local scope
    //.clone creates a clone

    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}, State = {}",name, state.lock().unwrap().as_str());
    t.join().unwrap();
}

fn main()
{
    mutex_demo();
}