use state_pattern_design::*;

fn main() {
    let mut draft_post = Post::new();
    draft_post.add_text("This is a draft text!");
    let pending_review_post = draft_post.request_review();
    let post = pending_review_post.approve();
    println!("Content for the post: {}", post.content());
}
