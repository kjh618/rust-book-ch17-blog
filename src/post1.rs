use std::mem;

pub struct Post {
    state: Box<dyn State>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Box::new(Draft {}),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.content(self)
    }

    pub fn request_review(&mut self) {
        let s = mem::replace(&mut self.state, Box::new(TemporaryState));
        self.state = s.request_review();
    }

    pub fn approve(&mut self) {
        let s = mem::replace(&mut self.state, Box::new(TemporaryState));
        self.state = s.approve();
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct TemporaryState;

impl State for TemporaryState {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        panic!()
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        panic!()
    }

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        panic!()
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
