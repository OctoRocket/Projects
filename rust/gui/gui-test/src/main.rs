use iced::{
    Sandbox,
    Element,
    Alignment,
    Settings,
    widget::{
        button, text, column
    },
    theme::{
        Theme
    },
};

struct Counter {
    value: i32,
}
impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter { value: (0) }
    }

    fn title(&self) -> String {
        String::from("Test app")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
    
    fn view(&self) -> Element<Message> {
        column![
            button("+").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("-").on_press(Message::DecrementPressed),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

fn main() -> iced::Result{
    Counter::run(Settings::default())
}
