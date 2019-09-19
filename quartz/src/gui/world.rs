pub struct World {
    pub items: Vec<Box<dyn Visual>>,
}

pub trait Visual {}
