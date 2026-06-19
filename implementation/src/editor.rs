use std::collections::BTreeMap;

use crate::model::{
    CalendarDate, Field, FieldId, FieldType, FieldValue, ImageData, ListValueId, NumberKind,
    Object, ObjectId, Project,
};
use crate::validation::{ValidationError, normalize_text, validate_object};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectDraft {
    pub object_id: Option<ObjectId>,
    pub values: BTreeMap<FieldId, Vec<DraftValue>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DraftValue {
    Text(String),
    Image(Vec<u8>),
}

impl ObjectDraft {
    pub fn empty(project: &Project) -> Self {
        Self {
            object_id: None,
            values: project
                .fields
                .iter()
                .map(|field| (field.id.clone(), Vec::new()))
                .collect(),
        }
    }

    pub fn from_object(project: &Project, object: &Object) -> Self {
        let values = project
            .fields
            .iter()
            .map(|field| {
                let values = object
                    .values
                    .get(&field.id)
                    .into_iter()
                    .flatten()
                    .map(draft_value)
                    .collect();
                (field.id.clone(), values)
            })
            .collect();
        Self {
            object_id: Some(object.id.clone()),
            values,
        }
    }

    pub fn to_object(
        &self,
        project: &Project,
        object_id: ObjectId,
    ) -> Result<Object, Vec<ValidationError>> {
        let mut values = BTreeMap::new();
        let mut conversion_errors = Vec::new();
        for field in &project.fields {
            let mut converted = Vec::new();
            for value in self.values.get(&field.id).into_iter().flatten() {
                match convert_value(field, value) {
                    Some(value) => converted.push(value),
                    None if draft_value_is_empty(value) => {}
                    None => {
                        conversion_errors.push(ValidationError::InvalidValueType(field.id.clone()))
                    }
                }
            }
            values.insert(field.id.clone(), converted);
        }
        let object = Object {
            id: object_id,
            project_id: project.id.clone(),
            values,
        };
        let mut errors = conversion_errors;
        errors.extend(validate_object(project, &object));
        if errors.is_empty() {
            Ok(object)
        } else {
            Err(errors)
        }
    }
}

fn draft_value_is_empty(value: &DraftValue) -> bool {
    match value {
        DraftValue::Text(value) => value.trim().is_empty(),
        DraftValue::Image(bytes) => bytes.is_empty(),
    }
}

fn draft_value(value: &FieldValue) -> DraftValue {
    match value {
        FieldValue::Text(value) => DraftValue::Text(value.clone()),
        FieldValue::Integer(value) => DraftValue::Text(value.to_string()),
        FieldValue::Decimal(value) => DraftValue::Text(value.to_string()),
        FieldValue::Date(value) => DraftValue::Text(value.as_str().to_owned()),
        FieldValue::List(id) => DraftValue::Text(id.as_str().to_owned()),
        FieldValue::Image(image) => DraftValue::Image(image.bytes.clone()),
    }
}

fn convert_value(field: &Field, value: &DraftValue) -> Option<FieldValue> {
    match (&field.field_type, value) {
        (FieldType::Text, DraftValue::Text(value)) => normalize_text(value).map(FieldValue::Text),
        (FieldType::Number(NumberKind::Integer), DraftValue::Text(value)) => {
            value.trim().parse().ok().map(FieldValue::Integer)
        }
        (FieldType::Number(NumberKind::Decimal), DraftValue::Text(value)) => value
            .trim()
            .replace(',', ".")
            .parse()
            .ok()
            .map(FieldValue::Decimal),
        (FieldType::Date, DraftValue::Text(value)) => normalize_text(value)
            .filter(|value| valid_date_shape(value))
            .map(|value| FieldValue::Date(CalendarDate::new(value))),
        (FieldType::List, DraftValue::Text(value)) => {
            normalize_text(value).map(|value| FieldValue::List(ListValueId::new(value)))
        }
        (FieldType::Image, DraftValue::Image(bytes)) if !bytes.is_empty() => {
            Some(FieldValue::Image(ImageData {
                bytes: bytes.clone(),
            }))
        }
        _ => None,
    }
}

fn valid_date_shape(value: &str) -> bool {
    value.len() == 10
        && value.as_bytes()[4] == b'-'
        && value.as_bytes()[7] == b'-'
        && value
            .chars()
            .enumerate()
            .all(|(index, character)| index == 4 || index == 7 || character.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DetailFormat, Field, ProjectId, ValueMode, ViewId};

    #[test]
    fn normalizes_and_converts_draft_values() {
        let mut project = Project::empty(ProjectId::new("project"), ViewId::new("view"));
        let text_id = FieldId::new("text");
        let number_id = FieldId::new("number");
        project.fields = vec![
            Field {
                id: text_id.clone(),
                project_id: project.id.clone(),
                name: "Text".into(),
                field_type: FieldType::Text,
                value_mode: ValueMode::Single,
                required: true,
                detail_format: DetailFormat::NormalRow,
            },
            Field {
                id: number_id.clone(),
                project_id: project.id.clone(),
                name: "Tal".into(),
                field_type: FieldType::Number(NumberKind::Decimal),
                value_mode: ValueMode::Single,
                required: false,
                detail_format: DetailFormat::Number,
            },
        ];
        let draft = ObjectDraft {
            object_id: None,
            values: BTreeMap::from([
                (text_id.clone(), vec![DraftValue::Text("  värde  ".into())]),
                (number_id.clone(), vec![DraftValue::Text("1,5".into())]),
            ]),
        };

        let object = draft
            .to_object(&project, ObjectId::new("object"))
            .expect("draft must be valid");

        assert_eq!(
            object.values[&text_id],
            vec![FieldValue::Text("värde".into())]
        );
        assert_eq!(object.values[&number_id], vec![FieldValue::Decimal(1.5)]);
    }
}
