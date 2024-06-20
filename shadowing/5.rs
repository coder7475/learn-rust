// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 5;
    {
        // shadowing x 
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);
    // redeclare
    let x = 42;
    println!("{}", x); // Prints "42".
}