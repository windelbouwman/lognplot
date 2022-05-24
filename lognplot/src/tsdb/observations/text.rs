use super::CountMetrics;

#[derive(Clone, Debug)]
pub struct Text {
    pub text: String,
}

impl Text {
    pub fn new(text: String) -> Self {
        Text { text }
    }
}

// This is a bit lame, but impl<V> From<V> for CountMetrics conflicts with a builtin From implementation.
impl From<Text> for CountMetrics {
    fn from(_observation: Text) -> Self {
        CountMetrics { count: 1 }
    }
}
