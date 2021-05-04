use crate::models::{Person as Model, NewPerson as NewModel, UpdatePerson as UpdateModel};
use anyhow::Result;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;

impl Model {
    pub async fn all(pool: &PgPool) -> Result<Vec<Self>> {
        let mut items = Vec::new();

        let recs = sqlx::query!(
            r#"
                SELECT id, name
                    FROM persons
                ORDER BY id
            "#
        )
            .fetch_all(pool)
            .await?;

        for rec in recs {
            items.push(Self {
                id: rec.id,
                name: rec.name,
            });
        }

        Ok(items)
    }

    pub async fn by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM persons WHERE id = $1
            "#,
            id
        )
            .fetch_one(pool)
            .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
        })
    }

    pub async fn by_name(name: String, pool: &PgPool) -> Result<Self> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM persons WHERE name = $1
            "#,
            name
        )
            .fetch_one(pool)
            .await?;

        Ok(Self {
            id: rec.id,
            name: rec.name,
        })
    }

    pub async fn create(item: NewModel, pool: &PgPool) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let created = sqlx::query(
            r#"
                INSERT INTO persons (name) VALUES ($1)
                RETURNING id, name
            "#,
        )
            .bind(&item.name)
            .map(|row: PgRow| Self {
                id: row.get(0),
                name: row.get(1),
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
                UPDATE persons SET name = $1
                WHERE id = $2
                RETURNING id, name
            "#,
        )
            .bind(&item.name)
            .bind(id)
            .map(|row: PgRow| Self {
                id: row.get(0),
                name: row.get(1),
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
                DELETE FROM persons
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
