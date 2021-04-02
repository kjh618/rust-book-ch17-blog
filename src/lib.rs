pub mod post1;
pub mod post2;

#[cfg(test)]
mod tests {
    use crate::post1;
    use crate::post2;

    #[test]
    fn post1_test() {
        // Draft
        let mut post = post1::Post::new();
        post.add_text("a");
        assert_eq!("", post.content());

        // Draft -> PendingReview
        post.request_review();
        post.add_text("b");
        assert_eq!("", post.content());

        // PendingReview -> Draft
        post.reject();
        post.add_text("c");
        assert_eq!("", post.content());

        // Draft -> PendingReview
        post.request_review();
        post.add_text("d");
        assert_eq!("", post.content());

        // PendingReview -> PendingReview
        for _ in 0..(post1::REQUIRED_APPROVE_COUNT - 1) {
            post.approve();
            post.add_text("e");
            assert_eq!("", post.content());
        }

        // PendingReview -> Published
        post.approve();
        assert_eq!("ac", post.content());
    }

    #[test]
    fn post2_test() {
        let mut post = post2::Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.reject();

        let post = post.request_review();

        let post = post.approve();

        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
