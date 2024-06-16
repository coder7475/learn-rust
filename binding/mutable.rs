fn main() {
    let mut x: i32 = 3;
    x += 2;

    assert_eq!(x, 5);
    println!("mutation success!")
}