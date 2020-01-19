use rand::Rng;
use std::cmp::Ordering; // bring enum type into scope

// use nested path to clean up 'use' list
// 'self' - bring 'io' library into scope
// 'Write' - needed for 'stdout().flush()'
use std::io::{self, Write};

fn main() {
    println!("Welcome to the guessing game!");
    println!("-----------------------------");
    println!();

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // 'loop' = creates an infinite loop
    loop {
        // print message without newline
        print!("Please input your guess: ");
        io::stdout().flush().unwrap(); // needed as stdout is line-buffered

        // 'let mut' = create mutable variable
        // 'String' = type provided by the standard library
        // '::new' = associated function implemented on a type (static method)
        let mut guess = String::new();

        // 'stdin' = returns an instance of 'std::io::Stdin'
        // '.read_line(&mut guess)' = method on standard input handle
        //      '&' = indicated that this argument is a reference
        //      '&mut' = make reference mutable
        //  '.expect()' = handles 'io::Result' returned by 'read_line()'
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 'let guess' = shadow previous value (often used on type conversion)
        // ': u32' = annotate the variable type
        // '.trim()' = remove any whitespace
        // '.parse()' = parses a string into some kind of number
        // 'match' = is generally how to move from crashing to handling the error
        // '_' = catchall value, no matter what information in inside them
        // 'continue' = go to next iteration of the loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // '{}' = placeholder
        println!("You guessed: {}", guess);

        // 'cmp()' = compares two values, takes reference to whatever to campare with
        // 'match' = is made up of arms (like switch-case)
        //      - checks each arms from top to bottom
        //      - on match all following arms are ignored
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your number is too small!"),
            Ordering::Greater => println!("Your number is too big!"),
            Ordering::Equal => {
                println!();
                println!("---[ You win! ]---");
                println!();
                break; // exit the loop
            }
        }
    }
}
