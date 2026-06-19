use iced::Element;
use iced::widget::{center, text};

fn main() -> iced::Result {
    iced::application(|| (), update, view)
        .title("GridCellar")
        .run()
}

fn update(_state: &mut (), _message: ()) {}

fn view(_state: &()) -> Element<'_, ()> {
    center(text("GridCellar implementation foundation")).into()
}
