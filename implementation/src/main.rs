use gridcellar::editor::{DraftValue, ObjectDraft};
use gridcellar::label::diagram_label;
use gridcellar::model::{
    DetailFormat, Field, FieldId, FieldType, FieldValue, ListValue, ListValueId, ObjectId, Project,
    ProjectId, TimeRange, ValueMode, View, ViewId,
};
use gridcellar::validation::{ValidationError, can_remove_list_value, can_remove_or_change_field};
use iced::widget::{
    button, checkbox, column, container, opaque, pick_list, responsive, row, rule, scrollable,
    stack, text, text_input,
};
use iced::{Element, Fill, Length, Size};

const NARROW_WIDTH: f32 = 720.0;

struct App {
    project: Project,
    search: String,
    active_view_id: ViewId,
    view_draft: View,
    view_dirty: bool,
    view_name_draft: String,
    next_view_number: usize,
    filter_draft: FilterDraft,
    panel: Option<Panel>,
    project_settings: ProjectSettingsDraft,
    new_field: FieldDraft,
    field_status: Option<String>,
    next_field_number: usize,
    new_list_value_names: std::collections::BTreeMap<FieldId, String>,
    next_list_value_number: usize,
    selected_object_id: Option<ObjectId>,
    object_draft: Option<ObjectDraft>,
    saved_object_draft: Option<ObjectDraft>,
    object_status: Option<String>,
    next_object_number: usize,
}

impl Default for App {
    fn default() -> Self {
        let project = Project::empty(
            ProjectId::new("default-project"),
            ViewId::new("all-objects"),
        );
        let project_settings = ProjectSettingsDraft::from_project(&project);
        let view_draft = project.views[0].clone();
        Self {
            active_view_id: project.last_used_view_id.clone(),
            view_name_draft: view_draft.name.clone(),
            view_draft,
            view_dirty: false,
            next_view_number: 2,
            filter_draft: FilterDraft::default(),
            project,
            search: String::new(),
            panel: None,
            project_settings,
            new_field: FieldDraft::default(),
            field_status: None,
            next_field_number: 1,
            new_list_value_names: std::collections::BTreeMap::new(),
            next_list_value_number: 1,
            selected_object_id: None,
            object_draft: None,
            saved_object_draft: None,
            object_status: None,
            next_object_number: 1,
        }
    }
}

#[derive(Clone, Debug)]
struct FieldDraft {
    name: String,
    field_type: &'static str,
    value_mode: &'static str,
    required: bool,
    detail_format: &'static str,
}

#[derive(Clone, Debug)]
struct FilterDraft {
    field_name: Option<String>,
    operator: &'static str,
    operands: String,
    include_empty: bool,
}

impl Default for FilterDraft {
    fn default() -> Self {
        Self {
            field_name: None,
            operator: "Är tomt",
            operands: String::new(),
            include_empty: false,
        }
    }
}

