//Drop is a destructor 


struct Creature{name:String}

impl Creature {
    fn new(name: &str) ->Creature{
        println!("{} spawned", name);
        Creature{name: name.into()}
    }
}

impl Drop for Creature 
{
    fn drop(&mut self){
        println!("{} is dead", self.name);
    }
}

fn main(){

    let mut clever: Creature;{
        let goblin = Creature::new("Jeff");
        println!("Game stuff");
        clever = goblin;
    }
}
