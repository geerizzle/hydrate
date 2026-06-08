use iced::{
    Element, Length,
    widget::{Svg, Text, button, column, row, space::horizontal, svg},
};

use crate::ui::resources::{ResourceManager, SvgId};

pub struct Event<'a, Message: Clone> {
    title: Text<'a>,
    icon: Option<Svg<'a>>,
    on_press: Option<Message>,
    on_hover: Option<Message>,
}

impl<'a, Message> Event<'a, Message>
where
    Message: Clone,
{
    pub fn with_name(title: impl Into<Text<'a>>) -> Self {
        Self {
            title: title.into(),
            icon: None,
            on_press: None,
            on_hover: None,
        }
    }

    pub fn icon(mut self, icon: Svg<'a>) -> Self {
        self.icon = Some(icon);
        self
    }
}

impl<'a, Message> From<Event<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(item: Event<'a, Message>) -> Self {
        let mut content = column![];
        let title = row![item.title, horizontal().width(Length::Fill), item.icon]
            .width(Length::Fill)
            .padding(10);
        content = content.push(title);
        button(content).on_press(item.on_press.unwrap()).into()
    }
}
