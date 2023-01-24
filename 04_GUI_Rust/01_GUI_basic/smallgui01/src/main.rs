// A Simple GUI Library

pub trait Widget {
    fn width(&self) -> usize;

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);
}

struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
}
