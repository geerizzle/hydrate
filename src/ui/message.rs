use iced::time::Instant;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    GoToHome,
    GoToSettings,
    CreateEvent,
    Tick(Instant),
}
