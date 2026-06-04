use iced::time::Duration;

#[derive(Debug, Clone)]
pub(crate) enum EventKind {
    OneShot(Duration),
    Recurring(Duration)
}

#[derive(Debug, Clone)]
pub struct ScheduledEvent {
    label: String,
    kind: EventKind,
    fired: bool,
    activated: bool,
}
