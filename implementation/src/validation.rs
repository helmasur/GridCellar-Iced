use crate::model::{
    Field, FieldId, FieldType, FieldValue, Filter, FilterOperand, FilterOperator, ListValueId,
    Object, Project, ValueMode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationError {
    DuplicateFieldName(String),
    DuplicateListValueName(String),
    DuplicateViewName(String),
    MissingRequiredValue(FieldId),
    InvalidValueType(FieldId),
    TooManyValues(FieldId),
    MultipleImageValues(FieldId),
    InvalidDiagramLabel,
    UnknownField(FieldId),
    InvalidFilter(FieldId),
    FieldHasValues {
        field_id: FieldId,
        object_count: usize,
    },
    FieldIsUsed {
        field_id: FieldId,
        view_count: usize,
        in_label: bool,
    },
    ListValueIsUsed {
        list_value_id: ListValueId,
        object_count: usize,
        filter_count: usize,
    },
}

pub fn normalize_text(value: &str) -> Option<String> {
    let normalized = value.trim();
    (!normalized.is_empty()).then(|| normalized.to_owned())
}

pub fn validate_project(project: &Project) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    push_duplicate_names(
        project.fields.iter().map(|field| field.name.as_str()),
        |name| ValidationError::DuplicateFieldName(name.to_owned()),
        &mut errors,
    );
    push_duplicate_names(
        project.views.iter().map(|view| view.name.as_str()),
        |name| ValidationError::DuplicateViewName(name.to_owned()),
        &mut errors,
    );

    for field in &project.fields {
        push_duplicate_names(
            project
                .list_values
                .iter()
                .filter(|value| value.field_id == field.id)
                .map(|value| value.name.as_str()),
            |name| ValidationError::DuplicateListValueName(name.to_owned()),
            &mut errors,
        );
    }

    if (project.fields.is_empty() && !project.diagram_label_field_ids.is_empty())
        || (!project.fields.is_empty()
            && (project.diagram_label_field_ids.is_empty()
                || project.diagram_label_field_ids.len() > 5
                || project
                    .diagram_label_field_ids
                    .iter()
                    .any(|id| !project.fields.iter().any(|field| field.id == *id))))
    {
        errors.push(ValidationError::InvalidDiagramLabel);
    }

    for object in &project.objects {
        errors.extend(validate_object(project, object));
    }

    for view in &project.views {
        if view.grouping.len() > 3 {
            errors.push(ValidationError::InvalidFilter(
                view.grouping[3].field_id.clone(),
            ));
        }

        for grouping in &view.grouping {
            match project
                .fields
                .iter()
                .find(|field| field.id == grouping.field_id)
            {
                Some(field) if field.field_type != FieldType::Image => {}
                _ => errors.push(ValidationError::UnknownField(grouping.field_id.clone())),
            }
        }

        for filter in &view.filters {
            if !is_valid_filter(project, filter) {
                errors.push(ValidationError::InvalidFilter(filter.field_id.clone()));
            }
        }
    }

    errors
}

pub fn validate_object(project: &Project, object: &Object) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    for field in &project.fields {
        let values = object
            .values
            .get(&field.id)
            .map(Vec::as_slice)
            .unwrap_or_default();

        if field.required && values.is_empty() {
            errors.push(ValidationError::MissingRequiredValue(field.id.clone()));
        }
        if field.value_mode == ValueMode::Single && values.len() > 1 {
            errors.push(ValidationError::TooManyValues(field.id.clone()));
        }
        if field.field_type == FieldType::Image && values.len() > 1 {
            errors.push(ValidationError::MultipleImageValues(field.id.clone()));
        }
        if values
            .iter()
            .any(|value| !value_matches_field(value, field))
        {
            errors.push(ValidationError::InvalidValueType(field.id.clone()));
        }
    }

    for field_id in object.values.keys() {
        if !project.fields.iter().any(|field| field.id == *field_id) {
            errors.push(ValidationError::UnknownField(field_id.clone()));
        }
    }

    errors
}

