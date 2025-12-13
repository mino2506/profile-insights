use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::PgPool;
use thiserror::Error;

use crate::infra::{
    json_loader::{JsonLoadError, load_json_file},
    wantedly::{
        converter::{WantedlyProfileViewConvertError, convert_wantedly_json_node_to_storage},
        dto::{WantedlyProfileViewNode, WantedlyProfileViewNodeError},
        json::{WantedlyJsonStructureError, extract_impressed_user_edges},
    },
};
use storage::wantedly::{
    NewWantedlyProfileViewRaw, WantedlyProfileViewRawError, upsert_profile_view_raw,
};

#[derive(Debug, Error)]
pub enum WantedlyImportError {
    #[error("failed to load json file: {0}")]
    JsonLoad(#[from] JsonLoadError),

    #[error("wantedly json error: {0}")]
    WantedlyJson(#[from] WantedlyJsonStructureError),

    #[error("wantedly profile view json node error: {0}")]
    WantedlyProfileViewJsonNode(#[from] WantedlyProfileViewNodeError),

    #[error("wantedly profile view convert error: {0}")]
    WantedlyProfileViewConvert(#[from] WantedlyProfileViewConvertError),

    #[error("failed to process one record: {0}")]
    RawRecord(#[from] WantedlyProfileViewRawError),

    #[error("failed to process one record: {0}")]
    MissingNode(&'static str),
}

pub async fn import_one_profile_view(
    pool: &PgPool,
    json_node: &Value,
    snapshot_at: DateTime<Utc>,
) -> Result<i64, WantedlyImportError> {
    let profile_view_node: WantedlyProfileViewNode =
        WantedlyProfileViewNode::from_value(json_node)?;
    let storage_dto: NewWantedlyProfileViewRaw =
        convert_wantedly_json_node_to_storage(&profile_view_node, json_node.clone(), snapshot_at)?;

    // let inserted = insert_profile_view_raw(pool, &storage_dto).await?;
    let inserted = upsert_profile_view_raw(pool, &storage_dto).await?;
    Ok(inserted)
}

pub async fn import_wantedly_profile_views_from_file(
    pool: &PgPool,
    path: &str,
    snapshot_at: DateTime<Utc>,
) -> Result<usize, WantedlyImportError> {
    let json = load_json_file(path)?;
    let nodes: &Vec<Value> = extract_impressed_user_edges(&json)?;
    println!("{:?}", serde_json::to_string_pretty(nodes).unwrap());

    let mut count = 0;
    for edge in nodes {
        let node_value = edge.get("node").ok_or(WantedlyImportError::MissingNode(
            "missing `node` field in edge",
        ))?;
        import_one_profile_view(pool, node_value, snapshot_at).await?;
        count += 1;
    }

    Ok(count)
}
