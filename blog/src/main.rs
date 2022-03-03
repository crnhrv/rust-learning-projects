use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("we're still in draft stages so I can add text");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.reject();

    post.approve();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    post.approve();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("we're still in draft stages so I can add text", post.content());
}