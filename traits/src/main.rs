use std::fmt::{Debug, Display};

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
    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }
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

#[allow(dead_code)]
struct Pair<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[allow(dead_code)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 'Blanket' example: We can conditionally implement a trait for any type that implements another trait.
// impl<T: Display> ToString for T {
//     // ...
// }

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

    notify(&article);

    println!("{}", returns_summarizable().summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("devil_books"),
        content: String::from("of course, as you probably already know, demons..."),
        reply: false,
        retweet: false,
    }
}

// We cannot use below as this can return 2 different types, we can use impl trait with 1 type output
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// Some syntax comparison \\

// ------------------------------------------------------------------------------------- \\

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// Same as above, but longer version
pub fn _notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

// ------------------------------------------------------------------------------------- \\

pub fn example_one(_item1: &(impl Summary + Display), _item2: &impl Summary) {
    // ...
}

pub fn example_two<T: Summary + Display>(_item1: &T, _item2: &T) {
    // ...
}

// ------------------------------------------------------------------------------------- \\

#[allow(dead_code)]
fn some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    666
}

#[allow(dead_code)]
fn some_fn<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    666
}
