use crate::db::PgPool;
use crate::models::{Conlang as Model, NewConlang as NewModel, UpdateConlang as UpdateModel};
use crate::queries::ConlangQuery as QueryModel;
use crate::requests::{FullConlang, NewFullConlang};
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, Error, HttpRequest, HttpResponse};

pub fn endpoints(config: &mut ServiceConfig) {
    config
        .service(all)
        .service(by_id)
        .service(by_name)
        .service(new)
        .service(new_full)
        .service(update)
        .service(delete);
}

#[get("/api/conlangs")]
pub async fn all(pool: web::Data<PgPool>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    if request.query_string().is_empty() {
        let items = Model::all(&conn).await.unwrap();
        Ok(HttpResponse::Ok().json(items))
    } else {
        match serde_qs::from_str::<QueryModel>(&request.query_string()) {
            Ok(query) => {
                let items = Model::by_query(query, &conn).await.unwrap();
                Ok(HttpResponse::Ok().json(items))
            }
            Err(_) => Ok(HttpResponse::UnprocessableEntity().finish()),
        }
    }
}

#[get("/api/conlangs/{id}")]
pub async fn by_id(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let id = id.into_inner();
    let item = Model::by_id(id, &conn).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[get("/api/conlangs/by-name/{name}")]
pub async fn by_name(
    pool: web::Data<PgPool>,
    name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let name = name.into_inner();
    let item = Model::by_name(name, &conn).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[post("/api/conlangs")]
pub async fn new(
    pool: web::Data<PgPool>,
    item: web::Json<NewModel>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let item = item.into_inner();
    let item = Model::create(item, &conn).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[post("/api/f/conlangs")]
pub async fn new_full(
    pool: web::Data<PgPool>,
    item: web::Json<NewFullConlang>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let item = item.into_inner();
    let item = FullConlang::create_with_sub(item, &conn).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[patch("/api/conlangs/{id}")]
pub async fn update(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    item: web::Json<UpdateModel>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let id = id.into_inner();
    let item = item.into_inner();
    let item = Model::update(id, item, &conn).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[delete("/api/conlangs/{id}")]
pub async fn delete(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let id = id.into_inner();
    let item = Model::delete(id, &conn).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}
