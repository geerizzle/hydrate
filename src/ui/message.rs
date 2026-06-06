use iced::time::Instant;

use crate::core::event::ScheduledEvent;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    GoToHome,
    GoToSettings,
    CreateEvent,
    Tick(Instant),
    EventFired(ScheduledEvent)
}
