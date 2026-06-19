use crate::model::{FieldValue, Object, Project};

const SEPARATOR: &str = " – ";

pub fn diagram_label(project: &Project, object: &Object) -> String {
    let parts: Vec<String> = project
        .diagram_label_field_ids
        .iter()
        .filter_map(|field_id| {
            let values = object.values.get(field_id)?;
            let formatted: Vec<String> = values
                .iter()
                .filter_map(|value| format_value(project, value))
                .filter(|value| !value.is_empty())
                .collect();
            (!formatted.is_empty()).then(|| formatted.join(", "))
        })
        .collect();

    if parts.is_empty() {
        object.id.as_str().to_owned()
    } else {
        parts.join(SEPARATOR)
    }
}

fn format_value(project: &Project, value: &FieldValue) -> Option<String> {
    match value {
        FieldValue::Text(value) => {
            let value = value.trim();
            (!value.is_empty()).then(|| value.to_owned())
        }
        FieldValue::Integer(value) => Some(value.to_string()),
        FieldValue::Decimal(value) => Some(value.to_string()),
        FieldValue::Date(value) => Some(value.as_str().to_owned()),
        FieldValue::List(id) => project
            .list_values
            .iter()
            .find(|item| item.id == *id)
            .map(|item| item.name.clone()),
        FieldValue::Image(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::model::{FieldId, FieldValue, ObjectId, ProjectId, ViewId};

    #[test]
    fn skips_empty_parts_and_uses_fixed_separator() {
        let mut project = Project::empty(ProjectId::new("project"), ViewId::new("view"));
        let name = FieldId::new("name");
        let origin = FieldId::new("origin");
        project.diagram_label_field_ids = vec![name.clone(), origin.clone()];
        let object = Object {
            id: ObjectId::new("object-01"),
            project_id: project.id.clone(),
            values: BTreeMap::from([
                (name, vec![FieldValue::Text("  Objekt  ".to_owned())]),
                (origin, vec![FieldValue::Text(" ".to_owned())]),
            ]),
        };

        assert_eq!(diagram_label(&project, &object), "Objekt");
    }

    #[test]
    fn falls_back_to_object_id() {
        let project = Project::empty(ProjectId::new("project"), ViewId::new("view"));
        let object = Object {
            id: ObjectId::new("object-01"),
            project_id: project.id.clone(),
            values: BTreeMap::new(),
        };

        assert_eq!(diagram_label(&project, &object), "object-01");
    }
}
