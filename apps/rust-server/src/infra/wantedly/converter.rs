use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde_json::Value;

use crate::infra::wantedly::dto::WantedlyProfileViewNode;

use storage::wantedly::NewWantedlyProfileViewRaw;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WantedlyProfileViewConvertError {
    #[error("invalid impression date: {raw}")]
    InvalidDate { raw: String },
}

pub fn convert_wantedly_json_node_to_storage(
    json_node_dto: &WantedlyProfileViewNode,
    raw_json: Value,
    snapshot_at: DateTime<Utc>,
) -> Result<NewWantedlyProfileViewRaw, WantedlyProfileViewConvertError> {
    let viewer_user_id = json_node_dto.user_id.to_string();
    let viewer_company_page_url = json_node_dto.company_page_url.clone();
    let viewer_company_name_raw = json_node_dto.short_description.clone(); // TODO: 分岐

    let viewed_at_raw = json_node_dto
        .profile_impression_meta
        .impressed_date_time
        .clone();

    let viewed_at = parse_viewed_at(&viewed_at_raw, snapshot_at).ok_or(
        WantedlyProfileViewConvertError::InvalidDate {
            raw: viewed_at_raw.clone(),
        },
    )?;

    Ok(NewWantedlyProfileViewRaw {
        viewer_user_id,
        viewer_company_page_url,
        viewer_company_name_raw,
        viewed_at_raw,
        viewed_at,
        raw_json,
    })
}

fn parse_viewed_at(raw: &str, snapshot_at: DateTime<Utc>) -> Option<DateTime<Utc>> {
    let base_date = snapshot_at.date_naive();

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
