use iced::{Element, Task, widget::{container, text}};

use crate::{core::event::{ScheduledEvent, ScheduledEventBuilder}, ui::{message::Message, screens::Screen}};

#[derive(Debug, Default)]
pub struct State {
    event: ScheduledEventBuilder,
}

impl Screen for State {
    fn update(&mut self, _message: Message) -> Task<Message> {
        todo!()
    }

    fn view<'a>(&self) -> Element<'a, Message> {
        container(text("To be implemented")).into()
    }
}
