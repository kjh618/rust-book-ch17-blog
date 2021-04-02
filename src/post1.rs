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
        if self.state.can_add_text() {
            self.content.push_str(text);
        }
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

    pub fn reject(&mut self) {
        let s = mem::replace(&mut self.state, Box::new(TemporaryState));
        self.state = s.reject();
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn can_add_text(&self) -> bool {
        false
    }

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

    fn reject(self: Box<Self>) -> Box<dyn State> {
        panic!()
    }

    fn can_add_text(&self) -> bool {
        panic!()
    }

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        panic!()
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approve_count: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn can_add_text(&self) -> bool {
        true
    }
}

struct PendingReview {
    approve_count: usize,
}

pub const REQUIRED_APPROVE_COUNT: usize = 2;

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.approve_count += 1;

        if self.approve_count >= REQUIRED_APPROVE_COUNT {
            Box::new(Published {})
        } else {
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
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

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
