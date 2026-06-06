use crate::core::event::ScheduledEvent;
use iced::time::Duration;

#[derive(Default, Debug)]
pub(crate) struct TimerService {
    events: Vec<ScheduledEvent>,
}

impl TimerService {
    pub fn tick(&mut self, interval: Duration) -> Vec<ScheduledEvent> {
        let mut fired = vec![];
        for event in &mut self.events {
            if event.tick(interval) {
                fired.push(event.clone());
            }
        }
        fired
    }

    pub fn push(&mut self, event: ScheduledEvent) {
        self.events.push(event);
    }
}
