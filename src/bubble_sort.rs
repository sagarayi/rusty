
pub fn sort_array(mut arr: Vec<i32>) {

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

	println!("The sorted array is {:?}", arr);
}