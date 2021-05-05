use crate::models::{Conlang as Model, NewConlang as NewModel, UpdateConlang as UpdateModel};
use anyhow::Result;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;

impl Model {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let recs = sqlx::query(
            r#"
                SELECT *
                FROM conlangs
                ORDER BY id
            "#,
        )
        .fetch_all(pool)
        .await?;

        for row in recs {
            items.push(Self {
                id: row.get(0),
                name: row.get(1),
                native_name: row.get(2),
                registry_code: row.get(3),
                creators: row.get(4),
                links: row.get(5),
                start_year: row.get(6),
                physical_mode: row.get(7),
                scripts: row.get(8),
                groups: row.get(9),
                purpose: row.get(10),
                vocabulary_source: row.get(11),
                development: row.get(12),
                notes: row.get(13),
            });
        }

        Ok(items)
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let row = sqlx::query(
            r#"
                SELECT * FROM conlangs WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: row.get(0),
            name: row.get(1),
            native_name: row.get(2),
            registry_code: row.get(3),
            creators: row.get(4),
            links: row.get(5),
            start_year: row.get(6),
            physical_mode: row.get(7),
            scripts: row.get(8),
            groups: row.get(9),
            purpose: row.get(10),
            vocabulary_source: row.get(11),
            development: row.get(12),
            notes: row.get(13),
        })
    }

    pub async fn by_name(name: String, pool: &PgPool) -> Result<Self> {
        let row = sqlx::query(
            r#"
                SELECT * FROM conlangs WHERE name = $1
            "#,
        )
        .bind(name)
        .fetch_one(pool)
        .await?;

        Ok(Self {
            id: row.get(0),
            name: row.get(1),
            native_name: row.get(2),
            registry_code: row.get(3),
            creators: row.get(4),
            links: row.get(5),
            start_year: row.get(6),
            physical_mode: row.get(7),
            scripts: row.get(8),
            groups: row.get(9),
            purpose: row.get(10),
            vocabulary_source: row.get(11),
            development: row.get(12),
            notes: row.get(13),
        })
    }

    pub async fn create(item: NewModel, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let created = sqlx::query(
            r#"
                INSERT INTO conlangs
                (name, native_name, registry_code, creators, links, start_year, physical_mode, scripts, groups, purpose, vocabulary_source, development, notes)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                RETURNING *
            "#,
        )
            .bind(&item.name)
            .map(|row: PgRow| Self {
                id: row.get(0),
                name: row.get(1),
                native_name: row.get(2),
                registry_code: row.get(3),
                creators: row.get(4),
                links: row.get(5),
                start_year: row.get(6),
                physical_mode: row.get(7),
                scripts: row.get(8),
                groups: row.get(9),
                purpose: row.get(10),
                vocabulary_source: row.get(11),
                development: row.get(12),
                notes: row.get(13)
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(created)
    }

    pub async fn update(id: i32, item: UpdateModel, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let updated = sqlx::query(
            r#"
                UPDATE conlangs
                SET name = $1,
                native_name = $2,
                registry_code = $3,
                creators = $4,
                links = $5,
                start_year = $6,
                physical_mode = $7,
                scripts = $8,
                groups = $9,
                purpose = $10,
                vocabulary_source = $11,
                development = $12,
                notes = $13
                WHERE id = $14
                RETURNING *
            "#,
        )
        .bind(&item.name)
        .bind(id)
        .map(|row: PgRow| Self {
            id: row.get(0),
            name: row.get(1),
            native_name: row.get(2),
            registry_code: row.get(3),
            creators: row.get(4),
            links: row.get(5),
            start_year: row.get(6),
            physical_mode: row.get(7),
            scripts: row.get(8),
            groups: row.get(9),
            purpose: row.get(10),
            vocabulary_source: row.get(11),
            development: row.get(12),
            notes: row.get(13),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(updated)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<bool> {
        let mut tx = pool.begin().await?;
        sqlx::query(
            r#"
                DELETE FROM conlangs
                WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(true)
    }
}
