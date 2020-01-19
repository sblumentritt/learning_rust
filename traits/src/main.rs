pub trait Summary {
    // required implementation
    fn summarize_author(&self) -> String;

    // default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticel {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticel {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        self.author.to_string()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct RedditPost {
    pub content: String,
}

// use default implementation of trait
impl Summary for RedditPost {
    fn summarize_author(&self) -> String {
        String::from("[hidden]")
    }
}

pub fn notify_sugar(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_longer<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// clearer trait bounds with `where`
// -------------------------------------
// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {}
//
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let artical = NewsArticel {
        headline: String::from("Urgent news"),
        location: String::from("Worldwide"),
        author: String::from("Some dude"),
        content: String::from("Some click bait news articel without real content"),
    };

    let post = RedditPost {
        content: String::from("Here is a new blog post."),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new news artical: {}", artical.summarize());
    println!("1 new reddit post: {}", post.summarize());

    notify_sugar(&post);
    notify_longer(&post);
}
