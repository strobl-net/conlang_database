use crate::models::{DevelopmentLevel, PhysicalMode, PurposeSub, VocabularySource};
use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PersonQuery {
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScriptQuery {
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GroupQuery {
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConlangQuery {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub native_name: Option<String>,
    pub registry_code: Option<String>,
    pub creators: Option<Vec<i32>>,
    pub links: Option<Vec<String>>,
    pub start_year: Option<NaiveDateTime>,
    pub physical_mode: Option<PhysicalMode>,
    pub scripts: Option<Vec<i32>>,
    pub groups: Option<Vec<i32>>,
    pub purpose: Option<PurposeSub>,
    pub vocabulary_source: Option<VocabularySource>,
    pub development: Option<DevelopmentLevel>,
    pub notes: Option<String>,
}
