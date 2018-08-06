extern crate rand;

use std::io;
use rand::Rng;

fn main()
{
    let difficulty = menu();

    if difficulty == 1
        {
            game(100);
        }
    else if difficulty == 2
        {
            game(1_000)
        }
    else if difficulty == 3
        {
            game(10_000);
        }

    println!("See you next time!");
}

fn menu() -> u8
{
    println!("######################################");
    println!("#################MENU#################");
    println!("######################################");
    println!("\n");
    println!("1.easy");
    println!("2.intermediate");
    println!("3.expert");


    loop
        {
            println!("\nyour choice: ");

            let mut choice = String::new();
            io::stdin().read_line(&mut choice)//must explicit mutability
                .expect("Failed to read line!");

            let choice: u8 = match choice.trim().parse()
                {
                    Ok(num) => num,
                    Err(_) => { println!("Invalid input!"); 0}//needs to return the expected type
                };

            if choice > 0 && choice < 4
                {
                    return choice;//return is a statement
                }
        }
}

fn game(max_num : u32)
{
    let nbr_to_guess : u32 = rand::thread_rng().gen_range(1, max_num);
    let mut nbr_of_guess : u32 = 0;

    loop
        {
            println!("Your {} guess: ", nbr_of_guess+1);

            let mut guess = String::new();
            io::stdin().read_line(&mut guess)
                .expect("Failed to read line!");

            let guess: u32 = match guess.trim().parse()
                {
                    Ok(num) => num,
                    Err(_) => continue
                };

            nbr_of_guess = nbr_of_guess + 1;

            if guess == nbr_to_guess
                {
                    println!("You won with {} guesses!", nbr_of_guess);
                    break;
                }
            else if guess > nbr_to_guess
                {
                    println!("Too big!");
                }
            else if guess < nbr_to_guess
                {
                    println!("Too small!");
                }
        }
}