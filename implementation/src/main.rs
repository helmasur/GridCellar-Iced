use gridcellar::model::{Project, ProjectId, TimeRange, ViewId};
use iced::widget::{
    button, column, container, opaque, pick_list, responsive, row, rule, stack, text, text_input,
};
use iced::{Element, Fill, Length, Size};

const NARROW_WIDTH: f32 = 720.0;

struct App {
    project: Project,
    search: String,
    selected_view: Option<&'static str>,
    panel: Option<Panel>,
    project_settings: ProjectSettingsDraft,
}

impl Default for App {
    fn default() -> Self {
        let project = Project::empty(
            ProjectId::new("default-project"),
            ViewId::new("all-objects"),
        );
        let project_settings = ProjectSettingsDraft::from_project(&project);
        Self {
            project,
            search: String::new(),
            selected_view: None,
            panel: None,
            project_settings,
        }
    }
}

#[derive(Clone, Debug)]
struct ProjectSettingsDraft {
    name: String,
    row_height: String,
    name_column_width: String,
    time_range: &'static str,
    custom_start: String,
    custom_end: String,
}

impl ProjectSettingsDraft {
    fn from_project(project: &Project) -> Self {
        Self {
            name: project.name.clone(),
            row_height: project.diagram_settings.row_height.to_string(),
            name_column_width: project.diagram_settings.name_column_width.to_string(),
            time_range: time_range_label(&project.diagram_settings.time_range),
            custom_start: custom_start(&project.diagram_settings.time_range),
            custom_end: custom_end(&project.diagram_settings.time_range),
        }
    }
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
    ProjectNameChanged(String),
    RowHeightChanged(String),
    NameColumnWidthChanged(String),
    CustomStartChanged(String),
    CustomEndChanged(String),
    SaveProjectSettings,
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
        Message::RangeSelected(value) => {
            app.project.diagram_settings.time_range = time_range_from_label(value);
            app.project_settings.time_range = value;
        }
        Message::ProjectNameChanged(value) => app.project_settings.name = value,
        Message::RowHeightChanged(value) => app.project_settings.row_height = value,
        Message::NameColumnWidthChanged(value) => app.project_settings.name_column_width = value,
        Message::CustomStartChanged(value) => app.project_settings.custom_start = value,
        Message::CustomEndChanged(value) => app.project_settings.custom_end = value,
        Message::SaveProjectSettings => {
            if let (Ok(row_height), Ok(name_column_width)) = (
                app.project_settings.row_height.parse(),
                app.project_settings.name_column_width.parse(),
            ) {
                let name = app.project_settings.name.trim();
                if !name.is_empty() {
                    app.project.name = name.to_owned();
                    app.project.diagram_settings.row_height = row_height;
                    app.project.diagram_settings.name_column_width = name_column_width;
                    app.project.diagram_settings.time_range = time_range_from_draft(
                        app.project_settings.time_range,
                        &app.project_settings.custom_start,
                        &app.project_settings.custom_end,
                    );
                    app.panel = None;
                }
            }
        }
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
        text(&app.project.name).size(24),
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

    page(app, top_bar.into(), size, app.panel)
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
        app,
        column![text(&app.project.name).size(22), primary, secondary]
            .spacing(8)
            .into(),
        size,
        app.panel,
    )
}

fn page<'a>(
    app: &'a App,
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
            row![
                container(text("Objektnamn"))
                    .padding(8)
                    .width(Length::Fixed(
                        app.project.diagram_settings.name_column_width as f32
                    ))
                    .height(Length::Fixed(
                        app.project.diagram_settings.row_height as f32
                    ))
                    .style(container::bordered_box),
                container(text("Tidslinje"))
                    .padding(8)
                    .width(Fill)
                    .height(Length::Fixed(
                        app.project.diagram_settings.row_height as f32
                    ))
                    .style(container::bordered_box),
            ],
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
        Some(panel) => stack![main, panel_overlay(app, panel, size)].into(),
        None => main,
    }
}

fn panel_overlay(app: &App, panel: Panel, size: Size) -> Element<'_, Message> {
    let narrow = size.width < NARROW_WIDTH;
    let (title, content): (&str, Element<'_, Message>) = match panel {
        Panel::Detail => (
            "Nytt objekt",
            text("Detaljpanelen används senare för skapande, visning och redigering.").into(),
        ),
        Panel::Configuration => ("Konfiguration", project_settings(app)),
        Panel::Filters => (
            "Filter",
            text("Här visas och redigeras den aktiva vyns filter.").into(),
        ),
        Panel::DateFields => (
            "Datumfält",
            text("Här väljs vilka datumfält som visas i den aktiva vyn.").into(),
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
            content,
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
        Some(time_range_label(&app.project.diagram_settings.time_range)),
        Message::RangeSelected,
    )
    .placeholder("Tidsintervall")
    .into()
}

fn project_settings(app: &App) -> Element<'_, Message> {
    column![
        text("Projekt").size(20),
        text_input("Projektnamn", &app.project_settings.name).on_input(Message::ProjectNameChanged),
        text("Diagram").size(20),
        row![
            column![
                text("Radhöjd"),
                text_input("40", &app.project_settings.row_height)
                    .on_input(Message::RowHeightChanged),
            ]
            .spacing(4),
            column![
                text("Namnkolumnens bredd"),
                text_input("240", &app.project_settings.name_column_width)
                    .on_input(Message::NameColumnWidthChanged),
            ]
            .spacing(4),
        ]
        .spacing(12),
        text("Tidsintervall").size(20),
        pick_list(
            ["Visa allt", "5 år", "10 år", "Egen period"],
            Some(app.project_settings.time_range),
            Message::RangeSelected,
        ),
        if app.project_settings.time_range == "Egen period" {
            row![
                text_input("Start YYYY-MM-DD", &app.project_settings.custom_start)
                    .on_input(Message::CustomStartChanged),
                text_input("Slut YYYY-MM-DD", &app.project_settings.custom_end)
                    .on_input(Message::CustomEndChanged),
            ]
            .spacing(12)
        } else {
            row![]
        },
        button("Spara").on_press(Message::SaveProjectSettings),
    ]
    .spacing(12)
    .into()
}

fn time_range_label(time_range: &TimeRange) -> &'static str {
    match time_range {
        TimeRange::ShowAll => "Visa allt",
        TimeRange::FiveYears => "5 år",
        TimeRange::TenYears => "10 år",
        TimeRange::Custom { .. } => "Egen period",
    }
}

fn time_range_from_label(label: &str) -> TimeRange {
    time_range_from_draft(label, "2026-01-01", "2026-12-31")
}

fn time_range_from_draft(label: &str, start: &str, end: &str) -> TimeRange {
    match label {
        "5 år" => TimeRange::FiveYears,
        "10 år" => TimeRange::TenYears,
        "Egen period" => TimeRange::Custom {
            start: gridcellar::model::CalendarDate::new(start),
            end: gridcellar::model::CalendarDate::new(end),
        },
        _ => TimeRange::ShowAll,
    }
}

fn custom_start(time_range: &TimeRange) -> String {
    match time_range {
        TimeRange::Custom { start, .. } => start.as_str().to_owned(),
        _ => "2026-01-01".to_owned(),
    }
}

fn custom_end(time_range: &TimeRange) -> String {
    match time_range {
        TimeRange::Custom { end, .. } => end.as_str().to_owned(),
        _ => "2026-12-31".to_owned(),
    }
}
