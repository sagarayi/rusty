
use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn main() {

println!("The 'Guess the number' game ");

let random_number = rand::thread_rng().gen_range(1,101);

println!("The random number generated is : {}",random_number);

loop 
	{
		println!("Please enter your guess");

		let mut user_guess = String::new();

		io::stdin().read_line(&mut user_guess)
		    .expect("Failed to read the input");

		let user_guess: u32 = match user_guess.trim().parse()
		    {
		    	Ok(num) => num,
		    	Err(_) =>
		    	{
		    		println!("Error : Only numbers accepted");
		    		continue;
		    	}
		    };

		println!("Your guess is : {}",user_guess);


		match user_guess.cmp(&random_number) 
		{
		    Ordering::Less => println!("Too small !"),
		    Ordering::Greater => println!("Too Big !"),
		    Ordering::Equal => 
		    {
		    	println!("Perfect !!!");
		    	break;
		    }
		}
	    	
	}
}