use iced::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub(crate) enum Message {
    GoToHome,
    GoToSettings,
    AddNotification,
    Tick(Instant)
}
