fn main() {
    // let x = 5; // -> immutable, the following code does not work
    let mut x = 5; // -> mutable, the following code will work
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
