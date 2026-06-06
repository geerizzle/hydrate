use iced::time::Duration;

#[derive(Debug, Clone, Default)]
pub(crate) enum EventKind {
    #[default]
    NotSet,
    OneShot(Duration),
    Recurring(Duration),
}

#[derive(Debug, Clone)]
pub struct ScheduledEvent {
    label: String,
    kind: EventKind,
    fired: bool,
    remaining: Duration,
    activated: bool,
}

impl ScheduledEvent {
    pub fn event_name(&self) -> &String {
        &self.label
    }

    pub fn is_active(&self) -> bool {
        self.activated
    }

    pub fn remaining(&self) -> Duration {
        self.remaining
    }

    pub fn has_been_fired(&self) -> bool {
        self.fired
    }

    pub fn tick(&mut self, interval: Duration) -> bool {
        if !self.activated || self.fired || self.remaining.is_zero() {
            return false;
        }
        if interval >= self.remaining {
            let was_recurring = matches!(self.kind, EventKind::Recurring(_));
            self.remaining = match self.kind {
                EventKind::Recurring(d) => d,
                _ => Duration::ZERO,
            };
            self.fired = !was_recurring;
            true
        } else {
            self.remaining = self.remaining.saturating_sub(interval);
            false
        }
    }

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
        let remaining = match self.kind {
            EventKind::OneShot(d) | EventKind::Recurring(d) => d,
            EventKind::NotSet => Duration::ZERO,
        };
        ScheduledEvent {
            label: self.label,
            remaining,
            kind: self.kind,
            fired: false,
            activated: self.activated,
        }
    }
}
