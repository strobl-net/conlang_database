use crate::models::{Group as Model, NewGroup as NewModel, UpdateGroup as UpdateModel};
use crate::queries::GroupQuery as QueryModel;
use crate::schema::groups as model;
use crate::schema::groups::dsl::groups as query;
use diesel::prelude::*;

impl Model {
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
            .values(item)
            .get_result(conn)
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
