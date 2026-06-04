use crate::{
    core::timer::TimerService,
    ui::{message::Message, screens::Screens, widgets::navbar::NavBar},
};
use iced::{
    Element, Task,
    time::{Duration, Instant},
    widget::column,
};

#[derive(Debug)]
pub struct Application {
    screen: Screens,
    timer: TimerService,
    last_tick: Option<Instant>,
}

mod message;
mod screens;
mod widgets;

impl Application {
    pub(crate) fn new() -> Self {
        Application {
            screen: Screens::default(),
            timer: TimerService::default(),
            last_tick: None,
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
            Message::Tick(tick) => {
                let _delta = state
                    .last_tick
                    .map(|last| tick.duration_since(last))
                    .unwrap_or(Duration::ZERO);
                state.last_tick = Some(tick);
                // state.timer.tick(delta);
                Task::none()
            }
        }
    }

    pub(crate) fn subscription(_: &Application) -> iced::Subscription<Message> {
        iced::time::every(Duration::from_secs(1)).map(Message::Tick)
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
