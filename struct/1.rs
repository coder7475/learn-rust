fn main() {
	
	struct Student { name: String, level: u8, remote: bool }

	let user = Student { 
		name: String::from("Robiul Hossain"), level: 4, remote: false
	 };	

	println!("{}, level: {}, remote: {}", user.name, user.level, user.remote);

}
