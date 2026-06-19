use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::model::Project;

#[derive(Debug)]
pub enum PersistenceError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl From<io::Error> for PersistenceError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for PersistenceError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

pub fn save_project(path: &Path, project: &Project) -> Result<(), PersistenceError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let temporary_path = temporary_path(path);
    let document = serde_json::to_vec_pretty(project)?;
    fs::write(&temporary_path, document)?;
    fs::rename(&temporary_path, path)?;
    Ok(())
}

pub fn load_project(path: &Path) -> Result<Option<Project>, PersistenceError> {
    match fs::read(path) {
        Ok(document) => Ok(Some(serde_json::from_slice(&document)?)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.into()),
    }
}

fn temporary_path(path: &Path) -> PathBuf {
    let mut temporary = path.as_os_str().to_owned();
    temporary.push(".tmp");
    PathBuf::from(temporary)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use crate::model::{
        DetailFormat, Field, FieldId, FieldType, FieldValue, Object, ObjectId, ProjectId,
        ValueMode, ViewId,
    };

    fn test_path(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("gridcellar-{name}-{nonce}.json"))
    }

    #[test]
    fn absent_project_returns_none() {
        let path = test_path("absent");
        assert_eq!(load_project(&path).expect("load must succeed"), None);
    }

    #[test]
    fn saves_and_loads_empty_standard_project() {
        let path = test_path("empty");
        let project = Project::empty(ProjectId::new("project-01"), ViewId::new("view-all"));

        save_project(&path, &project).expect("save must succeed");
        let loaded = load_project(&path)
            .expect("load must succeed")
            .expect("project must exist");

        assert_eq!(loaded, project);
        fs::remove_file(path).expect("test file must be removable");
    }

    #[test]
    fn preserves_changed_fields_objects_views_and_settings() {
        let path = test_path("changed");
        let mut project = Project::empty(ProjectId::new("project-01"), ViewId::new("view-all"));
        let field_id = FieldId::new("field-name");
        project.name = "Provkällaren".to_owned();
        project.diagram_settings.row_height = 52;
        project.fields.push(Field {
            id: field_id.clone(),
            project_id: project.id.clone(),
            name: "Namn".to_owned(),
            field_type: FieldType::Text,
            value_mode: ValueMode::Single,
            required: true,
            detail_format: DetailFormat::Title,
        });
        project.diagram_label_field_ids.push(field_id.clone());
        project.objects.push(Object {
            id: ObjectId::new("object-01"),
            project_id: project.id.clone(),
            values: BTreeMap::from([(field_id, vec![FieldValue::Text("Källaröl".to_owned())])]),
        });
        project.views[0].name = "Min vy".to_owned();

        save_project(&path, &project).expect("save must succeed");
        let loaded = load_project(&path)
            .expect("load must succeed")
            .expect("project must exist");

        assert_eq!(loaded, project);
        fs::remove_file(path).expect("test file must be removable");
    }
}
