extern crate rand;

use std::io::{self, Write};

mod insertion_sort;
mod bubble_sort;
mod guess_number;
mod boiler_plate;


fn main()
{
	match display_menu() {
		1 => bubble_sort::sort_array(boiler_plate::number_array_input()),
		2 => insertion_sort::sort_array(),//boiler_plate::number_array_input()),
		3 => guess_number::main(),
		_ => println!("Invalid Option. Ok Bye ;)"),
	}
}

fn display_menu() -> i32 {

	println!("***************************************");
	println!("              Algo Menu                ");
	println!("***************************************");
	println!("1. Bubble Sort");
	println!("2. Insertion Sort");
	println!("Bonus : Guessing game - Press (last_option + 1)");

	print!("Enter your choice : ");
	io::stdout().flush().unwrap();

	let mut buffer = String::new();

	io::stdin().read_line(&mut buffer).expect("Invalid entry");

	let user_choice : i32 = match buffer.trim().to_string().parse()
						  {
						  		Ok(num) => num,
						  		Err(_) => -1,
						  };
	return user_choice;
}
