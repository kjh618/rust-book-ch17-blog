pub mod post1;

#[cfg(test)]
mod tests {
    use crate::post1;

    #[test]
    fn post1_test() {
        // Draft
        let mut post = post1::Post::new();
        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        // Draft -> PendingReview
        post.request_review();
        assert_eq!("", post.content());

        // PendingReview -> Draft
        post.reject();
        assert_eq!("", post.content());

        // Draft -> PendingReview
        post.request_review();
        assert_eq!("", post.content());

        // PendingReview -> PendingReview
        for _ in 0..(post1::REQUIRED_APPROVE_COUNT - 1) {
            post.approve();
            assert_eq!("", post.content());
        }

        // PendingReview -> Published
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
