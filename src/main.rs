mod ui;

fn main() -> iced::Result {
    iced::application(
        ui::Application::new,
        ui::Application::update,
        ui::Application::view,
    )
    .run()
}
