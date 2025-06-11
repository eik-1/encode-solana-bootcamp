pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet : Tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World"),
        reply: false,
        retweet: false,
    };

    println!("Tweet summary: {}", tweet.summarize());
}