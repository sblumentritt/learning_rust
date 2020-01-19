use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

struct PancakesImpl;

impl HelloMacro for PancakesImpl {
    fn hello_macro() {
        println!("Hello from the impl!");
    }
}

#[derive(HelloMacro)]
struct PancakesDerive;

fn main() {
    PancakesImpl::hello_macro();
    PancakesDerive::hello_macro();
}
