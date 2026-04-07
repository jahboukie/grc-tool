use sqlx::{PgPool, Row, postgres::PgRow};
use uuid::Uuid;
use grc_shared::*;
use crate::db::*;

fn row_to_risk(row: &PgRow) -> RiskEntry {
    RiskEntry {
        id: row.get("id"),
        ai_system_id: row.get("ai_system_id"),
        title: row.get("title"),
        description: row.get("description"),
        risk_source: row.get("risk_source"),
        affected_rights: row.get("affected_rights"),
        likelihood: parse_enum(row.get::<String, _>("likelihood").as_str()),
        impact: parse_enum(row.get::<String, _>("impact").as_str()),
        inherent_score: row.get("inherent_score"),
        mitigation_measures: row.get("mitigation_measures"),
        residual_likelihood: row.get::<Option<String>, _>("residual_likelihood")
            .map(|s| parse_enum(&s)),
        residual_impact: row.get::<Option<String>, _>("residual_impact")
            .map(|s| parse_enum(&s)),
        residual_score: row.get("residual_score"),
        related_requirement_ids: row.get("related_requirement_ids"),
        status: parse_enum(row.get::<String, _>("status").as_str()),
        priority: parse_enum(row.get::<String, _>("priority").as_str()),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub async fn create(pool: &PgPool, dto: CreateRiskDto) -> Result<RiskEntry, String> {
    let likelihood_str = enum_to_str(&dto.likelihood);
    let impact_str = enum_to_str(&dto.impact);
    let inherent = risk_score(&dto.likelihood, &dto.impact);
    let residual_l = dto.residual_likelihood.as_ref().map(|l| enum_to_str(l));
    let residual_i = dto.residual_impact.as_ref().map(|i| enum_to_str(i));
    let residual_s = match (&dto.residual_likelihood, &dto.residual_impact) {
        (Some(l), Some(i)) => Some(risk_score(l, i)),
        _ => None,
    };
    let priority_str = enum_to_str(&dto.priority);

    let row = sqlx::query(
        "INSERT INTO risk_entries (ai_system_id, title, description, risk_source, affected_rights,
            likelihood, impact, inherent_score, mitigation_measures,
            residual_likelihood, residual_impact, residual_score,
            related_requirement_ids, priority)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
         RETURNING *"
    )
    .bind(dto.ai_system_id)
    .bind(&dto.title)
    .bind(&dto.description)
    .bind(&dto.risk_source)
    .bind(&dto.affected_rights)
    .bind(&likelihood_str)
    .bind(&impact_str)
    .bind(inherent)
    .bind(&dto.mitigation_measures)
    .bind(&residual_l)
    .bind(&residual_i)
    .bind(residual_s)
    .bind(&dto.related_requirement_ids)
    .bind(&priority_str)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row_to_risk(&row))
}

pub async fn list(pool: &PgPool, ai_system_id: Option<Uuid>) -> Result<Vec<RiskEntry>, String> {
    let rows = if let Some(sys_id) = ai_system_id {
        sqlx::query("SELECT * FROM risk_entries WHERE ai_system_id = $1 ORDER BY inherent_score DESC")
            .bind(sys_id)
            .fetch_all(pool)
            .await
    } else {
        sqlx::query("SELECT * FROM risk_entries ORDER BY inherent_score DESC")
            .fetch_all(pool)
            .await
    }
    .map_err(|e| e.to_string())?;

    Ok(rows.iter().map(row_to_risk).collect())
}

