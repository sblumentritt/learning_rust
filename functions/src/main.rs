fn main() {
    another_function(7, 8);

    let expression = return_value_with_expression();
    let keyword = return_value_with_keyword();

    println!("The value of expression is: {}", expression);
    println!("The value of keyword is: {}", keyword);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn return_value_with_expression() -> i32 {
    7 * 7
}

fn return_value_with_keyword() -> i32 {
    return 5 * 5; // NOTE: clippy warns here about needless return
}