pub fn can_remove_or_change_field(
    project: &Project,
    field_id: &FieldId,
) -> Result<(), Vec<ValidationError>> {
    let object_count = project
        .objects
        .iter()
        .filter(|object| {
            object
                .values
                .get(field_id)
                .is_some_and(|values| !values.is_empty())
        })
        .count();
    let view_count = project
        .views
        .iter()
        .filter(|view| {
            view.grouping
                .iter()
                .any(|group| group.field_id == *field_id)
                || view
                    .filters
                    .iter()
                    .any(|filter| filter.field_id == *field_id)
                || view.excluded_date_field_ids.contains(field_id)
        })
        .count();
    let in_label = project.diagram_label_field_ids.contains(field_id);

    let mut errors = Vec::new();
    if object_count > 0 {
        errors.push(ValidationError::FieldHasValues {
            field_id: field_id.clone(),
            object_count,
        });
    }
    if view_count > 0 || in_label {
        errors.push(ValidationError::FieldIsUsed {
            field_id: field_id.clone(),
            view_count,
            in_label,
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn can_remove_list_value(
    project: &Project,
    list_value_id: &ListValueId,
) -> Result<(), ValidationError> {
    let object_count = project
        .objects
        .iter()
        .filter(|object| {
            object
                .values
                .values()
                .flatten()
                .any(|value| matches!(value, FieldValue::List(id) if id == list_value_id))
        })
        .count();
    let filter_count = project
        .views
        .iter()
        .flat_map(|view| &view.filters)
        .filter(|filter| {
            filter.operands.iter().any(|operand| {
                matches!(
                    operand,
                    FilterOperand::ListValue(id) if id == list_value_id
                )
            })
        })
        .count();

    if object_count == 0 && filter_count == 0 {
        Ok(())
    } else {
        Err(ValidationError::ListValueIsUsed {
            list_value_id: list_value_id.clone(),
            object_count,
            filter_count,
        })
    }
}

fn push_duplicate_names<'a>(
    names: impl Iterator<Item = &'a str>,
    error: impl Fn(&str) -> ValidationError,
    errors: &mut Vec<ValidationError>,
) {
    let mut seen = Vec::new();
    for name in names {
        let normalized = name.trim().to_lowercase();
        if seen.contains(&normalized) {
            errors.push(error(name));
        } else {
            seen.push(normalized);
        }
    }
}

fn value_matches_field(value: &FieldValue, field: &Field) -> bool {
    matches!(
        (value, &field.field_type),
        (FieldValue::Text(text), FieldType::Text) if normalize_text(text).is_some()
    ) || matches!(
        (value, &field.field_type),
        (
            FieldValue::Integer(_),
            FieldType::Number(crate::model::NumberKind::Integer)
        ) | (
            FieldValue::Decimal(_),
            FieldType::Number(crate::model::NumberKind::Decimal)
        ) | (FieldValue::Date(_), FieldType::Date)
            | (FieldValue::List(_), FieldType::List)
            | (FieldValue::Image(_), FieldType::Image)
    )
}

fn is_valid_filter(project: &Project, filter: &Filter) -> bool {
    let Some(field) = project
        .fields
        .iter()
        .find(|field| field.id == filter.field_id)
    else {
        return false;
    };

    if field.field_type == FieldType::Image {
        return false;
    }

    if filter.operator == FilterOperator::IsEmpty {
        return filter.operands.is_empty();
    }

    match (&field.field_type, filter.operator) {
        (FieldType::Text, FilterOperator::Contains | FilterOperator::Equals) => {
            one_operand(filter, |value| matches!(value, FilterOperand::Text(_)))
        }
        (
            FieldType::Number(crate::model::NumberKind::Integer),
            FilterOperator::Equals | FilterOperator::GreaterThan | FilterOperator::LessThan,
        ) => one_operand(filter, |value| matches!(value, FilterOperand::Integer(_))),
        (
            FieldType::Number(crate::model::NumberKind::Decimal),
            FilterOperator::Equals | FilterOperator::GreaterThan | FilterOperator::LessThan,
        ) => one_operand(filter, |value| matches!(value, FilterOperand::Decimal(_))),
        (FieldType::Number(_), FilterOperator::Range) => two_numeric_operands(filter),
        (FieldType::Date, FilterOperator::Before | FilterOperator::After) => {
            one_operand(filter, |value| matches!(value, FilterOperand::Date(_)))
        }
        (FieldType::Date, FilterOperator::Between) => {
            filter.operands.len() == 2
                && filter
                    .operands
                    .iter()
                    .all(|value| matches!(value, FilterOperand::Date(_)))
        }
        (FieldType::List, FilterOperator::IsAnyOf) => {
            !filter.operands.is_empty()
                && filter.operands.iter().all(|value| {
                    matches!(value, FilterOperand::ListValue(id) if project
                        .list_values
                        .iter()
                        .any(|item| item.field_id == field.id && item.id == *id))
                })
        }
        _ => false,
    }
}

fn one_operand(filter: &Filter, predicate: impl Fn(&FilterOperand) -> bool) -> bool {
    filter.operands.len() == 1 && predicate(&filter.operands[0])
}

fn two_numeric_operands(filter: &Filter) -> bool {
    filter.operands.len() == 2
        && (filter
            .operands
            .iter()
            .all(|value| matches!(value, FilterOperand::Integer(_)))
            || filter
                .operands
                .iter()
                .all(|value| matches!(value, FilterOperand::Decimal(_))))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::model::{
        DetailFormat, FieldType, ListValue, NumberKind, ObjectId, ProjectId, ViewId,
    };

    fn text_field(project_id: &ProjectId, id: &str, name: &str) -> Field {
        Field {
            id: FieldId::new(id),
            project_id: project_id.clone(),
            name: name.to_owned(),
            field_type: FieldType::Text,
            value_mode: ValueMode::Single,
            required: false,
            detail_format: DetailFormat::NormalRow,
        }
    }

    fn project() -> Project {
        Project::empty(ProjectId::new("project-01"), ViewId::new("view-all"))
    }

    #[test]
    fn normalizes_empty_and_trimmed_text() {
        assert_eq!(normalize_text("   "), None);
        assert_eq!(
            normalize_text("  interna   mellanslag  "),
            Some("interna   mellanslag".to_owned())
        );
    }

    #[test]
    fn rejects_duplicate_names_and_invalid_label() {
        let mut project = project();
        project.fields = vec![
            text_field(&project.id, "field-a", "Namn"),
            text_field(&project.id, "field-b", " namn "),
        ];

        let errors = validate_project(&project);

        assert!(
            errors
                .iter()
                .any(|error| matches!(error, ValidationError::DuplicateFieldName(_)))
        );
        assert!(errors.contains(&ValidationError::InvalidDiagramLabel));
    }

    #[test]
    fn validates_required_and_typed_object_values() {
        let mut project = project();
        let mut required = text_field(&project.id, "field-name", "Namn");
        required.required = true;
        let number = Field {
            id: FieldId::new("field-count"),
            project_id: project.id.clone(),
            name: "Antal".to_owned(),
            field_type: FieldType::Number(NumberKind::Integer),
            value_mode: ValueMode::Single,
            required: false,
            detail_format: DetailFormat::Number,
        };
        project.fields = vec![required, number.clone()];
        project.diagram_label_field_ids = vec![FieldId::new("field-name")];

        let mut values = BTreeMap::new();
        values.insert(
            number.id.clone(),
            vec![FieldValue::Text("fel typ".to_owned())],
        );
        let object = Object {
            id: ObjectId::new("object-01"),
            project_id: project.id.clone(),
            values,
        };

        let errors = validate_object(&project, &object);

        assert!(
            errors.contains(&ValidationError::MissingRequiredValue(FieldId::new(
                "field-name"
            )))
        );
        assert!(
            errors.contains(&ValidationError::InvalidValueType(FieldId::new(
                "field-count"
            )))
        );
    }

    #[test]
    fn blocks_field_and_list_value_dependencies() {
        let mut project = project();
        let field = text_field(&project.id, "field-name", "Namn");
        project.fields.push(field.clone());
        project.diagram_label_field_ids.push(field.id.clone());
        project.objects.push(Object {
            id: ObjectId::new("object-01"),
            project_id: project.id.clone(),
            values: BTreeMap::from([(
                field.id.clone(),
                vec![FieldValue::Text("Objekt".to_owned())],
            )]),
        });

        let errors = can_remove_or_change_field(&project, &field.id)
            .expect_err("used field must be blocked");
        assert_eq!(errors.len(), 2);

        let list_field_id = FieldId::new("field-origin");
        let list_value_id = ListValueId::new("origin-france");
        project.list_values.push(ListValue {
            id: list_value_id.clone(),
            field_id: list_field_id.clone(),
            name: "Frankrike".to_owned(),
            order: 0,
        });
        project.objects[0]
            .values
            .insert(list_field_id, vec![FieldValue::List(list_value_id.clone())]);

        assert!(can_remove_list_value(&project, &list_value_id).is_err());
    }

    #[test]
    fn identifies_valid_and_invalid_filters() {
        let mut project = project();
        let number = Field {
            id: FieldId::new("field-count"),
            project_id: project.id.clone(),
            name: "Antal".to_owned(),
            field_type: FieldType::Number(NumberKind::Integer),
            value_mode: ValueMode::Single,
            required: false,
            detail_format: DetailFormat::Number,
        };
        project.fields.push(number.clone());
        project.diagram_label_field_ids.push(number.id.clone());

        let valid = Filter {
            field_id: number.id.clone(),
            operator: FilterOperator::GreaterThan,
            operands: vec![FilterOperand::Integer(0)],
            include_empty: false,
        };
        let invalid = Filter {
            field_id: number.id.clone(),
            operator: FilterOperator::Contains,
            operands: vec![FilterOperand::Text("1".to_owned())],
            include_empty: false,
        };

        assert!(is_valid_filter(&project, &valid));
        assert!(!is_valid_filter(&project, &invalid));
    }
}
