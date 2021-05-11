use crate::models::{Conlang, NewConlang as NewModel, UpdateConlang as UpdateModel};
use crate::queries::ConlangQuery as QueryModel;
use crate::requests::NewFullConlang;
use crate::schema::conlangs as model;
use crate::schema::conlangs::dsl::conlangs as query;
use diesel::prelude::*;

impl Conlang {
    pub async fn all(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        query.order(model::id.asc()).load(conn)
    }

    pub async fn by_id(id: i32, conn: &PgConnection) -> QueryResult<Self> {
        query.find(id).get_result(conn)
    }

    pub async fn by_name(name: String, conn: &PgConnection) -> QueryResult<Vec<Self>> {
        query
            .filter(model::name.ilike(format!("%{}%", name)))
            .load(conn)
    }

    pub async fn create(item: NewModel, conn: &PgConnection) -> QueryResult<Self> {
        diesel::insert_into(model::table)
            .values(item.clone())
            .on_conflict(model::name)
            .do_update()
            .set(item)
            .get_result(conn)
    }

    // I really do not like this approach but diesel does not apparently support set(items) :/
    pub async fn create_multiple(
        items: &[NewModel],
        conn: &PgConnection,
    ) -> QueryResult<Vec<Self>> {
        let mut vec = Vec::with_capacity(items.len());
        for item in items {
            vec.push(Self::create(item.clone(), &conn).await?)
        }
        Ok(vec)
    }

    pub async fn update(id: i32, item: UpdateModel, conn: &PgConnection) -> QueryResult<Self> {
        diesel::update(query.find(id)).set(item).get_result(conn)
    }

    pub async fn delete(id: i32, conn: &PgConnection) -> QueryResult<Self> {
        diesel::delete(query.find(id)).get_result(conn)
    }

    pub async fn by_query(item: QueryModel, conn: &PgConnection) -> QueryResult<Vec<Self>> {
        let mut request = model::table.into_boxed();

        if let Some(id) = item.id {
            request = request.filter(model::id.eq(id))
        }

        if let Some(name) = item.name {
            request = request.filter(model::name.eq(name))
        }

        if let Some(native_name) = item.native_name {
            request = request.filter(model::native_name.eq(native_name))
        }

        if let Some(registry_code) = item.registry_code {
            request = request.filter(model::registry_code.eq(registry_code))
        }

        if let Some(creators) = item.creators {
            request = request.filter(model::creators.eq(creators))
        }

        if let Some(links) = item.links {
            request = request.filter(model::links.eq(links))
        }

        if let Some(start_year) = item.start_year {
            request = request.filter(model::start_year.eq(start_year))
        }

        if let Some(physical_mode) = item.physical_mode {
            request = request.filter(model::physical_mode.eq(physical_mode))
        }

        if let Some(scripts) = item.scripts {
            request = request.filter(model::scripts.eq(scripts))
        }

        if let Some(groups) = item.groups {
            request = request.filter(model::groups.eq(groups))
        }

        if let Some(purpose) = item.purpose {
            request = request.filter(model::purpose.eq(purpose))
        }
        if let Some(vocabulary_source) = item.vocabulary_source {
            request = request.filter(model::vocabulary_source.eq(vocabulary_source))
        }

        if let Some(development) = item.development {
            request = request.filter(model::development.eq(development))
        }

        if let Some(notes) = item.notes {
            request = request.filter(model::notes.eq(notes))
        }

        request.order(model::id.asc()).load(conn)
    }
}
