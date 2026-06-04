use iced::{Element, Length, widget::{Button, button, row}};

pub struct NavBar<'a, Message: Clone> {
    pub home: Button<'a, Message>,
    pub settings: Button<'a, Message>,
    pub add_event: Button<'a, Message>,
}

impl <'a, Message> NavBar<'a, Message> where Message: Clone {
    pub fn new() -> Self {
        Self {
            home: button("Home"),
            settings: button("Settings"),
            add_event: button("Add")
        }
    }
    pub fn on_home_press(mut self, message: Message) -> Self {
        self.home = self.home.on_press(message);
        self
    }

    pub fn on_settings_press(mut self, message: Message) -> Self {
        self.settings = self.settings.on_press(message);
        self
    }

    pub fn on_add_press(mut self, message: Message) -> Self {
        self.add_event = self.add_event.on_press(message);
        self
    }
}
impl<'a, Message> From<NavBar<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(item: NavBar<'a, Message>) -> Self {
        let mut view = row![];
        view = view.push(item.settings);
        view = view.push(item.home);
        view = view.push(row![].width(Length::Fill));
        view = view.push(item.add_event);
        view.width(Length::Fill).spacing(10).into()
    }
}
