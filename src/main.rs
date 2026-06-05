mod core;
mod ui;

fn main() -> iced::Result {
    iced::application(
        ui::Application::new,
        ui::Application::update,
        ui::Application::view,
    )
    .subscription(ui::Application::subscription)
    .theme(iced::theme::Theme::Dark)
    .run()
}
