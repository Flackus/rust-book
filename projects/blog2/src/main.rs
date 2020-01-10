use blog2::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    // Allow users to add text content only when a post is in the Draft state.
    // post.add_text("Lol, tricked you!");

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // Add a reject method that changes the post’s state from PendingReview back to Draft.
    let post = post.reject();
    //assert_eq!("I ate a salad for lunch today", post.content());
}
