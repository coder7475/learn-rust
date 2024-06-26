// <-- Scope A Starts
let x : f32 = 12.0;
    
println!("x is originally {}",x);
    
{ // <-- Scope B Starts
 	let x =  x as i32;

	let a_string = "hello";
        
	println!("The value x inside the scope {}", x / 2);
	println!("a_string is {}",a_string);

} // <-- Scope B Ends, drops the value of x that is inside the Scoep B
    
println!("Outside the scope, the value of scope is {}", x)

println!("a_string outside the scope is {}", a_string); // Error: cannot find value `a_string` in this scope

println!("Trying to divide the original floating point x by 2 (int) {}", x / 2)
// "cannot divide `f32` by `{integer}` ..."


// <-- Scope A Ends
