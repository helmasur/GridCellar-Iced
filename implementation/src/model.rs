use std::collections::BTreeMap;

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

id_type!(ProjectId);
id_type!(FieldId);
id_type!(ListValueId);
id_type!(ObjectId);
id_type!(ViewId);

#[derive(Clone, Debug, PartialEq)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub fields: Vec<Field>,
    pub list_values: Vec<ListValue>,
    pub objects: Vec<Object>,
    pub views: Vec<View>,
    pub diagram_label_field_ids: Vec<FieldId>,
    pub diagram_settings: DiagramSettings,
    pub last_used_view_id: ViewId,
}

impl Project {
    pub fn empty(id: ProjectId, view_id: ViewId) -> Self {
        Self {
            id: id.clone(),
            name: "Min källare".to_owned(),
            fields: Vec::new(),
            list_values: Vec::new(),
            objects: Vec::new(),
            views: vec![View::all_objects(view_id.clone(), id)],
            diagram_label_field_ids: Vec::new(),
            diagram_settings: DiagramSettings::default(),
            last_used_view_id: view_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiagramSettings {
    pub row_height: u16,
    pub group_row_height: u16,
    pub name_column_width: u16,
    pub time_range: TimeRange,
}

impl Default for DiagramSettings {
    fn default() -> Self {
        Self {
            row_height: 40,
            group_row_height: 28,
            name_column_width: 240,
            time_range: TimeRange::ShowAll,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimeRange {
    ShowAll,
    FiveYears,
    TenYears,
    Custom {
        start: CalendarDate,
        end: CalendarDate,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field {
    pub id: FieldId,
    pub project_id: ProjectId,
    pub name: String,
    pub field_type: FieldType,
    pub value_mode: ValueMode,
    pub required: bool,
    pub detail_format: DetailFormat,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldType {
    Text,
    Number(NumberKind),
    Date,
    List,
    Image,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumberKind {
    Integer,
    Decimal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueMode {
    Single,
    Multiple,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DetailFormat {
    NormalRow,
    Title,
    Chips,
    LongText,
    Image,
    Date,
    Number,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListValue {
    pub id: ListValueId,
    pub field_id: FieldId,
    pub name: String,
    pub order: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    pub id: ObjectId,
    pub project_id: ProjectId,
    pub values: BTreeMap<FieldId, Vec<FieldValue>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    Text(String),
    Integer(i64),
    Decimal(f64),
    Date(CalendarDate),
    List(ListValueId),
    Image(ImageData),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarDate(String);

impl CalendarDate {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageData {
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct View {
    pub id: ViewId,
    pub project_id: ProjectId,
    pub name: String,
    pub grouping: Vec<Grouping>,
    pub filters: Vec<Filter>,
    pub excluded_date_field_ids: Vec<FieldId>,
}

impl View {
    fn all_objects(id: ViewId, project_id: ProjectId) -> Self {
        Self {
            id,
            project_id,
            name: "Alla objekt".to_owned(),
            grouping: Vec::new(),
            filters: Vec::new(),
            excluded_date_field_ids: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grouping {
    pub field_id: FieldId,
    pub direction: SortDirection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Filter {
    pub field_id: FieldId,
    pub operator: FilterOperator,
    pub operands: Vec<FilterOperand>,
    pub include_empty: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FilterOperator {
    Contains,
    Equals,
    GreaterThan,
    LessThan,
    Range,
    Before,
    After,
    Between,
    IsAnyOf,
    IsEmpty,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FilterOperand {
    Text(String),
    Integer(i64),
    Decimal(f64),
    Date(CalendarDate),
    ListValue(ListValueId),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_empty_standard_project() {
        let project = Project::empty(ProjectId::new("project-01"), ViewId::new("view-all"));

        assert_eq!(project.name, "Min källare");
        assert!(project.fields.is_empty());
        assert!(project.objects.is_empty());
        assert_eq!(project.views.len(), 1);
        assert_eq!(project.views[0].name, "Alla objekt");
        assert_eq!(project.last_used_view_id.as_str(), "view-all");
    }

    #[test]
    fn represents_contract_entities_and_ordered_values() {
        let project_id = ProjectId::new("project-01");
        let name_field_id = FieldId::new("field-name");
        let origin_field_id = FieldId::new("field-origin");
        let france_id = ListValueId::new("origin-france");
        let italy_id = ListValueId::new("origin-italy");

        let mut values = BTreeMap::new();
        values.insert(
            name_field_id.clone(),
            vec![FieldValue::Text("Blandning B".to_owned())],
        );
        values.insert(
            origin_field_id.clone(),
            vec![
                FieldValue::List(italy_id.clone()),
                FieldValue::List(france_id.clone()),
            ],
        );

        let mut project = Project::empty(project_id.clone(), ViewId::new("view-all"));
        project.fields = vec![
            Field {
                id: name_field_id,
                project_id: project_id.clone(),
                name: "Namn".to_owned(),
                field_type: FieldType::Text,
                value_mode: ValueMode::Single,
                required: false,
                detail_format: DetailFormat::Title,
            },
            Field {
                id: origin_field_id.clone(),
                project_id: project_id.clone(),
                name: "Ursprung".to_owned(),
                field_type: FieldType::List,
                value_mode: ValueMode::Multiple,
                required: false,
                detail_format: DetailFormat::Chips,
            },
        ];
        project.list_values = vec![
            ListValue {
                id: france_id,
                field_id: origin_field_id.clone(),
                name: "Frankrike".to_owned(),
                order: 1,
            },
            ListValue {
                id: italy_id,
                field_id: origin_field_id.clone(),
                name: "Italien".to_owned(),
                order: 2,
            },
        ];
        project.objects.push(Object {
            id: ObjectId::new("object-02"),
            project_id,
            values,
        });

        let origins = &project.objects[0].values[&origin_field_id];
        assert_eq!(origins.len(), 2);
        assert_eq!(
            origins[0],
            FieldValue::List(ListValueId::new("origin-italy"))
        );
    }
}
