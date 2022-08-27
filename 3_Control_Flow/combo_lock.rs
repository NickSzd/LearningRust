#[allow(dead_code)]
#[allow(unused_variables)]

//use rand::Rng;
use std::io::stdin;

enum State{
    Locked,
    Failed,
    Unlocked
}

fn main()
{
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop
    {
        match state
        {
            //On state locked get user input
            State::Locked => 
            {
                let mut input = String::new();
                match stdin().read_line(&mut input)
                    { // read input from console
                        Ok(_) =>{
                            entry.push_str(&input.trim_end());
                        }

                        Err(_) => continue
                        /*
                        the function stdin() returns two values 
                        Ok() which is the value that has been given if there is no errors and 
                        Err() which gives an error code
                        */
                    }
                
                //if user input is equal to the code unlock
                if entry == code 
                {
                    state = State::Unlocked;
                    continue;
                } 
                //else if entry != code state is still locked
                if !code.starts_with(&entry)
                {
                    state = State::Failed;
                }       
            }

            //if state is Failed print failed and clear entry
            State::Failed =>
            {
                println!("FAILED");
                entry.clear();
                //state = State::Locked; // 

            }

            State::Unlocked => 
            {
                println!("UNLOCKED");
                return;
            }
        }
        
        
        
    }

}