use crate::{
    core::{event::ScheduledEvent, timer::TimerService},
    ui::{
        message::Message,
        resources::ResourceManager,
        screens::{Screen, Screens},
        widgets::navbar::NavBar,
    },
};
use iced::{
    Element, Task,
    time::{Duration, Instant},
    widget::{center, column, container, text},
};
use notify_rust::{Notification, Timeout};

#[derive(Debug)]
pub struct Application {
    screen: Screens,
    timer: TimerService,
    resources: ResourceManager,
    last_tick: Option<Instant>,
}

mod message;
mod resources;
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
                // state.screen = Screens::CreateEvent(create_event::State::default());
                let event = ScheduledEvent::builder()
                    .recurring(Duration::from_secs(10))
                    .with_name("TEST")
                    .activated(true)
                    .build();
                state.timer.push(event);
                Task::none()
            }
            Message::GoToSettings => {
                state.screen = Screens::Settings;
                Task::none()
            }
            Message::Tick(tick) => {
                let delta = state
                    .last_tick
                    .map(|last| tick.duration_since(last))
                    .unwrap_or(Duration::ZERO);
                state.last_tick = Some(tick);
                let to_fire = state.timer.tick(delta);
                Task::batch(
                    to_fire
                        .iter()
                        .map(|event| Task::done(Message::EventFired(event.clone()))),
                )
            }
            Message::EventFired(scheduled_event) => {
                let notification = Notification::new()
                    .summary(scheduled_event.event_name())
                    .body("This will almost look like a real firefox notification.")
                    .icon("firefox")
                    .timeout(Timeout::Milliseconds(6000)) //milliseconds
                    .show()
                    .unwrap();
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
