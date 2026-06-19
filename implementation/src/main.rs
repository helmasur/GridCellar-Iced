use iced::widget::{
    button, column, container, opaque, pick_list, responsive, row, rule, stack, text, text_input,
};
use iced::{Element, Fill, Length, Size};

const NARROW_WIDTH: f32 = 720.0;

#[derive(Default)]
struct App {
    search: String,
    selected_view: Option<&'static str>,
    selected_range: Option<&'static str>,
    panel: Option<Panel>,
}

#[derive(Clone, Copy, Debug)]
enum Panel {
    Detail,
    Configuration,
    Filters,
    DateFields,
}

#[derive(Clone, Debug)]
enum Message {
    SearchChanged(String),
    ViewSelected(&'static str),
    RangeSelected(&'static str),
    AddObject,
    OpenConfiguration,
    OpenFilters,
    OpenDateFields,
    ClosePanel,
}

fn main() -> iced::Result {
    iced::application(App::default, update, view)
        .title("GridCellar")
        .run()
}

fn update(app: &mut App, message: Message) {
    match message {
        Message::SearchChanged(value) => app.search = value,
        Message::ViewSelected(value) => app.selected_view = Some(value),
        Message::RangeSelected(value) => app.selected_range = Some(value),
        Message::AddObject => app.panel = Some(Panel::Detail),
        Message::OpenConfiguration => app.panel = Some(Panel::Configuration),
        Message::OpenFilters => app.panel = Some(Panel::Filters),
        Message::OpenDateFields => app.panel = Some(Panel::DateFields),
        Message::ClosePanel => app.panel = None,
    }
}

fn view(app: &App) -> Element<'_, Message> {
    responsive(move |size| {
        if size.width < NARROW_WIDTH {
            narrow_layout(app, size)
        } else {
            wide_layout(app, size)
        }
    })
    .into()
}

fn wide_layout(app: &App, size: Size) -> Element<'_, Message> {
    let top_bar = row![
        text("Min källare").size(24),
        view_picker(app),
        range_picker(app),
        text_input("Sök objekt", &app.search)
            .on_input(Message::SearchChanged)
            .width(Fill),
        button("Lägg till objekt").on_press(Message::AddObject),
        button("Konfiguration").on_press(Message::OpenConfiguration),
    ]
    .spacing(12)
    .align_y(iced::Center);

    page(top_bar.into(), size, app.panel)
}

fn narrow_layout(app: &App, size: Size) -> Element<'_, Message> {
    let primary = row![
        view_picker(app),
        range_picker(app),
        button("Lägg till objekt").on_press(Message::AddObject),
    ]
    .spacing(8);
    let secondary = row![
        text_input("Sök objekt", &app.search)
            .on_input(Message::SearchChanged)
            .width(Fill),
        button("Konfiguration").on_press(Message::OpenConfiguration),
    ]
    .spacing(8);

    page(
        column![text("Min källare").size(22), primary, secondary]
            .spacing(8)
            .into(),
        size,
        app.panel,
    )
}

fn page<'a>(
    top_bar: Element<'a, Message>,
    size: Size,
    panel: Option<Panel>,
) -> Element<'a, Message> {
    let controls = row![
        button("Filter (0)").on_press(Message::OpenFilters),
        button("Nivå 1"),
        button("Nivå 2"),
        button("Nivå 3"),
        button("Datumfält (0/0)").on_press(Message::OpenDateFields),
        button("Passa in alla datum"),
    ]
    .spacing(8);

    let diagram = container(
        column![
            text("Diagram").size(22),
            text("Tidslinjediagrammets huvudyta"),
            text(format!(
                "Tillgänglig yta: {:.0} × {:.0}",
                size.width, size.height
            ))
            .size(12),
        ]
        .spacing(8),
    )
    .padding(20)
    .width(Fill)
    .height(Fill)
    .style(container::bordered_box);

    let main = container(
        column![top_bar, rule::horizontal(1), controls, diagram]
            .spacing(12)
            .height(Fill),
    )
    .padding(12)
    .width(Fill)
    .height(Fill)
    .into();

    match panel {
        Some(panel) => stack![main, panel_overlay(panel, size)].into(),
        None => main,
    }
}

fn panel_overlay(panel: Panel, size: Size) -> Element<'static, Message> {
    let narrow = size.width < NARROW_WIDTH;
    let (title, content) = match panel {
        Panel::Detail => (
            "Nytt objekt",
            "Detaljpanelen används senare för skapande, visning och redigering.",
        ),
        Panel::Configuration => (
            "Konfiguration",
            "Sektioner: Fält, Diagram, Etikett och Projekt.",
        ),
        Panel::Filters => ("Filter", "Här visas och redigeras den aktiva vyns filter."),
        Panel::DateFields => (
            "Datumfält",
            "Här väljs vilka datumfält som visas i den aktiva vyn.",
        ),
    };

    let panel = container(
        column![
            row![
                text(title).size(24).width(Fill),
                button("Stäng").on_press(Message::ClosePanel),
            ]
            .align_y(iced::Center),
            rule::horizontal(1),
            text(content),
        ]
        .spacing(16),
    )
    .padding(20)
    .width(if narrow { Fill } else { Length::Fixed(520.0) })
    .height(if narrow { Fill } else { Length::Fixed(360.0) })
    .style(container::bordered_box);

    opaque(
        container(panel)
            .padding(if narrow { 8 } else { 40 })
            .center_x(Fill)
            .center_y(Fill)
            .width(Fill)
            .height(Fill)
            .style(container::dark),
    )
    .into()
}

fn view_picker(app: &App) -> Element<'_, Message> {
    pick_list(
        ["Alla objekt"],
        app.selected_view.or(Some("Alla objekt")),
        Message::ViewSelected,
    )
    .placeholder("Vy")
    .into()
}

fn range_picker(app: &App) -> Element<'_, Message> {
    pick_list(
        ["Visa allt", "5 år", "10 år", "Egen period"],
        app.selected_range.or(Some("Visa allt")),
        Message::RangeSelected,
    )
    .placeholder("Tidsintervall")
    .into()
}
