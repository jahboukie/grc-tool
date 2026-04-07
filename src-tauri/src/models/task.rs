use sqlx::{PgPool, Row, postgres::PgRow};
use uuid::Uuid;
use grc_shared::*;
use crate::db::*;

fn row_to_task(row: &PgRow) -> Task {
    Task {
        id: row.get("id"),
        engagement_id: row.get("engagement_id"),
        ai_system_id: row.get("ai_system_id"),
        title: row.get("title"),
        description: row.get("description"),
        framework: row.get::<Option<String>, _>("framework").map(|s| parse_enum(&s)),
        related_requirement_id: row.get("related_requirement_id"),
        status: parse_enum(row.get::<String, _>("status").as_str()),
        priority: parse_enum(row.get::<String, _>("priority").as_str()),
        due_date: row.get("due_date"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub async fn create(pool: &PgPool, dto: CreateTaskDto) -> Result<Task, String> {
    let fw_str = dto.framework.as_ref().map(|f| enum_to_str(f));
    let priority_str = enum_to_str(&dto.priority);

    let row = sqlx::query(
        "INSERT INTO tasks (engagement_id, ai_system_id, title, description, framework,
            related_requirement_id, priority, due_date)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING *"
    )
    .bind(dto.engagement_id)
    .bind(dto.ai_system_id)
    .bind(&dto.title)
    .bind(&dto.description)
    .bind(&fw_str)
    .bind(dto.related_requirement_id)
    .bind(&priority_str)
    .bind(dto.due_date)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row_to_task(&row))
}

pub async fn list(
    pool: &PgPool,
    engagement_id: Option<Uuid>,
    status: Option<String>,
    priority: Option<String>,
) -> Result<Vec<Task>, String> {
    let rows = sqlx::query(
        "SELECT * FROM tasks
         WHERE ($1::uuid IS NULL OR engagement_id = $1)
           AND ($2::text IS NULL OR status = $2)
           AND ($3::text IS NULL OR priority = $3)
         ORDER BY
            CASE priority WHEN 'critical' THEN 0 WHEN 'high' THEN 1
                          WHEN 'medium' THEN 2 WHEN 'low' THEN 3 END,
            due_date ASC NULLS LAST"
    )
    .bind(engagement_id)
    .bind(&status)
    .bind(&priority)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows.iter().map(row_to_task).collect())
}

pub async fn update(pool: &PgPool, id: Uuid, dto: UpdateTaskDto) -> Result<Task, String> {
    let fw_str = dto.framework.as_ref().map(|f| enum_to_str(f));
    let status_str = dto.status.as_ref().map(|s| enum_to_str(s));
    let priority_str = dto.priority.as_ref().map(|p| enum_to_str(p));

    let row = sqlx::query(
        "UPDATE tasks SET
            title = COALESCE($2, title),
            description = COALESCE($3, description),
            framework = COALESCE($4, framework),
            related_requirement_id = COALESCE($5, related_requirement_id),
            status = COALESCE($6, status),
            priority = COALESCE($7, priority),
            due_date = COALESCE($8, due_date)
         WHERE id = $1
         RETURNING *"
    )
    .bind(id)
    .bind(&dto.title)
    .bind(&dto.description)
    .bind(&fw_str)
    .bind(dto.related_requirement_id)
    .bind(&status_str)
    .bind(&priority_str)
    .bind(dto.due_date)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row_to_task(&row))
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
