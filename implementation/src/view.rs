use std::cmp::Ordering;

use crate::model::{
    FieldId, FieldType, FieldValue, Filter, FilterOperand, FilterOperator, ListValueId, Object,
    ObjectId, Project, SortDirection, View,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagramDate {
    pub field_id: FieldId,
    pub date: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagramRow {
    pub object_id: ObjectId,
    pub groups: Vec<String>,
    pub dates: Vec<DiagramDate>,
    pub has_visible_dates: bool,
}

pub fn calculate_view(project: &Project, view: &View, search: Option<&str>) -> Vec<DiagramRow> {
    let mut objects: Vec<&Object> = project
        .objects
        .iter()
        .filter(|object| {
            view.filters
                .iter()
                .all(|filter| matches_filter(project, object, filter))
        })
        .filter(|object| matches_search(project, object, search))
        .collect();

    objects.sort_by(|left, right| compare_objects(project, view, left, right));

    objects
        .into_iter()
        .map(|object| {
            let groups = view
                .grouping
                .iter()
                .map(|group| group_label(project, object, &group.field_id))
                .collect();
            let dates: Vec<DiagramDate> = project
                .fields
                .iter()
                .filter(|field| {
                    field.field_type == FieldType::Date
                        && !view.excluded_date_field_ids.contains(&field.id)
                })
                .flat_map(|field| {
                    object
                        .values
                        .get(&field.id)
                        .into_iter()
                        .flatten()
                        .filter_map(|value| match value {
                            FieldValue::Date(date) => Some(DiagramDate {
                                field_id: field.id.clone(),
                                date: date.as_str().to_owned(),
                            }),
                            _ => None,
                        })
                })
                .collect();

            DiagramRow {
                object_id: object.id.clone(),
                groups,
                has_visible_dates: !dates.is_empty(),
                dates,
            }
        })
        .collect()
}

fn matches_search(project: &Project, object: &Object, search: Option<&str>) -> bool {
    let Some(query) = search.map(str::trim).filter(|query| !query.is_empty()) else {
        return true;
    };
    let query = query.to_lowercase();

    object.values.iter().any(|(field_id, values)| {
        let Some(field) = project.fields.iter().find(|field| field.id == *field_id) else {
            return false;
        };
        match field.field_type {
            FieldType::Text => values.iter().any(|value| {
                matches!(value, FieldValue::Text(text) if text.to_lowercase().contains(&query))
            }),
            FieldType::List => values.iter().any(|value| {
                matches!(value, FieldValue::List(id) if project.list_values.iter().any(
                    |item| item.id == *id && item.name.to_lowercase().contains(&query)
                ))
            }),
            _ => false,
        }
    })
}

fn matches_filter(project: &Project, object: &Object, filter: &Filter) -> bool {
    let values = object
        .values
        .get(&filter.field_id)
        .map(Vec::as_slice)
        .unwrap_or_default();
    let empty = values.is_empty();

    if filter.operator == FilterOperator::IsEmpty {
        return empty;
    }
    if empty {
        return filter.include_empty;
    }

    values
        .iter()
        .any(|value| matches_filter_value(project, value, filter))
}

fn matches_filter_value(project: &Project, value: &FieldValue, filter: &Filter) -> bool {
    match (value, filter.operator, filter.operands.as_slice()) {
        (FieldValue::Text(value), FilterOperator::Contains, [FilterOperand::Text(operand)]) => {
            value.to_lowercase().contains(&operand.to_lowercase())
        }
        (FieldValue::Text(value), FilterOperator::Equals, [FilterOperand::Text(operand)]) => {
            value.eq_ignore_ascii_case(operand)
        }
        (FieldValue::Integer(value), operator, [FilterOperand::Integer(operand)]) => {
            compare_operator(value.cmp(operand), operator)
        }
        (FieldValue::Decimal(value), operator, [FilterOperand::Decimal(operand)]) => value
            .partial_cmp(operand)
            .is_some_and(|ordering| compare_operator(ordering, operator)),
        (FieldValue::Date(value), operator, [FilterOperand::Date(operand)]) => {
            compare_operator(value.as_str().cmp(operand.as_str()), operator)
        }
        (
            FieldValue::Integer(value),
            FilterOperator::Range,
            [FilterOperand::Integer(a), FilterOperand::Integer(b)],
        ) => value >= a && value <= b,
        (
            FieldValue::Decimal(value),
            FilterOperator::Range,
            [FilterOperand::Decimal(a), FilterOperand::Decimal(b)],
        ) => value >= a && value <= b,
        (
            FieldValue::Date(value),
            FilterOperator::Between,
            [FilterOperand::Date(a), FilterOperand::Date(b)],
        ) => value.as_str() >= a.as_str() && value.as_str() <= b.as_str(),
        (FieldValue::List(value), FilterOperator::IsAnyOf, operands) => operands
            .iter()
            .any(|operand| matches!(operand, FilterOperand::ListValue(id) if id == value)),
        _ => {
            let _ = project;
            false
        }
    }
}

fn compare_operator(ordering: Ordering, operator: FilterOperator) -> bool {
    match operator {
        FilterOperator::Equals => ordering == Ordering::Equal,
        FilterOperator::GreaterThan | FilterOperator::After => ordering == Ordering::Greater,
        FilterOperator::LessThan | FilterOperator::Before => ordering == Ordering::Less,
        _ => false,
    }
}

fn compare_objects(project: &Project, view: &View, left: &Object, right: &Object) -> Ordering {
    for grouping in &view.grouping {
        let ordering = compare_field_values(project, left, right, &grouping.field_id);
        if ordering != Ordering::Equal {
            return match grouping.direction {
                SortDirection::Ascending => ordering,
                SortDirection::Descending => ordering.reverse(),
            };
        }
    }
    left.id.cmp(&right.id)
}

fn compare_field_values(
    project: &Project,
    left: &Object,
    right: &Object,
    field_id: &FieldId,
) -> Ordering {
    let left = left
        .values
        .get(field_id)
        .map(Vec::as_slice)
        .unwrap_or_default();
    let right = right
        .values
        .get(field_id)
        .map(Vec::as_slice)
        .unwrap_or_default();

    for (left, right) in left.iter().zip(right) {
        let ordering = compare_values(project, left, right);
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    left.len().cmp(&right.len())
}

fn compare_values(project: &Project, left: &FieldValue, right: &FieldValue) -> Ordering {
    match (left, right) {
        (FieldValue::Text(left), FieldValue::Text(right)) => {
            left.to_lowercase().cmp(&right.to_lowercase())
        }
        (FieldValue::Integer(left), FieldValue::Integer(right)) => left.cmp(right),
        (FieldValue::Decimal(left), FieldValue::Decimal(right)) => {
            left.partial_cmp(right).unwrap_or(Ordering::Equal)
        }
        (FieldValue::Date(left), FieldValue::Date(right)) => left.as_str().cmp(right.as_str()),
        (FieldValue::List(left), FieldValue::List(right)) => {
            let order = |id: &ListValueId| {
                project
                    .list_values
                    .iter()
                    .find(|value| value.id == *id)
                    .map(|value| value.order)
                    .unwrap_or(usize::MAX)
            };
            order(left).cmp(&order(right))
        }
        _ => Ordering::Equal,
    }
}

fn group_label(project: &Project, object: &Object, field_id: &FieldId) -> String {
    let Some(field) = project.fields.iter().find(|field| field.id == *field_id) else {
        return "Saknar värde".to_owned();
    };
    let Some(value) = object
        .values
        .get(field_id)
        .and_then(|values| values.first())
    else {
        return "Saknar värde".to_owned();
    };

    match (value, &field.field_type) {
        (FieldValue::Text(value), _) => value.clone(),
        (FieldValue::Integer(value), _) => value.to_string(),
        (FieldValue::Decimal(value), _) => value.to_string(),
        (FieldValue::Date(value), FieldType::Date) => value.as_str()[..4].to_owned(),
        (FieldValue::List(id), FieldType::List) => project
            .list_values
            .iter()
            .find(|item| item.id == *id)
            .map(|item| item.name.clone())
            .unwrap_or_else(|| "Saknar värde".to_owned()),
        _ => "Saknar värde".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::model::{
        CalendarDate, DetailFormat, Field, Grouping, ObjectId, ProjectId, ValueMode, ViewId,
    };

    #[test]
    fn filters_searches_groups_sorts_and_selects_dates() {
        let mut project = Project::empty(ProjectId::new("p"), ViewId::new("v"));
        let name = FieldId::new("name");
        let date = FieldId::new("date");
        project.fields = vec![
            Field {
                id: name.clone(),
                project_id: project.id.clone(),
                name: "Namn".into(),
                field_type: FieldType::Text,
                value_mode: ValueMode::Single,
                required: false,
                detail_format: DetailFormat::Title,
            },
            Field {
                id: date.clone(),
                project_id: project.id.clone(),
                name: "Datum".into(),
                field_type: FieldType::Date,
                value_mode: ValueMode::Single,
                required: false,
                detail_format: DetailFormat::Date,
            },
        ];
        project.objects = vec![
            Object {
                id: ObjectId::new("b"),
                project_id: project.id.clone(),
                values: BTreeMap::from([(name.clone(), vec![FieldValue::Text("Beta".into())])]),
            },
            Object {
                id: ObjectId::new("a"),
                project_id: project.id.clone(),
                values: BTreeMap::from([
                    (name.clone(), vec![FieldValue::Text("Alfa".into())]),
                    (
                        date.clone(),
                        vec![FieldValue::Date(CalendarDate::new("2026-01-01"))],
                    ),
                ]),
            },
        ];
        let mut view = project.views[0].clone();
        view.grouping = vec![Grouping {
            field_id: name.clone(),
            direction: SortDirection::Ascending,
        }];

        let rows = calculate_view(&project, &view, Some("a"));

        assert_eq!(
            rows.iter()
                .map(|row| row.object_id.as_str())
                .collect::<Vec<_>>(),
            vec!["a", "b"]
        );
        assert!(rows[0].has_visible_dates);
        assert!(!rows[1].has_visible_dates);
        assert_eq!(rows[0].groups, vec!["Alfa"]);
    }
}
