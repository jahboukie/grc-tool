use sqlx::{PgPool, Row, postgres::PgRow};
use uuid::Uuid;
use grc_shared::*;
use crate::db::*;

fn row_to_requirement(row: &PgRow) -> FrameworkRequirement {
    FrameworkRequirement {
        id: row.get("id"),
        framework: parse_enum(row.get::<String, _>("framework").as_str()),
        reference_id: row.get("reference_id"),
        title: row.get("title"),
        description: row.get("description"),
        article_clause: row.get("article_clause"),
        category: row.get("category"),
        subcategory: row.get("subcategory"),
        applicable_risk_categories: parse_enum_vec(&row.get::<Vec<String>, _>("applicable_risk_categories")),
        applicable_roles: parse_enum_vec(&row.get::<Vec<String>, _>("applicable_roles")),
        is_mandatory: row.get("is_mandatory"),
        guidance_text: row.get("guidance_text"),
        implementation_notes: row.get("implementation_notes"),
        sort_order: row.get("sort_order"),
    }
}

pub async fn list(
    pool: &PgPool,
    framework: Option<String>,
    category: Option<String>,
) -> Result<Vec<FrameworkRequirement>, String> {
    let rows = match (&framework, &category) {
        (Some(fw), Some(cat)) => {
            sqlx::query(
                "SELECT * FROM framework_requirements WHERE framework = $1 AND category = $2 ORDER BY sort_order"
            )
            .bind(fw)
            .bind(cat)
            .fetch_all(pool)
            .await
        }
        (Some(fw), None) => {
            sqlx::query(
                "SELECT * FROM framework_requirements WHERE framework = $1 ORDER BY sort_order"
            )
            .bind(fw)
            .fetch_all(pool)
            .await
        }
        (None, Some(cat)) => {
            sqlx::query(
                "SELECT * FROM framework_requirements WHERE category = $1 ORDER BY framework, sort_order"
            )
            .bind(cat)
            .fetch_all(pool)
            .await
        }
        (None, None) => {
            sqlx::query("SELECT * FROM framework_requirements ORDER BY framework, sort_order")
                .fetch_all(pool)
                .await
        }
    }
    .map_err(|e| e.to_string())?;

    Ok(rows.iter().map(row_to_requirement).collect())
}

pub async fn get(pool: &PgPool, id: Uuid) -> Result<FrameworkRequirement, String> {
    let row = sqlx::query("SELECT * FROM framework_requirements WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(row_to_requirement(&row))
}

pub async fn search(pool: &PgPool, query: &str) -> Result<Vec<FrameworkRequirement>, String> {
    let pattern = format!("%{}%", query);
    let rows = sqlx::query(
        "SELECT * FROM framework_requirements
         WHERE title ILIKE $1 OR description ILIKE $1 OR reference_id ILIKE $1
         ORDER BY framework, sort_order"
    )
    .bind(&pattern)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows.iter().map(row_to_requirement).collect())
}
