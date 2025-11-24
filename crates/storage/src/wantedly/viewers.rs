use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// db-shema: wantedly_viewers
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WantedlyViewer {
    pub id: i64,
    pub source_user_id: String,
    pub company_id: Option<i64>,
    pub created_at: DateTime<Utc>,
}
