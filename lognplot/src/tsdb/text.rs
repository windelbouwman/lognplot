#[derive(Clone, Debug)]
pub struct Text {
    pub text: String,
}

impl Text {
    pub fn new(text: String) -> Self {
        Text { text }
    }
}
