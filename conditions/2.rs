fn main() {
	let formal = true;
	let greeting = if formal { // if used here as an expression
	    "Good day to you."     // return a String
	} else {
	    "Hey!"                 // return a String
	};
	println!("{}", greeting)   // prints "Good day to you."
}
