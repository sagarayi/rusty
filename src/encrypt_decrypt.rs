pub fn encrypt(plain_text: String)  {

	let mut final_string: String = " ".to_string();
	let char_arr : Vec<_> = plain_text.chars().collect();

	let mut count = 0;
	let mut current_char: String = char_arr[0].to_string();

	for i in 0..char_arr.len() {
		
		count = count + 1;
		if  i+1 < char_arr.len() && current_char != char_arr[i+1].to_string() {

			final_string = final_string + &count.to_string() + &current_char;

			count = 0;
			current_char = char_arr[i+1].to_string()	;
		}
		
	}
	final_string = final_string + &count.to_string() + &current_char;


	println!("The encrypted string is {}", final_string); 
}