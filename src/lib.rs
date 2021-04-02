pub mod post1;
pub mod post2;

#[cfg(test)]
mod tests {
    #[test]
    fn post1_test() {
        use crate::post1::Post;

        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
