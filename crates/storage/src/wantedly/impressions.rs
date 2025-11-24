use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// db-shema: wantedly_impressions
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WantedlyImpression {
    pub id: i64,
    pub viewer_id: i64,
    pub company_id_at_view: i64,
    pub impressed_at: DateTime<Utc>,
    pub raw_profile_view_id: i64,
    pub created_at: DateTime<Utc>,
}
