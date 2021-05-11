use crate::schema::conlangs;
use crate::schema::groups;
use crate::schema::persons;
use crate::schema::scripts;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub use enums::{DevelopmentLevel, PhysicalMode, PurposeSub, VocabularySource};

pub mod enums {
    use serde::{Deserialize, Serialize};

    #[derive(DbEnum)]
    #[PgType = "purpose_sub"]
    #[DieselType = "Purpose_sub"]
    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum PurposeSub {
        Personal = 0,
        Jokelang = 1,
        StoryBased = 2,
        Conworld = 3,
        Geofictional = 4,
        Future = 5,
        AlternateHistory = 6,
        Lostlang = 7,
        Xenolang = 8,
        PseudoAuxlang = 9,
        GlobalAuxlang = 10,
        ZonalAuxlang = 11,
        OtherAuxlang = 12,
        Ideal = 13,
        Philosophical = 14,
        Logical = 15,
        Experimental = 16,
        Conpidgin = 17,
        Secret = 18,
        Other = 19,
    }

    #[derive(DbEnum)]
    #[PgType = "physical_mode"]
    #[DieselType = "Physical_mode"]
    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum PhysicalMode {
        SpeechWriting = 0,
        SpeechOnly = 1,
        WritingOnly = 2,
        Sign = 3,
        Other = 4,
        Unknown = 5,
    }

    #[derive(DbEnum)]
    #[PgType = "development_level"]
    #[DieselType = "Development_level"]
    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum DevelopmentLevel {
        Minimal = 0,
        Some = 1,
        Medium = 2,
        Well = 3,
        Learners = 4,
        ActiveCommunity = 5,
        FluentUsers = 6,
        NativeUsers = 7,
        Other = 8,
    }

    #[derive(DbEnum)]
    #[PgType = "vocabulary_source"]
    #[DieselType = "Vocabulary_source"]
    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum VocabularySource {
        Priori = 0,
        Posteriori = 1,
        Mixture = 2,
        Other = 3,
        Unknown = 4,
    }
}

#[derive(Queryable, Identifiable, Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "persons"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPerson {
    pub name: String,
}

#[derive(AsChangeset)]
#[table_name = "persons"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePerson {
    pub name: String,
}

#[derive(Queryable, Identifiable, Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "scripts"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewScript {
    pub name: String,
}

#[derive(AsChangeset)]
#[table_name = "scripts"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScript {
    pub name: String,
}

#[derive(Queryable, Identifiable, Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "groups"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewGroup {
    pub name: String,
}

#[derive(AsChangeset)]
#[table_name = "groups"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGroup {
    pub name: String,
}

#[derive(Queryable, Identifiable, Debug, Clone, Serialize, Deserialize)]
pub struct Conlang {
    pub id: i32,
    pub name: String,
    pub native_name: Option<String>,
    pub registry_code: Option<String>,
    pub creators: Option<Vec<i32>>,
    pub links: Option<Vec<String>>,
    pub start_year: Option<NaiveDateTime>,
    pub physical_mode: PhysicalMode,
    pub scripts: Option<Vec<i32>>,
    pub groups: Option<Vec<i32>>,
    pub purpose: PurposeSub,
    pub vocabulary_source: VocabularySource,
    pub development: DevelopmentLevel,
    pub notes: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "conlangs"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewConlang {
    pub name: String,
    pub native_name: Option<String>,
    pub registry_code: Option<String>,
    pub creators: Option<Vec<i32>>,
    pub links: Option<Vec<String>>,
    pub start_year: Option<NaiveDateTime>,
    pub physical_mode: PhysicalMode,
    pub scripts: Option<Vec<i32>>,
    pub groups: Option<Vec<i32>>,
    pub purpose: PurposeSub,
    pub vocabulary_source: VocabularySource,
    pub development: DevelopmentLevel,
    pub notes: Option<String>,
}

#[derive(AsChangeset)]
#[table_name = "conlangs"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConlang {
    pub name: String,
    pub native_name: Option<String>,
    pub registry_code: Option<String>,
    pub creators: Option<Vec<i32>>,
    pub links: Option<Vec<String>>,
    pub start_year: Option<NaiveDateTime>,
    pub physical_mode: PhysicalMode,
    pub scripts: Option<Vec<i32>>,
    pub groups: Option<Vec<i32>>,
    pub purpose: PurposeSub,
    pub vocabulary_source: VocabularySource,
    pub development: DevelopmentLevel,
    pub notes: Option<String>,
}
