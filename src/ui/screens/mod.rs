use iced::{Element, Task};
use crate::ui::message::Message;

pub mod create_event;

#[derive(Debug, Default)]
pub enum Screens {
    #[default]
    Home,
    Settings,
    CreateEvent(create_event::State),
}

pub trait Screen {
    fn update(&mut self, message: Message) -> Task<Message>;
    fn view<'a>(&self) -> Element<'a, Message>;
}
