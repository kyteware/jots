use iced::{Sandbox, Settings, widget, Length, alignment::{Vertical, Horizontal}};

use jots_widgets::TreeList;

fn main() {
    App::run(Settings::default()).unwrap();
}

struct App;

impl Sandbox for App {
    type Message = Message;
    fn new() -> App {
        App
    }

    fn title(&self) -> String {
        String::from("Hello World")
    }

    fn update(&mut self, message: Self::Message) {
        println!("Message: {:?}", message);
    }

    fn view(&self) -> iced::Element<Self::Message> {
        widget::row!(
            sidebar(),
            widget::Rule::vertical(0),
            widget::Space::new(10, Length::Fill),
            widget::text("main content")
        ).into()
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Hello
}

fn sidebar<'a>() -> widget::Container<'a, Message> {
    let body = widget::Column::new()
        .push(sections())
        .push(widget::Rule::horizontal(10))
        .push(explorer());

    widget::Container::new(body)
        .width(Length::Fixed(200.0))
        .height(Length::Fill)
}

fn sections<'a>() -> widget::Container<'a, Message> {
    let paths = vec!["assets/logo.svg", "assets/notes.svg", "assets/journal.svg", "assets/calendar.svg", "assets/folder.svg"];
    let mut body = widget::Row::new();

    for path in paths {
        let handle = widget::svg::Handle::from_path(path);
        body = body.push(widget::svg(handle));
    }

    widget::Container::new(body)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}

fn explorer<'a>() -> widget::Container<'a, Message> {
    let body = widget::Column::new()
        .push(widget::text("file 1"))
        .push(widget::text("file 2"))
        .push(widget::text("file 3"));

    widget::Container::new(body)
        .width(Length::Fill)
        .height(Length::Fill)
}