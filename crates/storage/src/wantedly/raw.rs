use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
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

impl NewWantedlyProfileViewRaw {
    pub fn from_node(
        node: &Value,
        fetched_at: DateTime<Utc>,
    ) -> Result<Self, WantedlyProfileViewRawError> {
        let viewer_user_id = node
            .get("userId")
            .and_then(|v| v.as_i64())
            .ok_or(WantedlyProfileViewRawError::JsonStructure(
                "node.userId must be i64",
            ))?
            .to_string();

        let viewer_company_page_url = node
            .get("companyPageUrl")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let viewer_company_name_raw = node
            .get("shortDescription")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let viewed_at_raw = node
            .get("profileImpressionMeta")
            .and_then(|m| m.get("impressedDateTime"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let viewed_at = parse_viewed_at(&viewed_at_raw, fetched_at).ok_or(
            WantedlyProfileViewRawError::InvalidDate {
                raw: viewed_at_raw.clone(),
            },
        )?;

        Ok(Self {
            viewer_user_id,
            viewer_company_page_url,
            viewer_company_name_raw,
            viewed_at_raw,
            viewed_at,
            raw_json: node.clone(),
        })
    }
}

pub async fn insert_profile_view_raw(
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

fn parse_viewed_at(raw: &str, fetched_at: DateTime<Utc>) -> Option<DateTime<Utc>> {
    let base_date = fetched_at.date_naive();

    if raw == "今日" {
        return Some(date_to_utc_midnight(base_date));
    }

    if let Some(idx) = raw.find("日前") {
        let days: i64 = raw[..idx].parse().ok()?;
        let d = base_date - Duration::days(days);
        return Some(date_to_utc_midnight(d));
    }

    None
}

fn date_to_utc_midnight(date: NaiveDate) -> DateTime<Utc> {
    let dt = NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)
}
