use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, PgPool};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WantedlyProfileViewRawError {
    #[error("json structure mismatch: {0}")]
    JsonStructure(&'static str),

    #[error("invalid impression date: {raw}")]
    InvalidDate { raw: String },

    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
}

/// db-shema: wantedly_profile_view_raw
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WantedlyProfileViewRaw {
    pub id: i64,
    pub viewer_user_id: String,
    pub viewer_company_page_url: Option<String>,
    pub viewer_company_name_raw: Option<String>,
    pub viewed_at_raw: String,
    pub viewed_at: DateTime<Utc>,
    pub raw_json: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewWantedlyProfileViewRaw {
    pub viewer_user_id: String,
    pub viewer_company_page_url: Option<String>,
    pub viewer_company_name_raw: Option<String>,
    pub viewed_at_raw: String,
    pub viewed_at: DateTime<Utc>,
    pub raw_json: Value,
}

pub async fn insert_profile_view_raw_strict(
    pool: &PgPool,
    new: &NewWantedlyProfileViewRaw,
) -> Result<i64, WantedlyProfileViewRawError> {
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO wantedly_profile_view_raw (
            viewer_user_id,
            viewer_company_page_url,
            viewer_company_name_raw,
            viewed_at_raw,
            viewed_at,
            raw_json
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        new.viewer_user_id,
        new.viewer_company_page_url,
        new.viewer_company_name_raw,
        new.viewed_at_raw,
        new.viewed_at,
        new.raw_json,
    )
    .fetch_one(pool)
    .await?;

    Ok(id)
}

pub async fn upsert_profile_view_raw(
    pool: &PgPool,
    new: &NewWantedlyProfileViewRaw,
) -> Result<i64, WantedlyProfileViewRawError> {
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO wantedly_profile_view_raw (
            viewer_user_id,
            viewer_company_page_url,
            viewer_company_name_raw,
            viewed_at_raw,
            viewed_at,
            raw_json
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (viewer_user_id, viewed_at)
        DO UPDATE SET
            viewer_company_page_url = EXCLUDED.viewer_company_page_url,
            viewer_company_name_raw = EXCLUDED.viewer_company_name_raw,
            viewed_at_raw           = EXCLUDED.viewed_at_raw,
            raw_json                = EXCLUDED.raw_json
        RETURNING id
        "#,
        new.viewer_user_id,
        new.viewer_company_page_url,
        new.viewer_company_name_raw,
        new.viewed_at_raw,
        new.viewed_at,
        new.raw_json,
    )
    .fetch_one(pool)
    .await?;

    Ok(id)
}
