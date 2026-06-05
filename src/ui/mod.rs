use crate::{
    core::timer::TimerService,
    ui::{message::Message, resources::ResourceManager, screens::{Screen, Screens, create_event}, widgets::navbar::NavBar},
};
use iced::{
    Element, Task,
    time::{Duration, Instant},
    widget::{center, column, container, text},
};

#[derive(Debug)]
pub struct Application {
    screen: Screens,
    timer: TimerService,
    resources: ResourceManager,
    last_tick: Option<Instant>,
}

mod resources;
mod message;
mod screens;
mod widgets;

impl Application {
    pub(crate) fn new() -> Self {
        let mut resources = ResourceManager::new();
        resources.load_all();
        Application {
            screen: Screens::default(),
            timer: TimerService::default(),
            resources,
            last_tick: None,
        }
    }

    pub(crate) fn update(state: &mut Application, message: Message) -> Task<Message> {
        match message {
            Message::GoToHome => {
                state.screen = Screens::Home;
                Task::none()
            }
            Message::CreateEvent => {
                state.screen = Screens::CreateEvent(create_event::State::default());
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

    pub(crate) fn view(state: &Application) -> Element<'_, Message> {
        let mut view = column![];
        let nav_bar = NavBar::new(&state.resources)
            .on_settings_press(Message::GoToSettings)
            .on_home_press(Message::GoToHome)
            .on_add_press(Message::CreateEvent);
        view = view.push(nav_bar);
        let content: Element<'_, Message> = match state.screen {
            Screens::Home => container(text("Currently Home Screen")).into(),
            Screens::Settings => container(text("This is settigns")).into(),
            Screens::CreateEvent(ref state) => state.view(),
        };
        view = view.push(center(content));
        view.into()
    }
}
