fn main() {
	let num = 500; // num variable can be set at some point in the program
	let out_of_range: bool;
	if num < 0 {
	    out_of_range = true;
	} else if num == 0 {
	    out_of_range = true;
	} else if num > 512 {
	    out_of_range = true;
	} else {
	    out_of_range = false;
	}

	println!("{out_of_range}");
}
