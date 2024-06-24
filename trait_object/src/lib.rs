pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // 'dyn' -> stands for dynamic dispatch
    pub components: Vec<Box<dyn Draw>>,
}
