use crate::ui::{message::Message, screens::Screens, widgets::navbar::NavBar};
use iced::{Element, Task, widget::column};

#[derive(Debug, Clone, Default)]
pub struct Application {
    screen: Screens,
}

mod message;
mod screens;
mod widgets;

impl Application {
    pub(crate) fn new() -> Self {
        Application {
            screen: Screens::default(),
        }
    }

    pub(crate) fn update(state: &mut Application, message: Message) -> Task<Message> {
        match message {
            Message::GoToHome => {
                state.screen = Screens::Home;
                Task::none()
            }
            Message::AddNotification => {
                state.screen = Screens::NewNotification;
                Task::none()
            }
            Message::GoToSettings => {
                state.screen = Screens::Settings;
                Task::none()
            }
        }
    }

    pub(crate) fn view(_state: &Application) -> Element<'_, Message> {
        let mut view = column![];
        let nav_bar = NavBar::new()
            .on_settings_press(Message::GoToSettings)
            .on_home_press(Message::GoToHome)
            .on_add_press(Message::AddNotification);
        view = view.push(nav_bar);
        view.into()
    }
}
