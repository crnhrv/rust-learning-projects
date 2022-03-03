use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    
    let post = post.reject();

    let mut post = post.request_review();

    let potential_post = post.approve();

    match potential_post {
        Some(v) => {
            assert_eq!("I ate a salad for lunch today", v.content());
        }
        None => {
            let post  = post.approve().unwrap();
            assert_eq!("I ate a salad for lunch today", post.content());
        }
    }; 

    
}