impl Default for FieldDraft {
    fn default() -> Self {
        Self {
            name: String::new(),
            field_type: "Text",
            value_mode: "Ett värde",
            required: false,
            detail_format: "Normal rad",
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
    ViewSelected(String),
    NewView,
    SaveView,
    ViewNameChanged(String),
    RenameView,
    DuplicateView,
    ResetView,
    DeleteView,
    GroupingSelected(usize, String),
    ToggleGroupingDirection(usize),
    ToggleDateField(FieldId, bool),
    FilterFieldSelected(String),
    FilterOperatorSelected(&'static str),
    FilterOperandsChanged(String),
    FilterIncludeEmptyChanged(bool),
    AddFilter,
    RemoveFilter(usize),
    RangeSelected(&'static str),
    ProjectNameChanged(String),
    RowHeightChanged(String),
    NameColumnWidthChanged(String),
    CustomStartChanged(String),
    CustomEndChanged(String),
    SaveProjectSettings,
    NewFieldNameChanged(String),
    NewFieldTypeChanged(&'static str),
    NewFieldValueModeChanged(&'static str),
    NewFieldRequiredChanged(bool),
    NewFieldFormatChanged(&'static str),
    CreateField,
    RenameField(usize, String),
    ChangeFieldType(usize, &'static str),
    ChangeFieldValueMode(usize, &'static str),
    ChangeFieldRequired(usize, bool),
    ChangeFieldFormat(usize, &'static str),
    MoveFieldUp(usize),
    MoveFieldDown(usize),
    RemoveField(usize),
    NewListValueNameChanged(FieldId, String),
    CreateListValue(FieldId),
    RenameListValue(ListValueId, String),
    MoveListValueUp(ListValueId),
    MoveListValueDown(ListValueId),
    RemoveListValue(ListValueId),
    AddLabelField(FieldId),
    MoveLabelFieldUp(usize),
    MoveLabelFieldDown(usize),
    RemoveLabelField(usize),
    StartEditingObject,
    DraftValueChanged(FieldId, usize, String),
    AddDraftValue(FieldId),
    RemoveDraftValue(FieldId, usize),
    MoveDraftValueUp(FieldId, usize),
    MoveDraftValueDown(FieldId, usize),
    SelectListValue(FieldId, usize, String),
    ChooseImage(FieldId),
    RemoveImage(FieldId),
    SaveObject,
    ResetObject,
    DuplicateObject,
    RequestDeleteObject,
    ConfirmDeleteObject,
    CancelDeleteObject,
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
        Message::ViewSelected(value) => select_view(app, &value),
        Message::NewView => new_view(app),
        Message::SaveView => save_view(app),
        Message::ViewNameChanged(value) => app.view_name_draft = value,
        Message::RenameView => rename_view(app),
        Message::DuplicateView => duplicate_view(app),
        Message::ResetView => reset_view(app),
        Message::DeleteView => delete_view(app),
        Message::GroupingSelected(index, value) => set_grouping(app, index, &value),
        Message::ToggleGroupingDirection(index) => toggle_grouping_direction(app, index),
        Message::ToggleDateField(field_id, included) => {
            if included {
                app.view_draft.excluded_date_field_ids.retain(|id| id != &field_id);
            } else if !app.view_draft.excluded_date_field_ids.contains(&field_id) {
                app.view_draft.excluded_date_field_ids.push(field_id);
            }
            app.view_dirty = true;
        }
        Message::FilterFieldSelected(value) => app.filter_draft.field_name = Some(value),
        Message::FilterOperatorSelected(value) => app.filter_draft.operator = value,
        Message::FilterOperandsChanged(value) => app.filter_draft.operands = value,
        Message::FilterIncludeEmptyChanged(value) => app.filter_draft.include_empty = value,
        Message::AddFilter => add_filter(app),
        Message::RemoveFilter(index) => {
            if index < app.view_draft.filters.len() {
                app.view_draft.filters.remove(index);
                app.view_dirty = true;
            }
        }
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
        Message::NewFieldNameChanged(value) => app.new_field.name = value,
        Message::NewFieldTypeChanged(value) => app.new_field.field_type = value,
        Message::NewFieldValueModeChanged(value) => app.new_field.value_mode = value,
        Message::NewFieldRequiredChanged(value) => app.new_field.required = value,
        Message::NewFieldFormatChanged(value) => app.new_field.detail_format = value,
        Message::CreateField => create_field(app),
        Message::RenameField(index, value) => rename_field(app, index, value),
        Message::ChangeFieldType(index, value) => change_field_type(app, index, value),
        Message::ChangeFieldValueMode(index, value) => change_field_value_mode(app, index, value),
        Message::ChangeFieldRequired(index, value) => change_field_required(app, index, value),
        Message::ChangeFieldFormat(index, value) => change_field_format(app, index, value),
        Message::MoveFieldUp(index) => {
            if index > 0 {
                app.project.fields.swap(index, index - 1);
            }
        }
        Message::MoveFieldDown(index) => {
            if index + 1 < app.project.fields.len() {
                app.project.fields.swap(index, index + 1);
            }
        }
        Message::RemoveField(index) => remove_field(app, index),
        Message::NewListValueNameChanged(field_id, value) => {
            app.new_list_value_names.insert(field_id, value);
        }
        Message::CreateListValue(field_id) => create_list_value(app, &field_id),
        Message::RenameListValue(list_value_id, value) => {
            rename_list_value(app, &list_value_id, value)
        }
        Message::MoveListValueUp(list_value_id) => move_list_value(app, &list_value_id, true),
        Message::MoveListValueDown(list_value_id) => move_list_value(app, &list_value_id, false),
        Message::RemoveListValue(list_value_id) => remove_list_value(app, &list_value_id),
        Message::AddLabelField(field_id) => add_label_field(app, field_id),
        Message::MoveLabelFieldUp(index) => {
            if index > 0 {
                app.project.diagram_label_field_ids.swap(index, index - 1);
            }
        }
        Message::MoveLabelFieldDown(index) => {
            if index + 1 < app.project.diagram_label_field_ids.len() {
                app.project.diagram_label_field_ids.swap(index, index + 1);
            }
        }
        Message::RemoveLabelField(index) => remove_label_field(app, index),
        Message::AddObject => start_creating_object(app),
        Message::OpenConfiguration => app.panel = Some(Panel::Configuration),
        Message::OpenFilters => app.panel = Some(Panel::Filters),
        Message::OpenDateFields => app.panel = Some(Panel::DateFields),
        Message::ClosePanel => close_panel(app),
        Message::StartEditingObject => start_editing_object(app),
        Message::DraftValueChanged(field_id, index, value) => {
            set_draft_text(app, &field_id, index, value)
        }
        Message::AddDraftValue(field_id) => add_draft_value(app, &field_id),
        Message::RemoveDraftValue(field_id, index) => remove_draft_value(app, &field_id, index),
        Message::MoveDraftValueUp(field_id, index) => move_draft_value(app, &field_id, index, true),
        Message::MoveDraftValueDown(field_id, index) => {
            move_draft_value(app, &field_id, index, false)
        }
        Message::SelectListValue(field_id, index, value) => {
            if let Some(id) = app
                .project
                .list_values
                .iter()
                .find(|item| item.field_id == field_id && item.name == value)
                .map(|item| item.id.as_str().to_owned())
            {
                set_draft_text(app, &field_id, index, id);
            }
        }
        Message::ChooseImage(field_id) => choose_image(app, &field_id),
        Message::RemoveImage(field_id) => {
            if let Some(draft) = &mut app.object_draft {
                draft.values.insert(field_id, Vec::new());
            }
        }
        Message::SaveObject => save_object(app),
        Message::ResetObject => reset_object(app),
        Message::DuplicateObject => duplicate_object(app),
        Message::RequestDeleteObject => {
            app.object_status = Some("Bekräfta radering: åtgärden kan inte ångras.".to_owned())
        }
        Message::ConfirmDeleteObject => delete_object(app),
        Message::CancelDeleteObject => app.object_status = None,
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
        text_input("Vynamn", &app.view_name_draft).on_input(Message::ViewNameChanged),
        button("Ny vy").on_press(Message::NewView),
        button("Spara vy").on_press(Message::SaveView),
        button("Byt namn").on_press(Message::RenameView),
        button("Duplicera vy").on_press(Message::DuplicateView),
        button("Återställ vy").on_press(Message::ResetView),
        button("Ta bort vy").on_press(Message::DeleteView),
        button("Filter (0)").on_press(Message::OpenFilters),
        grouping_picker(app, 0),
        grouping_picker(app, 1),
        grouping_picker(app, 2),
        button("Datumfält (0/0)").on_press(Message::OpenDateFields),
        button("Passa in alla datum"),
    ]
    .spacing(8);

    let diagram = container(
        column![
            text("Diagram").size(22),
            text("Tidslinjediagrammets huvudyta"),
            text(if app.view_dirty {
                "Osparade vyändringar"
            } else {
                "Vyn är sparad"
            })
            .size(12),
            row![
                container(text(preview_diagram_label(app)))
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
        Panel::Detail => ("Objekt", detail_view(app)),
        Panel::Configuration => ("Konfiguration", configuration_panel(app)),
        Panel::Filters => ("Filter", filters_panel(app)),
        Panel::DateFields => ("Datumfält", date_fields_panel(app)),
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
    let options: Vec<String> = app.project.views.iter().map(|view| view.name.clone()).collect();
    let selected = app
        .project
        .views
        .iter()
        .find(|view| view.id == app.active_view_id)
        .map(|view| view.name.clone());
    pick_list(
        options,
        selected,
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

fn grouping_picker(app: &App, index: usize) -> Element<'_, Message> {
    let mut options = vec!["Ingen".to_owned()];
    options.extend(
        app.project
            .fields
            .iter()
            .filter(|field| field.field_type != FieldType::Image)
            .map(|field| field.name.clone()),
    );
    let selected = app
        .view_draft
        .grouping
        .get(index)
        .and_then(|group| {
            app.project
                .fields
                .iter()
                .find(|field| field.id == group.field_id)
                .map(|field| field.name.clone())
        })
        .unwrap_or_else(|| "Ingen".to_owned());
    let direction = app
        .view_draft
        .grouping
        .get(index)
        .map(|group| match group.direction {
            gridcellar::model::SortDirection::Ascending => "↑",
            gridcellar::model::SortDirection::Descending => "↓",
        })
        .unwrap_or("↑");
    row![
        pick_list(options, Some(selected), move |value| {
            Message::GroupingSelected(index, value)
        }),
        button(direction).on_press(Message::ToggleGroupingDirection(index)),
    ]
    .spacing(4)
    .into()
}

fn date_fields_panel(app: &App) -> Element<'_, Message> {
    let mut content = column![].spacing(8);
    for field in app
        .project
        .fields
        .iter()
        .filter(|field| field.field_type == FieldType::Date)
    {
        let included = !app.view_draft.excluded_date_field_ids.contains(&field.id);
        let field_id = field.id.clone();
        content = content.push(
            checkbox(included)
                .label(&field.name)
                .on_toggle(move |value| Message::ToggleDateField(field_id.clone(), value)),
        );
    }
    if !app
        .project
        .fields
        .iter()
        .any(|field| field.field_type == FieldType::Date)
    {
        content = content.push(text("Projektet saknar datumfält."));
    }
    content.into()
}

fn filters_panel(app: &App) -> Element<'_, Message> {
    let mut content = column![].spacing(10);
    for (index, filter) in app.view_draft.filters.iter().enumerate() {
        let field_name = app
            .project
            .fields
            .iter()
            .find(|field| field.id == filter.field_id)
            .map(|field| field.name.as_str())
            .unwrap_or("Saknat fält");
        content = content.push(
            row![
                text(format!(
                    "{} · {:?} · {} operand(er){}",
                    field_name,
                    filter.operator,
                    filter.operands.len(),
                    if filter.include_empty {
                        " · inkluderar tomma"
                    } else {
                        ""
                    }
                ))
                .width(Fill),
                button("Ta bort").on_press(Message::RemoveFilter(index)),
            ]
            .spacing(8),
        );
    }

    let field_options: Vec<String> = app
        .project
        .fields
        .iter()
        .filter(|field| field.field_type != FieldType::Image)
        .map(|field| field.name.clone())
        .collect();
    content
        .push(rule::horizontal(1))
        .push(pick_list(
            field_options,
            app.filter_draft.field_name.clone(),
            Message::FilterFieldSelected,
        ).placeholder("Fält"))
        .push(pick_list(
            filter_operator_options(),
            Some(app.filter_draft.operator),
            Message::FilterOperatorSelected,
        ))
        .push(
            text_input(
                "Värde; använd semikolon för intervall eller flera listvärden",
                &app.filter_draft.operands,
            )
            .on_input(Message::FilterOperandsChanged),
        )
        .push(
            checkbox(app.filter_draft.include_empty)
                .label("Inkludera tomma värden")
                .on_toggle(Message::FilterIncludeEmptyChanged),
        )
        .push(button("Lägg till filter").on_press(Message::AddFilter))
        .push(text(app.field_status.as_deref().unwrap_or("")))
        .into()
}

fn filter_operator_options() -> [&'static str; 11] {
    [
        "Innehåller",
        "Är exakt",
        "Lika med",
        "Större än",
        "Mindre än",
        "Intervall",
        "Före",
        "Efter",
        "Mellan",
        "Är någon av",
        "Är tomt",
    ]
}

fn configuration_panel(app: &App) -> Element<'_, Message> {
    scrollable(
        column![
            project_settings(app),
            rule::horizontal(1),
            field_administration(app),
            rule::horizontal(1),
            label_administration(app),
        ]
        .spacing(20),
    )
    .into()
}

fn detail_view(app: &App) -> Element<'_, Message> {
    if let Some(draft) = &app.object_draft {
        return object_editor(app, draft);
    }

    let object = app
        .selected_object_id
        .as_ref()
        .and_then(|id| app.project.objects.iter().find(|object| object.id == *id))
        .or_else(|| app.project.objects.first());

    let Some(object) = object else {
        return column![
            text("Inget objekt är valt."),
            text("Objekt kan skapas när skapandeläget implementeras i arbetspaket 6.2.").size(12),
        ]
        .spacing(8)
        .into();
    };

    let mut fields = column![
        text(format!("Internt id: {}", object.id.as_str())).size(12),
        row![
            button("Redigera").on_press(Message::StartEditingObject),
            button("Duplicera").on_press(Message::DuplicateObject),
            button("Ta bort").on_press(Message::RequestDeleteObject),
        ]
        .spacing(8),
    ]
    .spacing(12);
    if app.object_status.is_some() {
        fields = fields.push(
            column![
                text(app.object_status.as_deref().unwrap_or("")),
                row![
                    button("Bekräfta radering").on_press(Message::ConfirmDeleteObject),
                    button("Avbryt").on_press(Message::CancelDeleteObject),
                ]
                .spacing(8),
            ]
            .spacing(6),
        );
    }
    for field in &app.project.fields {
        let values = object
            .values
            .get(&field.id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        fields = fields.push(detail_field(app, field, values));
    }

    scrollable(fields).into()
}

fn object_editor<'a>(app: &'a App, draft: &'a ObjectDraft) -> Element<'a, Message> {
    let mut form = column![].spacing(12);
    for field in &app.project.fields {
        form = form.push(object_field_editor(app, draft, field));
    }
    form = form
        .push(text(app.object_status.as_deref().unwrap_or("")))
        .push(
            row![
                button("Spara").on_press(Message::SaveObject),
                button("Återställ").on_press(Message::ResetObject),
            ]
            .spacing(8),
        );
    scrollable(form).into()
}

fn object_field_editor<'a>(
    app: &'a App,
    draft: &'a ObjectDraft,
    field: &'a Field,
) -> Element<'a, Message> {
    let values = draft
        .values
        .get(&field.id)
        .map(Vec::as_slice)
        .unwrap_or_default();
    let mut content = column![text(format!(
        "{}{}",
        field.name,
        if field.required { " *" } else { "" }
    ))]
    .spacing(6);

    if field.field_type == FieldType::Image {
        let has_image = values
            .iter()
            .any(|value| matches!(value, DraftValue::Image(bytes) if !bytes.is_empty()));
        return content
            .push(text(if has_image { "Bild vald" } else { "—" }))
            .push(
                row![
                    button("Välj bild").on_press(Message::ChooseImage(field.id.clone())),
                    button("Ta bort").on_press(Message::RemoveImage(field.id.clone())),
                ]
                .spacing(8),
            )
            .into();
    }

    let visible_values = values.len().max(1);
    for index in 0..visible_values {
        let value = values
            .get(index)
            .and_then(|value| match value {
                DraftValue::Text(value) => Some(value.as_str()),
                DraftValue::Image(_) => None,
            })
            .unwrap_or("");
        let field_id = field.id.clone();
        let input: Element<'_, Message> = if field.field_type == FieldType::List {
            let options: Vec<String> = app
                .project
                .list_values
                .iter()
                .filter(|item| item.field_id == field.id)
                .map(|item| item.name.clone())
                .collect();
            let selected = app
                .project
                .list_values
                .iter()
                .find(|item| item.id.as_str() == value)
                .map(|item| item.name.clone());
            pick_list(options, selected, move |selected| {
                Message::SelectListValue(field_id.clone(), index, selected)
            })
            .placeholder("Välj listvärde")
            .into()
        } else {
            text_input(input_placeholder(&field.field_type), value)
                .on_input(move |value| Message::DraftValueChanged(field_id.clone(), index, value))
                .into()
        };

        content = content.push(
            row![
                input,
                button("Upp").on_press(Message::MoveDraftValueUp(field.id.clone(), index)),
                button("Ned").on_press(Message::MoveDraftValueDown(field.id.clone(), index)),
                button("Ta bort").on_press(Message::RemoveDraftValue(field.id.clone(), index)),
            ]
            .spacing(6),
        );
    }

    if field.value_mode == ValueMode::Multiple {
        content = content
            .push(button("Lägg till värde").on_press(Message::AddDraftValue(field.id.clone())));
    }
    content.into()
}

fn input_placeholder(field_type: &FieldType) -> &'static str {
    match field_type {
        FieldType::Text => "Text",
        FieldType::Number(_) => "Tal",
        FieldType::Date => "YYYY-MM-DD",
        FieldType::List => "Listvärde",
        FieldType::Image => "",
    }
}

fn detail_field<'a>(
    app: &'a App,
    field: &'a Field,
    values: &'a [FieldValue],
) -> Element<'a, Message> {
    let formatted: Vec<String> = values
        .iter()
        .filter_map(|value| display_field_value(app, value))
        .collect();
    let empty = formatted.is_empty();
    let combined = if empty {
        "—".to_owned()
    } else {
        formatted.join(", ")
    };

    match field.detail_format {
        DetailFormat::Title => {
            if empty {
                column![text(&field.name).size(14), text("—").size(22)]
                    .spacing(2)
                    .into()
            } else {
                text(combined).size(26).into()
            }
        }
        DetailFormat::Chips => {
            let mut chips = row![].spacing(6);
            if empty {
                chips = chips.push(container(text("—")).padding(6));
            } else {
                for value in formatted {
                    chips = chips.push(
                        container(text(value))
                            .padding(6)
                            .style(container::bordered_box),
                    );
                }
            }
            column![text(&field.name).size(14), chips].spacing(4).into()
        }
        DetailFormat::LongText => column![text(&field.name).size(14), text(combined).width(Fill),]
            .spacing(4)
            .into(),
        DetailFormat::Image => column![
            text(&field.name).size(14),
            container(text(if empty { "—" } else { "Bild lagrad" }))
                .padding(20)
                .width(Fill)
                .style(container::bordered_box),
        ]
        .spacing(4)
        .into(),
        DetailFormat::Date | DetailFormat::Number | DetailFormat::NormalRow => row![
            text(&field.name).width(Length::FillPortion(1)),
            text(combined).width(Length::FillPortion(2)),
        ]
        .spacing(12)
        .into(),
    }
}

fn display_field_value(app: &App, value: &FieldValue) -> Option<String> {
    match value {
        FieldValue::Text(value) => {
            let value = value.trim();
            (!value.is_empty()).then(|| value.to_owned())
        }
        FieldValue::Integer(value) => Some(value.to_string()),
        FieldValue::Decimal(value) => Some(value.to_string()),
        FieldValue::Date(value) => Some(value.as_str().to_owned()),
        FieldValue::List(id) => app
            .project
            .list_values
            .iter()
            .find(|item| item.id == *id)
            .map(|item| item.name.clone()),
        FieldValue::Image(_) => Some("Bild lagrad".to_owned()),
    }
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

fn field_administration(app: &App) -> Element<'_, Message> {
    let mut fields = column![text("Fält").size(20)].spacing(10);

    if app.project.fields.is_empty() {
        fields = fields.push(text("Projektet saknar fält."));
    }

    for (index, field) in app.project.fields.iter().enumerate() {
        let field_id = field.id.clone();
        let object_count = app
            .project
            .objects
            .iter()
            .filter(|object| {
                object
                    .values
                    .get(&field_id)
                    .is_some_and(|values| !values.is_empty())
            })
            .count();
        let view_count = app
            .project
            .views
            .iter()
            .filter(|view| {
                view.grouping.iter().any(|group| group.field_id == field_id)
                    || view
                        .filters
                        .iter()
                        .any(|filter| filter.field_id == field_id)
                    || view.excluded_date_field_ids.contains(&field_id)
            })
            .count();
        let label_use = app.project.diagram_label_field_ids.contains(&field_id);
        let usage = format!(
            "Används i: etikett {}, {} vyer, {} objektvärden",
            if label_use { "ja" } else { "nej" },
            view_count,
            object_count
        );

        let mut field_content = column![
            text_input("Fältnamn", &field.name)
                .on_input(move |value| Message::RenameField(index, value)),
            row![
                pick_list(
                    field_type_options(),
                    Some(field_type_label(&field.field_type)),
                    move |value| Message::ChangeFieldType(index, value),
                ),
                pick_list(
                    value_mode_options(),
                    Some(value_mode_label(field.value_mode)),
                    move |value| Message::ChangeFieldValueMode(index, value),
                ),
                pick_list(
                    detail_format_options(),
                    Some(detail_format_label(field.detail_format)),
                    move |value| Message::ChangeFieldFormat(index, value),
                ),
            ]
            .spacing(8),
            checkbox(field.required)
                .label("Obligatoriskt")
                .on_toggle(move |value| Message::ChangeFieldRequired(index, value)),
            text(usage).size(12),
            row![
                button("Upp").on_press(Message::MoveFieldUp(index)),
                button("Ned").on_press(Message::MoveFieldDown(index)),
                button("Ta bort").on_press(Message::RemoveField(index)),
            ]
            .spacing(8),
        ]
        .spacing(8);

        if field.field_type == FieldType::List {
            field_content = field_content
                .push(rule::horizontal(1))
                .push(list_value_administration(app, &field.id));
        }

        fields = fields.push(
            container(field_content)
                .padding(10)
                .style(container::bordered_box),
        );
    }

    let new_field = column![
        text("Lägg till fält").size(18),
        text_input("Fältnamn", &app.new_field.name).on_input(Message::NewFieldNameChanged),
        row![
            pick_list(
                field_type_options(),
                Some(app.new_field.field_type),
                Message::NewFieldTypeChanged,
            ),
            pick_list(
                value_mode_options(),
                Some(app.new_field.value_mode),
                Message::NewFieldValueModeChanged,
            ),
            pick_list(
                detail_format_options(),
                Some(app.new_field.detail_format),
                Message::NewFieldFormatChanged,
            ),
        ]
        .spacing(8),
        checkbox(app.new_field.required)
            .label("Obligatoriskt")
            .on_toggle(Message::NewFieldRequiredChanged),
        button("Skapa fält").on_press(Message::CreateField),
    ]
    .spacing(8);

    let status: Element<'_, Message> = app
        .field_status
        .as_deref()
        .map(|status| text(status).into())
        .unwrap_or_else(|| text("").into());

    fields.push(new_field).push(status).into()
}

fn label_administration(app: &App) -> Element<'_, Message> {
    let mut content = column![text("Global diagrametikett").size(20)].spacing(8);

    if app.project.fields.is_empty() {
        return content
            .push(text("Etiketten kan vara tom när projektet saknar fält."))
            .into();
    }

    for (index, field_id) in app.project.diagram_label_field_ids.iter().enumerate() {
        let name = app
            .project
            .fields
            .iter()
            .find(|field| field.id == *field_id)
            .map(|field| field.name.as_str())
            .unwrap_or("Saknat fält");
        content = content.push(
            row![
                text(name).width(Fill),
                button("Upp").on_press(Message::MoveLabelFieldUp(index)),
                button("Ned").on_press(Message::MoveLabelFieldDown(index)),
                button("Ta bort").on_press(Message::RemoveLabelField(index)),
            ]
            .spacing(6),
        );
    }

    if app.project.diagram_label_field_ids.len() < 5 {
        for field in app.project.fields.iter().filter(|field| {
            !app.project.diagram_label_field_ids.contains(&field.id)
                && field.field_type != FieldType::Image
        }) {
            content = content.push(
                button(text(format!("Lägg till {}", field.name)))
                    .on_press(Message::AddLabelField(field.id.clone())),
            );
        }
    }

    content
        .push(text("Fältvärden separeras med:  – ").size(12))
        .into()
}

fn list_value_administration<'a>(app: &'a App, field_id: &'a FieldId) -> Element<'a, Message> {
    let mut values = column![text("Listvärden").size(16)].spacing(6);
    let mut list_values: Vec<&ListValue> = app
        .project
        .list_values
        .iter()
        .filter(|value| value.field_id == *field_id)
        .collect();
    list_values.sort_by_key(|value| value.order);

    if list_values.is_empty() {
        values = values.push(text("Listan är tom."));
    }

    for value in list_values {
        let value_id = value.id.clone();
        let object_count = app
            .project
            .objects
            .iter()
            .filter(|object| {
                object.values.values().flatten().any(|field_value| {
                    matches!(
                        field_value,
                        gridcellar::model::FieldValue::List(id) if id == &value_id
                    )
                })
            })
            .count();
        let filter_count = app
            .project
            .views
            .iter()
            .flat_map(|view| &view.filters)
            .filter(|filter| {
                filter.operands.iter().any(|operand| {
                    matches!(
                        operand,
                        gridcellar::model::FilterOperand::ListValue(id) if id == &value_id
                    )
                })
            })
            .count();
        let rename_id = value.id.clone();
        let up_id = value.id.clone();
        let down_id = value.id.clone();
        let remove_id = value.id.clone();

        values = values.push(
            column![
                row![
                    text_input("Listvärde", &value.name)
                        .on_input(move |name| Message::RenameListValue(rename_id.clone(), name))
                        .width(Fill),
                    button("Upp").on_press(Message::MoveListValueUp(up_id)),
                    button("Ned").on_press(Message::MoveListValueDown(down_id)),
                    button("Ta bort").on_press(Message::RemoveListValue(remove_id)),
                ]
                .spacing(6),
                text(format!(
                    "Används av {object_count} objekt och {filter_count} filter"
                ))
                .size(12),
            ]
            .spacing(4),
        );
    }

    let new_name = app
        .new_list_value_names
        .get(field_id)
        .map(String::as_str)
        .unwrap_or("");
    let change_field_id = field_id.clone();
    let create_field_id = field_id.clone();

    values
        .push(
            row![
                text_input("Nytt listvärde", new_name)
                    .on_input(move |name| Message::NewListValueNameChanged(
                        change_field_id.clone(),
                        name
                    ))
                    .width(Fill),
                button("Lägg till").on_press(Message::CreateListValue(create_field_id)),
            ]
            .spacing(6),
        )
        .into()
}

fn create_field(app: &mut App) {
    let name = app.new_field.name.trim();
    if name.is_empty()
        || app
            .project
            .fields
            .iter()
            .any(|field| field.name.eq_ignore_ascii_case(name))
    {
        app.field_status = Some("Fältnamnet måste vara ifyllt och unikt.".to_owned());
        return;
    }
    if app.new_field.required && !app.project.objects.is_empty() {
        app.field_status =
            Some("Nya fält kan inte vara obligatoriska när objekt redan finns.".to_owned());
        return;
    }

    let field_type = field_type_from_label(app.new_field.field_type);
    let value_mode = if field_type == FieldType::Image {
        ValueMode::Single
    } else {
        value_mode_from_label(app.new_field.value_mode)
    };
    app.project.fields.push(Field {
        id: FieldId::new(format!("field-{}", app.next_field_number)),
        project_id: app.project.id.clone(),
        name: name.to_owned(),
        field_type,
        value_mode,
        required: app.new_field.required,
        detail_format: detail_format_from_label(app.new_field.detail_format),
    });
    app.next_field_number += 1;
    app.new_field = FieldDraft::default();
    app.field_status = Some("Fältet skapades.".to_owned());
}

fn rename_field(app: &mut App, index: usize, value: String) {
    let duplicate = app
        .project
        .fields
        .iter()
        .enumerate()
        .any(|(other, field)| other != index && field.name.eq_ignore_ascii_case(value.trim()));
    if value.trim().is_empty() || duplicate {
        app.field_status = Some("Fältnamnet måste vara ifyllt och unikt.".to_owned());
    } else if let Some(field) = app.project.fields.get_mut(index) {
        field.name = value;
        app.field_status = None;
    }
}

fn change_field_type(app: &mut App, index: usize, value: &'static str) {
    let Some(field_id) = app.project.fields.get(index).map(|field| field.id.clone()) else {
        return;
    };
    if let Err(errors) = can_remove_or_change_field(&app.project, &field_id) {
        app.field_status = Some(field_errors(&errors));
        return;
    }
    if let Some(field) = app.project.fields.get_mut(index) {
        field.field_type = field_type_from_label(value);
        if field.field_type == FieldType::Image {
            field.value_mode = ValueMode::Single;
            field.detail_format = DetailFormat::Image;
        }
    }
    app.field_status = None;
}

fn change_field_value_mode(app: &mut App, index: usize, value: &'static str) {
    let Some(field) = app.project.fields.get(index) else {
        return;
    };
    if field.field_type == FieldType::Image {
        app.field_status = Some("Bildfält kan endast ha ett värde.".to_owned());
        return;
    }
    let field_id = field.id.clone();
    if app.project.objects.iter().any(|object| {
        object
            .values
            .get(&field_id)
            .is_some_and(|values| !values.is_empty())
    }) {
        app.field_status = Some("Värdeläge kan bara ändras när fältet är tomt.".to_owned());
        return;
    }
    app.project.fields[index].value_mode = value_mode_from_label(value);
    app.field_status = None;
}

fn change_field_required(app: &mut App, index: usize, required: bool) {
    let Some(field) = app.project.fields.get(index) else {
        return;
    };
    if required
        && app
            .project
            .objects
            .iter()
            .any(|object| object.values.get(&field.id).is_none_or(Vec::is_empty))
    {
        app.field_status =
            Some("Alla objekt måste ha ett värde innan fältet blir obligatoriskt.".to_owned());
        return;
    }
    app.project.fields[index].required = required;
    app.field_status = None;
}

fn change_field_format(app: &mut App, index: usize, value: &'static str) {
    if let Some(field) = app.project.fields.get_mut(index) {
        field.detail_format = detail_format_from_label(value);
        app.field_status = None;
    }
}

fn remove_field(app: &mut App, index: usize) {
    let Some(field_id) = app.project.fields.get(index).map(|field| field.id.clone()) else {
        return;
    };
    match can_remove_or_change_field(&app.project, &field_id) {
        Ok(()) => {
            app.project.fields.remove(index);
            app.project
                .list_values
                .retain(|value| value.field_id != field_id);
            app.field_status = Some("Fältet togs bort.".to_owned());
        }
        Err(errors) => app.field_status = Some(field_errors(&errors)),
    }
}

fn create_list_value(app: &mut App, field_id: &FieldId) {
    let name = app
        .new_list_value_names
        .get(field_id)
        .map(|name| name.trim())
        .unwrap_or("");
    if name.is_empty()
        || app
            .project
            .list_values
            .iter()
            .any(|value| value.field_id == *field_id && value.name.eq_ignore_ascii_case(name))
    {
        app.field_status = Some("Listvärdesnamnet måste vara ifyllt och unikt.".to_owned());
        return;
    }

    let order = app
        .project
        .list_values
        .iter()
        .filter(|value| value.field_id == *field_id)
        .count();
    app.project.list_values.push(ListValue {
        id: ListValueId::new(format!("list-value-{}", app.next_list_value_number)),
        field_id: field_id.clone(),
        name: name.to_owned(),
        order,
    });
    app.next_list_value_number += 1;
    app.new_list_value_names.remove(field_id);
    app.field_status = Some("Listvärdet skapades.".to_owned());
}

fn rename_list_value(app: &mut App, list_value_id: &ListValueId, name: String) {
    let Some(field_id) = app
        .project
        .list_values
        .iter()
        .find(|value| value.id == *list_value_id)
        .map(|value| value.field_id.clone())
    else {
        return;
    };
    let duplicate = app.project.list_values.iter().any(|value| {
        value.field_id == field_id
            && value.id != *list_value_id
            && value.name.eq_ignore_ascii_case(name.trim())
    });
    if name.trim().is_empty() || duplicate {
        app.field_status = Some("Listvärdesnamnet måste vara ifyllt och unikt.".to_owned());
    } else if let Some(value) = app
        .project
        .list_values
        .iter_mut()
        .find(|value| value.id == *list_value_id)
    {
        value.name = name;
        app.field_status = None;
    }
}

fn move_list_value(app: &mut App, list_value_id: &ListValueId, upward: bool) {
    let Some(current) = app
        .project
        .list_values
        .iter()
        .find(|value| value.id == *list_value_id)
        .cloned()
    else {
        return;
    };
    let target_order = if upward {
        current.order.checked_sub(1)
    } else {
        Some(current.order + 1)
    };
    let Some(target_order) = target_order else {
        return;
    };
    let Some(target_id) = app
        .project
        .list_values
        .iter()
        .find(|value| value.field_id == current.field_id && value.order == target_order)
        .map(|value| value.id.clone())
    else {
        return;
    };

    for value in &mut app.project.list_values {
        if value.id == current.id {
            value.order = target_order;
        } else if value.id == target_id {
            value.order = current.order;
        }
    }
}

fn remove_list_value(app: &mut App, list_value_id: &ListValueId) {
    match can_remove_list_value(&app.project, list_value_id) {
        Ok(()) => {
            let field_id = app
                .project
                .list_values
                .iter()
                .find(|value| value.id == *list_value_id)
                .map(|value| value.field_id.clone());
            app.project
                .list_values
                .retain(|value| value.id != *list_value_id);
            if let Some(field_id) = field_id {
                let mut values: Vec<&mut ListValue> = app
                    .project
                    .list_values
                    .iter_mut()
                    .filter(|value| value.field_id == field_id)
                    .collect();
                values.sort_by_key(|value| value.order);
                for (order, value) in values.into_iter().enumerate() {
                    value.order = order;
                }
            }
            app.field_status = Some("Listvärdet togs bort.".to_owned());
        }
        Err(ValidationError::ListValueIsUsed {
            object_count,
            filter_count,
            ..
        }) => {
            app.field_status = Some(format!(
                "Listvärdet används av {object_count} objekt och {filter_count} filter."
            ));
        }
        Err(_) => {
            app.field_status = Some("Listvärdet kan inte tas bort.".to_owned());
        }
    }
}

fn add_label_field(app: &mut App, field_id: FieldId) {
    if app.project.diagram_label_field_ids.len() < 5
        && !app.project.diagram_label_field_ids.contains(&field_id)
    {
        app.project.diagram_label_field_ids.push(field_id);
        app.field_status = None;
    }
}

fn remove_label_field(app: &mut App, index: usize) {
    if app.project.fields.is_empty() || app.project.diagram_label_field_ids.len() > 1 {
        if index < app.project.diagram_label_field_ids.len() {
            app.project.diagram_label_field_ids.remove(index);
            app.field_status = None;
        }
    } else {
        app.field_status = Some("Minst ett etikettfält krävs när projektet har fält.".to_owned());
    }
}

fn preview_diagram_label(app: &App) -> String {
    app.project
        .objects
        .first()
        .map(|object| diagram_label(&app.project, object))
        .unwrap_or_else(|| "Objektnamn".to_owned())
}

fn start_creating_object(app: &mut App) {
    if app.project.fields.is_empty() {
        app.panel = Some(Panel::Configuration);
        app.field_status = Some("Skapa minst ett fält innan du lägger till objekt.".to_owned());
        return;
    }
    let draft = ObjectDraft::empty(&app.project);
    app.saved_object_draft = Some(draft.clone());
    app.object_draft = Some(draft);
    app.selected_object_id = None;
    app.object_status = None;
    app.panel = Some(Panel::Detail);
}

fn start_editing_object(app: &mut App) {
    let object = app
        .selected_object_id
        .as_ref()
        .and_then(|id| app.project.objects.iter().find(|object| object.id == *id))
        .or_else(|| app.project.objects.first());
    if let Some(object) = object {
        let draft = ObjectDraft::from_object(&app.project, object);
        app.saved_object_draft = Some(draft.clone());
        app.object_draft = Some(draft);
        app.selected_object_id = Some(object.id.clone());
    }
}

fn close_panel(app: &mut App) {
    if app.object_draft != app.saved_object_draft {
        app.object_status =
            Some("Osparade ändringar finns. Välj Återställ innan panelen stängs.".to_owned());
        return;
    }
    app.panel = None;
    app.object_draft = None;
    app.saved_object_draft = None;
}

fn set_draft_text(app: &mut App, field_id: &FieldId, index: usize, value: String) {
    let Some(draft) = &mut app.object_draft else {
        return;
    };
    let values = draft.values.entry(field_id.clone()).or_default();
    while values.len() <= index {
        values.push(DraftValue::Text(String::new()));
    }
    values[index] = DraftValue::Text(value);
}

fn add_draft_value(app: &mut App, field_id: &FieldId) {
    if let Some(draft) = &mut app.object_draft {
        draft
            .values
            .entry(field_id.clone())
            .or_default()
            .push(DraftValue::Text(String::new()));
    }
}

fn remove_draft_value(app: &mut App, field_id: &FieldId, index: usize) {
    if let Some(values) = app
        .object_draft
        .as_mut()
        .and_then(|draft| draft.values.get_mut(field_id))
    {
        if index < values.len() {
            values.remove(index);
        }
    }
}

fn move_draft_value(app: &mut App, field_id: &FieldId, index: usize, upward: bool) {
    let Some(values) = app
        .object_draft
        .as_mut()
        .and_then(|draft| draft.values.get_mut(field_id))
    else {
        return;
    };
    let target = if upward {
        index.checked_sub(1)
    } else if index + 1 < values.len() {
        Some(index + 1)
    } else {
        None
    };
    if let Some(target) = target {
        values.swap(index, target);
    }
}

fn choose_image(app: &mut App, field_id: &FieldId) {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("Images", &["png", "jpg", "jpeg", "webp"])
        .pick_file()
    else {
        return;
    };
    match std::fs::read(path) {
        Ok(bytes) => {
            if let Some(draft) = &mut app.object_draft {
                draft
                    .values
                    .insert(field_id.clone(), vec![DraftValue::Image(bytes)]);
            }
            app.object_status = None;
        }
        Err(_) => app.object_status = Some("Bilden kunde inte läsas.".to_owned()),
    }
}

fn save_object(app: &mut App) {
    let Some(draft) = app.object_draft.clone() else {
        return;
    };
    let object_id = draft
        .object_id
        .clone()
        .unwrap_or_else(|| ObjectId::new(format!("object-{}", app.next_object_number)));
    match draft.to_object(&app.project, object_id.clone()) {
        Ok(object) => {
            if let Some(existing) = app
                .project
                .objects
                .iter_mut()
                .find(|existing| existing.id == object_id)
            {
                *existing = object;
            } else {
                app.project.objects.push(object);
                app.next_object_number += 1;
            }
            app.selected_object_id = Some(object_id);
            app.object_draft = None;
            app.saved_object_draft = None;
            app.object_status = None;
        }
        Err(errors) => {
            app.object_status = Some(format!("Objektet kan inte sparas: {} fel.", errors.len()));
        }
    }
}

fn reset_object(app: &mut App) {
    app.object_draft = app.saved_object_draft.clone();
    app.object_status = None;
}

fn select_view(app: &mut App, name: &str) {
    if let Some(view) = app.project.views.iter().find(|view| view.name == name) {
        app.active_view_id = view.id.clone();
        app.project.last_used_view_id = view.id.clone();
        app.view_draft = view.clone();
        app.view_name_draft = view.name.clone();
        app.view_dirty = false;
    }
}

fn new_view(app: &mut App) {
    let id = ViewId::new(format!("view-{}", app.next_view_number));
    let name = unique_view_name(app, "Ny vy");
    let view = View {
        id: id.clone(),
        project_id: app.project.id.clone(),
        name: name.clone(),
        grouping: Vec::new(),
        filters: Vec::new(),
        excluded_date_field_ids: Vec::new(),
    };
    app.next_view_number += 1;
    app.project.views.push(view.clone());
    app.active_view_id = id;
    app.view_draft = view;
    app.view_name_draft = name;
    app.view_dirty = false;
}

fn save_view(app: &mut App) {
    if let Some(view) = app
        .project
        .views
        .iter_mut()
        .find(|view| view.id == app.active_view_id)
    {
        *view = app.view_draft.clone();
        app.view_dirty = false;
    }
}

fn rename_view(app: &mut App) {
    let name = app.view_name_draft.trim();
    if name.is_empty()
        || app
            .project
            .views
            .iter()
            .any(|view| view.id != app.active_view_id && view.name.eq_ignore_ascii_case(name))
    {
        return;
    }
    app.view_draft.name = name.to_owned();
    app.view_dirty = true;
    save_view(app);
}

fn duplicate_view(app: &mut App) {
    let mut view = app.view_draft.clone();
    view.id = ViewId::new(format!("view-{}", app.next_view_number));
    view.name = unique_view_name(app, &format!("{} kopia", view.name));
    app.next_view_number += 1;
    app.active_view_id = view.id.clone();
    app.view_name_draft = view.name.clone();
    app.project.views.push(view.clone());
    app.view_draft = view;
    app.view_dirty = false;
}

fn reset_view(app: &mut App) {
    app.view_draft.grouping.clear();
    app.view_draft.filters.clear();
    app.view_draft.excluded_date_field_ids.clear();
    app.view_dirty = true;
}

fn delete_view(app: &mut App) {
    if app.project.views.len() <= 1 {
        return;
    }
    app.project
        .views
        .retain(|view| view.id != app.active_view_id);
    let view = app.project.views[0].clone();
    app.active_view_id = view.id.clone();
    app.project.last_used_view_id = view.id.clone();
    app.view_name_draft = view.name.clone();
    app.view_draft = view;
    app.view_dirty = false;
}

fn unique_view_name(app: &App, base: &str) -> String {
    if !app.project.views.iter().any(|view| view.name == base) {
        return base.to_owned();
    }
    let mut number = 2;
    loop {
        let candidate = format!("{base} {number}");
        if !app.project.views.iter().any(|view| view.name == candidate) {
            return candidate;
        }
        number += 1;
    }
}

fn set_grouping(app: &mut App, index: usize, field_name: &str) {
    if field_name == "Ingen" {
        app.view_draft.grouping.truncate(index);
        app.view_dirty = true;
        return;
    }
    let Some(field_id) = app
        .project
        .fields
        .iter()
        .find(|field| field.name == field_name && field.field_type != FieldType::Image)
        .map(|field| field.id.clone())
    else {
        return;
    };
    let grouping = gridcellar::model::Grouping {
        field_id,
        direction: gridcellar::model::SortDirection::Ascending,
    };
    if index < app.view_draft.grouping.len() {
        app.view_draft.grouping[index] = grouping;
    } else if index == app.view_draft.grouping.len() && index < 3 {
        app.view_draft.grouping.push(grouping);
    }
    app.view_dirty = true;
}

fn toggle_grouping_direction(app: &mut App, index: usize) {
    if let Some(grouping) = app.view_draft.grouping.get_mut(index) {
        grouping.direction = match grouping.direction {
            gridcellar::model::SortDirection::Ascending => {
                gridcellar::model::SortDirection::Descending
            }
            gridcellar::model::SortDirection::Descending => {
                gridcellar::model::SortDirection::Ascending
            }
        };
        app.view_dirty = true;
    }
}

fn add_filter(app: &mut App) {
    let Some(field_name) = app.filter_draft.field_name.as_deref() else {
        return;
    };
    let Some(field) = app
        .project
        .fields
        .iter()
        .find(|field| field.name == field_name && field.field_type != FieldType::Image)
    else {
        return;
    };
    let operator = match app.filter_draft.operator {
        "Innehåller" => gridcellar::model::FilterOperator::Contains,
        "Är exakt" | "Lika med" => gridcellar::model::FilterOperator::Equals,
        "Större än" => gridcellar::model::FilterOperator::GreaterThan,
        "Mindre än" => gridcellar::model::FilterOperator::LessThan,
        "Intervall" => gridcellar::model::FilterOperator::Range,
        "Före" => gridcellar::model::FilterOperator::Before,
        "Efter" => gridcellar::model::FilterOperator::After,
        "Mellan" => gridcellar::model::FilterOperator::Between,
        "Är någon av" => gridcellar::model::FilterOperator::IsAnyOf,
        _ => gridcellar::model::FilterOperator::IsEmpty,
    };
    let operands = if operator == gridcellar::model::FilterOperator::IsEmpty {
        Vec::new()
    } else {
        app.filter_draft
            .operands
            .split(';')
            .filter_map(|value| filter_operand(app, field, value.trim()))
            .collect()
    };
    let filter = gridcellar::model::Filter {
        field_id: field.id.clone(),
        operator,
        operands,
        include_empty: app.filter_draft.include_empty,
    };
    let mut candidate = app.view_draft.clone();
    candidate.filters.push(filter.clone());
    let mut project = app.project.clone();
    if let Some(view) = project
        .views
        .iter_mut()
        .find(|view| view.id == app.active_view_id)
    {
        *view = candidate;
    }
    if gridcellar::validation::validate_project(&project)
        .iter()
        .any(|error| matches!(error, ValidationError::InvalidFilter(_)))
    {
        app.field_status = Some("Filtret är ogiltigt för valt fält eller operator.".to_owned());
        return;
    }
    app.view_draft.filters.push(filter);
    app.view_dirty = true;
    app.filter_draft = FilterDraft::default();
}

fn filter_operand(
    app: &App,
    field: &Field,
    value: &str,
) -> Option<gridcellar::model::FilterOperand> {
    use gridcellar::model::{FilterOperand, NumberKind};
    match &field.field_type {
        FieldType::Text => Some(FilterOperand::Text(value.to_owned())),
        FieldType::Number(NumberKind::Integer) => value.parse().ok().map(FilterOperand::Integer),
        FieldType::Number(NumberKind::Decimal) => value
            .replace(',', ".")
            .parse()
            .ok()
            .map(FilterOperand::Decimal),
        FieldType::Date => Some(FilterOperand::Date(gridcellar::model::CalendarDate::new(value))),
        FieldType::List => app
            .project
            .list_values
            .iter()
            .find(|item| item.field_id == field.id && item.name.eq_ignore_ascii_case(value))
            .map(|item| FilterOperand::ListValue(item.id.clone())),
        FieldType::Image => None,
    }
}

fn duplicate_object(app: &mut App) {
    let object = app
        .selected_object_id
        .as_ref()
        .and_then(|id| app.project.objects.iter().find(|object| object.id == *id))
        .or_else(|| app.project.objects.first());
    if let Some(object) = object {
        let mut draft = ObjectDraft::from_object(&app.project, object);
        draft.object_id = None;
        app.saved_object_draft = Some(draft.clone());
        app.object_draft = Some(draft);
        app.selected_object_id = None;
        app.object_status = None;
    }
}

fn delete_object(app: &mut App) {
    let Some(object_id) = app
        .selected_object_id
        .clone()
        .or_else(|| app.project.objects.first().map(|object| object.id.clone()))
    else {
        return;
    };
    app.project.objects.retain(|object| object.id != object_id);
    app.selected_object_id = None;
    app.object_status = None;
    app.panel = None;
}

fn field_errors(errors: &[ValidationError]) -> String {
    errors
        .iter()
        .map(|error| match error {
            ValidationError::FieldHasValues { object_count, .. } => {
                format!("Fältet har värde på {object_count} objekt")
            }
            ValidationError::FieldIsUsed {
                view_count,
                in_label,
                ..
            } => format!(
                "Fältet används i {}{}",
                if *in_label { "global etikett" } else { "" },
                if *view_count > 0 {
                    format!("{}{view_count} vyer", if *in_label { " och " } else { "" })
                } else {
                    String::new()
                }
            ),
            _ => "Fältåtgärden är spärrad.".to_owned(),
        })
        .collect::<Vec<_>>()
        .join(". ")
}

fn field_type_options() -> [&'static str; 5] {
    ["Text", "Tal", "Datum", "Lista", "Bild"]
}

fn field_type_label(field_type: &FieldType) -> &'static str {
    match field_type {
        FieldType::Text => "Text",
        FieldType::Number(_) => "Tal",
        FieldType::Date => "Datum",
        FieldType::List => "Lista",
        FieldType::Image => "Bild",
    }
}

fn field_type_from_label(label: &str) -> FieldType {
    match label {
        "Tal" => FieldType::Number(gridcellar::model::NumberKind::Decimal),
        "Datum" => FieldType::Date,
        "Lista" => FieldType::List,
        "Bild" => FieldType::Image,
        _ => FieldType::Text,
    }
}

fn value_mode_options() -> [&'static str; 2] {
    ["Ett värde", "Flera värden"]
}

fn value_mode_label(value_mode: ValueMode) -> &'static str {
    match value_mode {
        ValueMode::Single => "Ett värde",
        ValueMode::Multiple => "Flera värden",
    }
}

fn value_mode_from_label(label: &str) -> ValueMode {
    if label == "Flera värden" {
        ValueMode::Multiple
    } else {
        ValueMode::Single
    }
}

fn detail_format_options() -> [&'static str; 7] {
    [
        "Normal rad",
        "Rubrikrad",
        "Chip",
        "Längre text",
        "Bild",
        "Datum",
        "Tal",
    ]
}

fn detail_format_label(format: DetailFormat) -> &'static str {
    match format {
        DetailFormat::NormalRow => "Normal rad",
        DetailFormat::Title => "Rubrikrad",
        DetailFormat::Chips => "Chip",
        DetailFormat::LongText => "Längre text",
        DetailFormat::Image => "Bild",
        DetailFormat::Date => "Datum",
        DetailFormat::Number => "Tal",
    }
}

fn detail_format_from_label(label: &str) -> DetailFormat {
    match label {
        "Rubrikrad" => DetailFormat::Title,
        "Chip" => DetailFormat::Chips,
        "Längre text" => DetailFormat::LongText,
        "Bild" => DetailFormat::Image,
        "Datum" => DetailFormat::Date,
        "Tal" => DetailFormat::Number,
        _ => DetailFormat::NormalRow,
    }
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
