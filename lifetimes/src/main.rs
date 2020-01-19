// lifetime annotation with `'a`
// - does not change the lifetime
// - specifies constraints which help the borrow checker
//
// syntax is about connecting the lifetimes of various
// parameters and return values of functions
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// struct which holds a reference
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // annotation of lifetime no required (lifetime elision rule 1)
    fn level(&self) -> i32 {
        3
    }

    // annotation of lifetime no required (lifetime elision rule 3)
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// putting generics, trait bound and lifetimes together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");

    {
        // -- inner scope = smallest lifetime for `'a` in `longest()`
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is '{}'", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:?}", i);

    let short = "abc";
    let long = "some long string slice";
    let announcement = String::from("Generics, Trait Bounds and Lifetimes together!");

    longest_with_an_announcement(short, long, announcement);
}
