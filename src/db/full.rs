use crate::models::{Conlang, Group, NewConlang, NewGroup, NewPerson, NewScript, Person, Script};
use crate::requests::{FullConlang, NewFullConlang};
use diesel::{PgConnection, QueryResult};

impl FullConlang {
    pub async fn create_with_sub(item: NewFullConlang, conn: &PgConnection) -> QueryResult<Self> {
        let new_persons = if item.creators.is_some() {
            Vec::<NewPerson>::from(item.clone())
        } else {
            vec![]
        };
        let new_scripts = if item.scripts.is_some() {
            Vec::<NewScript>::from(item.clone())
        } else {
            vec![]
        };
        let new_groups = if item.groups.is_some() {
            Vec::<NewGroup>::from(item.clone())
        } else {
            vec![]
        };

        let persons = Person::create_multiple(&new_persons, conn).await?;
        let scripts = Script::create_multiple(&new_scripts, conn).await?;
        let groups = Group::create_multiple(&new_groups, conn).await?;

        let new_conlang = NewConlang::from((item, &groups[..], &persons[..], &scripts[..]));
        let conlang = Conlang::create(new_conlang, conn).await?;

        Ok(FullConlang::from((
            conlang,
            &groups[..],
            &persons[..],
            &scripts[..],
        )))
    }
}
