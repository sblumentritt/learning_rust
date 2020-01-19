// allow debug format printing for the struct
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    // use 'field init shorthand' syntax which is possible if the parameter
    // and the field have the same name
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // fields do not have to be specified in the same order
    // they were declared
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1: {:?}", user1);

    // to make a field mutable, the entire instance must be mutable
    // use 'struct update' syntax to use all not explicitly set variables from another struct
    let mut user2 = User {
        username: String::from("anotherusername123"),
        active: false,
        ..user1
    };

    // change the value of the email field
    user2.email = String::from("another@example.com");

    println!("user2: {:#?}", user2);

    let user3 = build_user(
        String::from("another2@example.com"),
        String::from("anotherusername456"),
    );

    println!("user3: {:#?}", user3);
}
