use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WantedlyProfileViewNodeError {
    #[error("json decode error for profile view node: {0}")]
    Decode(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Deserialize)]
pub struct WantedlyProfileViewNode {
    #[serde(rename = "userId")]
    pub user_id: i64,

    #[serde(rename = "shortDescription")]
    pub short_description: Option<String>,

    #[serde(rename = "companyPageUrl")]
    pub company_page_url: Option<String>,

    #[serde(rename = "profileImpressionMeta")]
    pub profile_impression_meta: ProfileImpressionMeta,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProfileImpressionMeta {
    #[serde(rename = "impressedDateTime")]
    pub impressed_date_time: String,
}

impl WantedlyProfileViewNode {
    pub fn from_value(value: &Value) -> Result<Self, WantedlyProfileViewNodeError> {
        let node = serde_json::from_value(value.clone())?;
        Ok(node)
    }
}
