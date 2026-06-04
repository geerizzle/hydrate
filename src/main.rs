mod ui;
mod core;

fn main() -> iced::Result {
    iced::application(
        ui::Application::new,
        ui::Application::update,
        ui::Application::view,
    )
    .subscription(ui::Application::subscription)
    .run()
}
