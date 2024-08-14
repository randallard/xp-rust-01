#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub done: bool,
    pub desc: Option<String>,
}

impl Task {
    pub fn new(title: impl Into<String>) -> Task {
        Task {
            title: title.into(),
            done: false,
            desc: None,
        }
    }
}