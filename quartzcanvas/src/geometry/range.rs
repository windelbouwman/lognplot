//// A certain range with a begining and an end.
#[derive(Default, Clone)]
pub struct Range<T>
where
    T: Copy,
    T: Default,
{
    begin: T,
    end: T,
}

impl<T: Default + Copy> Range<T> {
    pub fn begin(&self) -> T {
        self.begin
    }

    pub fn set_begin(&mut self, begin: T) {
        self.begin = begin;
    }

    pub fn end(&self) -> T {
        self.end
    }

    pub fn set_end(&mut self, end: T) {
        self.end = end;
    }
}
