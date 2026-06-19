use std::collections::BTreeMap;

use crate::label::diagram_label;
use crate::model::{FieldId, FieldValue, ObjectId, Project, View};
use crate::view::calculate_view;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChartRow {
    Group { level: usize, label: String },
    Object(ChartObjectRow),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChartObjectRow {
    pub object_id: ObjectId,
    pub label: String,
    pub dates: Vec<ChartPoint>,
    pub line_start: Option<String>,
    pub line_end: Option<String>,
    pub missing_visible_dates: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChartPoint {
    pub date: String,
    pub fields: Vec<FieldId>,
}

pub fn chart_rows(project: &Project, view: &View, search: Option<&str>) -> Vec<ChartRow> {
    let rows = calculate_view(project, view, search);
    let mut result = Vec::new();
    let mut previous_groups: Vec<String> = Vec::new();

    for row in rows {
        for (level, group) in row.groups.iter().enumerate() {
            if previous_groups.get(level) != Some(group) {
                result.push(ChartRow::Group {
                    level,
                    label: group.clone(),
                });
                previous_groups.truncate(level);
                previous_groups.push(group.clone());
            }
        }

        let object = project
            .objects
            .iter()
            .find(|object| object.id == row.object_id)
            .expect("view rows must reference project objects");
        let mut points: BTreeMap<String, Vec<FieldId>> = BTreeMap::new();
        for field in &project.fields {
            if view.excluded_date_field_ids.contains(&field.id) {
                continue;
            }
            for value in object.values.get(&field.id).into_iter().flatten() {
                if let FieldValue::Date(date) = value {
                    points
                        .entry(date.as_str().to_owned())
                        .or_default()
                        .push(field.id.clone());
                }
            }
        }
        let dates: Vec<ChartPoint> = points
            .into_iter()
            .map(|(date, fields)| ChartPoint { date, fields })
            .collect();
        result.push(ChartRow::Object(ChartObjectRow {
            object_id: object.id.clone(),
            label: diagram_label(project, object),
            line_start: dates.first().map(|point| point.date.clone()),
            line_end: dates.last().map(|point| point.date.clone()),
            missing_visible_dates: dates.is_empty(),
            dates,
        }));
    }
    result
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::model::{
        CalendarDate, DetailFormat, Field, FieldType, FieldValue, Object, ProjectId, ValueMode,
        ViewId,
    };

    #[test]
    fn builds_group_and_object_rows_with_merged_points() {
        let mut project = Project::empty(ProjectId::new("p"), ViewId::new("v"));
        let name = FieldId::new("name");
        let date_a = FieldId::new("date-a");
        let date_b = FieldId::new("date-b");
        project.fields = vec![
            Field { id: name.clone(), project_id: project.id.clone(), name: "Namn".into(), field_type: FieldType::Text, value_mode: ValueMode::Single, required: false, detail_format: DetailFormat::Title },
            Field { id: date_a.clone(), project_id: project.id.clone(), name: "A".into(), field_type: FieldType::Date, value_mode: ValueMode::Single, required: false, detail_format: DetailFormat::Date },
            Field { id: date_b.clone(), project_id: project.id.clone(), name: "B".into(), field_type: FieldType::Date, value_mode: ValueMode::Single, required: false, detail_format: DetailFormat::Date },
        ];
        project.diagram_label_field_ids = vec![name.clone()];
        project.objects.push(Object {
            id: ObjectId::new("o"),
            project_id: project.id.clone(),
            values: BTreeMap::from([
                (name, vec![FieldValue::Text("Objekt".into())]),
                (date_a, vec![FieldValue::Date(CalendarDate::new("2026-01-01"))]),
                (date_b, vec![FieldValue::Date(CalendarDate::new("2026-01-01"))]),
            ]),
        });
        let rows = chart_rows(&project, &project.views[0], None);
        let ChartRow::Object(row) = &rows[0] else { panic!("object row expected") };
        assert_eq!(row.dates.len(), 1);
        assert_eq!(row.dates[0].fields.len(), 2);
        assert!(!row.missing_visible_dates);
    }
}
