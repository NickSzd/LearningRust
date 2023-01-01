//use rand::Rng;
use std::io::stdin;

fn main()
{
    let number  = 42;//rand::thread_rng().gen_range(1,101);

    loop
    {
        let mut guess = String::new();
        println!("Enter your guess: ");
        match stdin().read_line(&mut guess){
            Ok(_) => {
                let parsed = guess
                .trim_end()
                .parse::<i64>();//trim_end removes the \0 
                match parsed{
                    Ok(guess)=>{
                        if guess < 1 || guess > 100
                        {
                            println!("Value out of range");
                        }
                        else if guess < number{
                            println!("Low");
                        }
                        else if guess > number{
                            println!("High");
                        }
                        else{
                            println!("Correct");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Could not read input. {}. Try Again",e);
                    }
                }

            },
            Err(_) => continue,
        }
    }
}