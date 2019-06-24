use std::io::{self, Write};

pub fn main() {

	print!("Enter the length of the array : ");

	io::stdout().flush().unwrap();

	let mut arr_inp = vec![];

	let mut buffer = String::new();

	io::stdin().read_line(&mut buffer).expect("Failed to read");

	buffer = buffer.trim().to_string();

	let length: i32 =  buffer.parse().expect("Only numbers accepted");

	buffer.clear();

	for x in 0..length {

		print!("Enter a[{0}] element : ",x);

		io::stdout().flush().unwrap();

		io::stdin().read_line(&mut buffer).expect("Failed to read");

		buffer = buffer.trim().to_string();

		let input_num: i32 = buffer.parse().expect("Only numbers accepted");

		// arr_inp.push(buffer.parse().expect("Only numbers accepted"));
		arr_inp.push(input_num);
		buffer.clear();
	}

	print!("The entered numbers are  {:?}", arr_inp );

	sort_array(arr_inp);
}

fn sort_array(mut arr: Vec<i32>) {

	let mut temp;

	for i in 0..arr.len() {
		for j in i..arr.len() {
			if arr[j]>arr[i] {
				temp = arr[i];
				arr[i] = arr[j];
				arr[j] = temp;
			}
		}
	}

	print!("The sorted array is {:?}", arr);
}