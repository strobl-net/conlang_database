use crate::models::{Group, NewGroup as NewModel, UpdateGroup as UpdateModel};
use crate::queries::GroupQuery as QueryModel;
use crate::schema::groups as model;
use crate::schema::groups::dsl::groups as query;
use diesel::prelude::*;

impl Group {
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

        request.order(model::id.asc()).load(conn)
    }
}
