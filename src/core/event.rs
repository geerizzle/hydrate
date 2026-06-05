use iced::time::Duration;

#[derive(Debug, Clone, Default)]
pub(crate) enum EventKind {
    #[default]
    NotSet,
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

impl ScheduledEvent {
    pub fn builder() -> ScheduledEventBuilder {
        ScheduledEventBuilder {
            label: String::new(),
            kind: EventKind::OneShot(Duration::ZERO),
            activated: true,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ScheduledEventBuilder {
    label: String,
    kind: EventKind,
    activated: bool,
}

impl ScheduledEventBuilder {
    pub fn with_name(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn one_shot(mut self, duration: Duration) -> Self {
        self.kind = EventKind::OneShot(duration);
        self
    }

    pub fn recurring(mut self, duration: Duration) -> Self {
        self.kind = EventKind::Recurring(duration);
        self
    }

    pub fn activated(mut self, activated: bool) -> Self {
        self.activated = activated;
        self
    }

    pub fn build(self) -> ScheduledEvent {
        ScheduledEvent {
            label: self.label,
            kind: self.kind,
            fired: false,
            activated: self.activated,
        }
    }
}
