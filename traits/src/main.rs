pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // We need to specify it as it doesnt have default implementation
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    // We dont need to specify below, so we can safely remove it as summarize has default implementation
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // Default Text -> we dont need to use it as we still override it in impl
        format!("Read more from {} ...", self.summarize_author())
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("Something Dark!"),
        content: String::from("Something is lurking in the woods brother, be aware..."),
    };

    println!("Tweet Summary: {}", tweet.summarize());
    println!("Article Summary: {}", article.summarize());
}
