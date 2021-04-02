pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approve_count: 0,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    approve_count: usize,
}

pub enum ApproveResult {
    NotEnough(PendingReviewPost),
    Done(Post),
}

pub const REQUIRED_APPROVE_COUNT: usize = 2;

impl PendingReviewPost {
    pub fn approve(mut self) -> ApproveResult {
        self.approve_count += 1;

        if self.approve_count >= REQUIRED_APPROVE_COUNT {
            ApproveResult::Done(Post {
                content: self.content,
            })
        } else {
            ApproveResult::NotEnough(self)
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

impl ApproveResult {
    pub fn unwrap_not_enough(self) -> PendingReviewPost {
        if let ApproveResult::NotEnough(p) = self {
            p
        } else {
            panic!()
        }
    }

    pub fn unwrap_done(self) -> Post {
        if let ApproveResult::Done(p) = self {
            p
        } else {
            panic!()
        }
    }
}
