pub trait Draw {
    fn draw(&self);
}

// ---------------- Generic Approach ----------------

// This approach is limited to store data for 1 component type, so for example slider.
// We cannot store mixture of few components, like button, slider, background etc.
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// pub struct Screen {
//     // 'dyn' -> stands for dynamic dispatch
//     pub components: Vec<Box<dyn Draw>>,
// }

// impl Screen {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }
