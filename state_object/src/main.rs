use state_object::alt::AltPost;
use state_object::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate salad for lunch today", post.content());

    // ------------------- Alternative Lib Use -------------------

    let mut post = AltPost::new();

    post.add_text("I ate salad for lunch today");

    let post = post.request_review();

    let post = post.approve();
    assert_eq!("I ate salad for lunch today", post.content());
}
