use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// db-shema: wantedly_companies
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WantedlyCompany {
    pub id: i64,
    pub company_page_url: String,
    pub company_slug: String,
    pub created_at: DateTime<Utc>,
}

/// db-shema: company_attribute_source ENUM
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "company_attribute_source", rename_all = "lowercase")]
pub enum CompanyAttributeSource {
    Ai,
}

/// db-shema: wantedly_company_attributes
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WantedlyCompanyAttributes {
    pub id: i64,
    pub company_id: i64,
    pub name: Option<String>,
    pub domain: Option<String>,
    pub source: CompanyAttributeSource,
    pub confidence: Option<f32>, // NUMERIC(3,2) → とりあえず f32
    pub updated_at: DateTime<Utc>,
}
