// A Simple GUI Libray

pub trait Widget {
    fn width(&self) -> usize;

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{}", &buffer);
    }
}

struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

struct Label {
    label: String,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let mut inner = String::new();
        for widget in &self.widgets {
            widget.draw_into(&mut inner);
        }

        let window_width = self.width();

        writeln!(buffer, "+={:-<window_width$}=+", "").unwrap();
        writeln!(buffer, "| {:^window_width$} |", &self.title).unwrap();
        writeln!(buffer, "+={:=<window_width$} |", "").unwrap();
        for line in inner.lines() {
            writeln!(buffer, "| {:window_width$} |", line).unwrap();
        }
        writeln!(buffer, "+-{:<window_width$}-+", "").unwrap();
    }
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        self.label
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0)
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        writeln!(buffer, "{}", &self.label).unwrap();
    }
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI Demo.")));
}
