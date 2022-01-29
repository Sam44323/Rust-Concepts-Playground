use state_pattern_design::*;

fn main() {
    let mut post = Post::new();

    post.add_text("This is the first blog!");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("This is the first blog!", post.content());
}
