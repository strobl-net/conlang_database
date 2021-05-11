use crate::models::{
    Conlang, DevelopmentLevel, Group, NewConlang, NewGroup, NewPerson, NewScript, Person,
    PhysicalMode, PurposeSub, Script, VocabularySource,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullConlang {
    pub id: i32,
    pub name: String,
    pub native_name: Option<String>,
    pub registry_code: Option<String>,
    pub creators: Option<Vec<Person>>,
    pub links: Option<Vec<String>>,
    pub start_year: Option<NaiveDateTime>,
    pub physical_mode: PhysicalMode,
    pub scripts: Option<Vec<Script>>,
    pub groups: Option<Vec<Group>>,
    pub purpose: PurposeSub,
    pub vocabulary_source: VocabularySource,
    pub development: DevelopmentLevel,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewFullConlang {
    pub name: String,
    pub native_name: Option<String>,
    pub registry_code: Option<String>,
    pub creators: Option<Vec<String>>,
    pub links: Option<Vec<String>>,
    pub start_year: Option<NaiveDateTime>,
    pub physical_mode: PhysicalMode,
    pub scripts: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
    pub purpose: PurposeSub,
    pub vocabulary_source: VocabularySource,
    pub development: DevelopmentLevel,
    pub notes: Option<String>,
}

impl From<NewFullConlang> for Vec<NewGroup> {
    fn from(rhs: NewFullConlang) -> Self {
        let groups = rhs.groups.unwrap();
        let mut arr = Vec::with_capacity(groups.len());
        for group in groups {
            arr.push(NewGroup { name: group })
        }
        arr
    }
}

impl From<NewFullConlang> for Vec<NewScript> {
    fn from(rhs: NewFullConlang) -> Self {
        let groups = rhs.scripts.unwrap();
        let mut arr = Vec::with_capacity(groups.len());
        for group in groups {
            arr.push(NewScript { name: group })
        }
        arr
    }
}

impl From<NewFullConlang> for Vec<NewPerson> {
    fn from(rhs: NewFullConlang) -> Self {
        let groups = rhs.creators.unwrap();
        let mut arr = Vec::with_capacity(groups.len());
        for group in groups {
            arr.push(NewPerson { name: group })
        }
        arr
    }
}

impl From<(NewFullConlang, &[Group], &[Person], &[Script])> for NewConlang {
    fn from(rhs: (NewFullConlang, &[Group], &[Person], &[Script])) -> Self {
        let creators = if rhs.2.len() > 0 {
            let mut values = Vec::with_capacity(rhs.2.len());
            for person in rhs.2 {
                values.push(person.id)
            }
            Some(values)
        } else {
            None
        };

        let scripts = if rhs.3.len() > 0 {
            let mut values = Vec::with_capacity(rhs.2.len());
            for script in rhs.3 {
                values.push(script.id)
            }
            Some(values)
        } else {
            None
        };

        let groups = if rhs.1.len() > 0 {
            let mut values = Vec::with_capacity(rhs.2.len());
            for group in rhs.1 {
                values.push(group.id)
            }
            Some(values)
        } else {
            None
        };

        Self {
            name: rhs.0.name,
            native_name: rhs.0.native_name,
            registry_code: rhs.0.registry_code,
            creators,
            links: rhs.0.links,
            start_year: rhs.0.start_year,
            physical_mode: rhs.0.physical_mode,
            scripts,
            groups,
            purpose: rhs.0.purpose,
            vocabulary_source: rhs.0.vocabulary_source,
            development: rhs.0.development,
            notes: rhs.0.notes,
        }
    }
}

impl From<(Conlang, &[Group], &[Person], &[Script])> for FullConlang {
    fn from(rhs: (Conlang, &[Group], &[Person], &[Script])) -> Self {
        Self {
            id: 0,
            name: rhs.0.name,
            native_name: rhs.0.native_name,
            registry_code: rhs.0.registry_code,
            creators: if rhs.2.is_empty() {
                None
            } else {
                Some(rhs.2.to_vec())
            },
            links: rhs.0.links,
            start_year: rhs.0.start_year,
            physical_mode: rhs.0.physical_mode,
            scripts: if rhs.3.is_empty() {
                None
            } else {
                Some(rhs.3.to_vec())
            },
            groups: if rhs.1.is_empty() {
                None
            } else {
                Some(rhs.1.to_vec())
            },
            purpose: rhs.0.purpose,
            vocabulary_source: rhs.0.vocabulary_source,
            development: rhs.0.development,
            notes: rhs.0.notes,
        }
    }
}