pub async fn update(pool: &PgPool, id: Uuid, dto: UpdateRiskDto) -> Result<RiskEntry, String> {
    let likelihood_str = dto.likelihood.as_ref().map(|l| enum_to_str(l));
    let impact_str = dto.impact.as_ref().map(|i| enum_to_str(i));
    let residual_l = dto.residual_likelihood.as_ref().map(|l| enum_to_str(l));
    let residual_i = dto.residual_impact.as_ref().map(|i| enum_to_str(i));
    let status_str = dto.status.as_ref().map(|s| enum_to_str(s));
    let priority_str = dto.priority.as_ref().map(|p| enum_to_str(p));

    let row = sqlx::query(
        "UPDATE risk_entries SET
            title = COALESCE($2, title),
            description = COALESCE($3, description),
            risk_source = COALESCE($4, risk_source),
            affected_rights = COALESCE($5, affected_rights),
            likelihood = COALESCE($6, likelihood),
            impact = COALESCE($7, impact),
            mitigation_measures = COALESCE($8, mitigation_measures),
            residual_likelihood = COALESCE($9, residual_likelihood),
            residual_impact = COALESCE($10, residual_impact),
            related_requirement_ids = COALESCE($11, related_requirement_ids),
            status = COALESCE($12, status),
            priority = COALESCE($13, priority),
            inherent_score = CASE
                WHEN $6 IS NOT NULL AND $7 IS NOT NULL THEN
                    (CASE $6
                        WHEN 'rare' THEN 1 WHEN 'unlikely' THEN 2 WHEN 'possible' THEN 3
                        WHEN 'likely' THEN 4 WHEN 'almost_certain' THEN 5 ELSE 3 END)
                    *
                    (CASE $7
                        WHEN 'negligible' THEN 1 WHEN 'minor' THEN 2 WHEN 'moderate' THEN 3
                        WHEN 'major' THEN 4 WHEN 'catastrophic' THEN 5 ELSE 3 END)
                WHEN $6 IS NOT NULL THEN
                    (CASE $6
                        WHEN 'rare' THEN 1 WHEN 'unlikely' THEN 2 WHEN 'possible' THEN 3
                        WHEN 'likely' THEN 4 WHEN 'almost_certain' THEN 5 ELSE 3 END)
                    *
                    (CASE impact
                        WHEN 'negligible' THEN 1 WHEN 'minor' THEN 2 WHEN 'moderate' THEN 3
                        WHEN 'major' THEN 4 WHEN 'catastrophic' THEN 5 ELSE 3 END)
                WHEN $7 IS NOT NULL THEN
                    (CASE likelihood
                        WHEN 'rare' THEN 1 WHEN 'unlikely' THEN 2 WHEN 'possible' THEN 3
                        WHEN 'likely' THEN 4 WHEN 'almost_certain' THEN 5 ELSE 3 END)
                    *
                    (CASE $7
                        WHEN 'negligible' THEN 1 WHEN 'minor' THEN 2 WHEN 'moderate' THEN 3
                        WHEN 'major' THEN 4 WHEN 'catastrophic' THEN 5 ELSE 3 END)
                ELSE inherent_score END,
            residual_score = CASE
                WHEN $9 IS NOT NULL AND $10 IS NOT NULL THEN
                    (CASE COALESCE($9, residual_likelihood)
                        WHEN 'rare' THEN 1 WHEN 'unlikely' THEN 2 WHEN 'possible' THEN 3
                        WHEN 'likely' THEN 4 WHEN 'almost_certain' THEN 5 ELSE NULL END)
                    *
                    (CASE COALESCE($10, residual_impact)
                        WHEN 'negligible' THEN 1 WHEN 'minor' THEN 2 WHEN 'moderate' THEN 3
                        WHEN 'major' THEN 4 WHEN 'catastrophic' THEN 5 ELSE NULL END)
                ELSE residual_score END
         WHERE id = $1
         RETURNING *"
    )
    .bind(id)
    .bind(&dto.title)
    .bind(&dto.description)
    .bind(&dto.risk_source)
    .bind(&dto.affected_rights)
    .bind(&likelihood_str)
    .bind(&impact_str)
    .bind(&dto.mitigation_measures)
    .bind(&residual_l)
    .bind(&residual_i)
    .bind(&dto.related_requirement_ids)
    .bind(&status_str)
    .bind(&priority_str)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row_to_risk(&row))
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM risk_entries WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn get_matrix_data(
    pool: &PgPool,
    engagement_id: Option<Uuid>,
    ai_system_id: Option<Uuid>,
) -> Result<RiskMatrixData, String> {
    let entries = if let Some(sys_id) = ai_system_id {
        list(pool, Some(sys_id)).await?
    } else if let Some(eng_id) = engagement_id {
        let rows = sqlx::query(
            "SELECT r.* FROM risk_entries r
             JOIN ai_systems a ON r.ai_system_id = a.id
             WHERE a.engagement_id = $1
             ORDER BY r.inherent_score DESC"
        )
        .bind(eng_id)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
        rows.iter().map(row_to_risk).collect()
    } else {
        list(pool, None).await?
    };

    // Build 5×5 matrix: matrix[likelihood_idx][impact_idx] → risk IDs
    // Indices are 0-based (0 = Rare/Negligible, 4 = AlmostCertain/Catastrophic)
    let mut matrix: Vec<Vec<Vec<Uuid>>> = vec![vec![vec![]; 5]; 5];
    for entry in &entries {
        let li = (entry.likelihood.value() - 1) as usize;
        let ii = (entry.impact.value() - 1) as usize;
        matrix[li][ii].push(entry.id);
    }

    Ok(RiskMatrixData { entries, matrix })
}
