use crate::annotation::Annotation;
use crate::error::AnotError;

pub enum RenderAdapter {
    Json(JsonAdapter),
    Yaml(YamlAdapter),
}

impl RenderAdapter {
    pub fn format(&self, annotations: &[Annotation]) -> Result<String, AnotError> {
        match self {
            RenderAdapter::Json(adapter) => adapter.format(annotations),
            RenderAdapter::Yaml(adapter) => adapter.format(annotations),
        }
    }
}

pub struct JsonAdapter;

impl JsonAdapter {
    pub fn format(&self, annotations: &[Annotation]) -> Result<String, AnotError> {
        serde_json::to_string_pretty(annotations)
            .map_err(|e| AnotError::Serialization(e.to_string()))
    }
}

pub struct YamlAdapter;

impl YamlAdapter {
    pub fn format(&self, annotations: &[Annotation]) -> Result<String, AnotError> {
        serde_yaml::to_string(annotations).map_err(|e| AnotError::Serialization(e.to_string()))
    }
}
