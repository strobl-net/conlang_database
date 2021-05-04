use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(sqlx::Type)]
#[sqlx(type_name = "purpose")]
#[sqlx(rename_all = "lowercase")]
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Purpose {
    Artistic,
    Personal,
    Artlang,
    Others,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "purpose_sub")]
#[sqlx(rename_all = "lowercase")]
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PurposeSub {
    Personal,
    Jokelang,
    StoryBased,
    Conworld,
    Geofictional,
    Future,
    AlternateHistory,
    Lostlang,
    Xenolang,
    PseudoAuxlang,
    GlobalAuxlang,
    ZonalAuxlang,
    OtherAuxlang,
    Ideal,
    Philosophical,
    Logical,
    Experimental,
    Conpidgin,
    Secret,
    Other,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "physical_mode")]
#[sqlx(rename_all = "lowercase")]
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PhysicalMode {
    SpeechWriting,
    SpeechOnly,
    WritingOnly,
    Sign,
    Other,
    Unknown,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "development_level")]
#[sqlx(rename_all = "lowercase")]
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum DevelopmentLevel {
    Minimal,
    Some,
    Medium,
    Well,
    Learners,
    ActiveCommunity,
    FluentUsers,
    NativeUsers,
    Other,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "vocabulary_source")]
#[sqlx(rename_all = "lowercase")]
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum VocabularySource {
    Priori,
    Posteriori,
    Mixture,
    Other,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
}

#[derive(FromRow)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPerson {
    pub name: String
}

#[derive(FromRow)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePerson {
    pub name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub id: i32,
    pub name: String,
}

#[derive(FromRow)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewScript {
    pub name: String
}

#[derive(FromRow)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScript {
    pub name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
}

#[derive(FromRow)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewGroup {
    pub name: String
}

#[derive(FromRow)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGroup {
    pub name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conlang {
    pub id: i32,
    pub name: String,
    pub native_name: Option<String>,
    pub registry_code: Option<String>,
    pub creators: Option<Vec<i32>>,
    pub links: Option<Vec<i32>>,
    pub start_year: NaiveDateTime,
    pub physical_mode: PhysicalMode,
    pub scripts: Option<Vec<i32>>,
    pub groups: Option<Vec<i32>>,
    pub purpose: PurposeSub,
    pub vocabulary_source: VocabularySource,
    pub development: DevelopmentLevel,
    pub notes: Option<String>,
}

#[derive(FromRow)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewConlang {
    pub name: String,
    pub native_name: Option<String>,
    pub registry_code: Option<String>,
    pub creators: Option<Vec<i32>>,
    pub links: Option<Vec<i32>>,
    pub start_year: Option<NaiveDateTime>,
    pub physical_mode: PhysicalMode,
    pub scripts: Option<Vec<i32>>,
    pub groups: Option<Vec<i32>>,
    pub purpose: PurposeSub,
    pub vocabulary_source: VocabularySource,
    pub development: DevelopmentLevel,
    pub notes: Option<String>,
}

#[derive(FromRow)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConlang {
    pub name: String,
    pub native_name: Option<String>,
    pub registry_code: Option<String>,
    pub creators: Option<Vec<i32>>,
    pub links: Option<Vec<i32>>,
    pub start_year: Option<NaiveDateTime>,
    pub physical_mode: PhysicalMode,
    pub scripts: Option<Vec<i32>>,
    pub groups: Option<Vec<i32>>,
    pub purpose: PurposeSub,
    pub vocabulary_source: VocabularySource,
    pub development: DevelopmentLevel,
    pub notes: Option<String>,
}
