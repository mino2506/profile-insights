use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum WantedlyJsonError {
    #[error(
        "invalid JSON structure: expected data.profileImpressionPage.impressedUsers.edges as array"
    )]
    InvalidStructure,
}

pub fn extract_impressed_user_edges(json_value: &Value) -> Result<&Vec<Value>, WantedlyJsonError> {
    json_value
        .get("data")
        .and_then(|d| d.get("profileImpressionPage"))
        .and_then(|pi| pi.get("impressedUsers"))
        .and_then(|iu| iu.get("edges"))
        .and_then(|e| e.as_array())
        .ok_or(WantedlyJsonError::InvalidStructure)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_impressed_user_edges_ok() {
        let sample = json!({
            "data": {
                "profileImpressionPage": {
                    "impressedUsers": {
                        "edges": [
                            { "node": { "id": 1 } },
                            { "node": { "id": 2 } }
                        ]
                    }
                }
            }
        });

        let result = extract_impressed_user_edges(&sample);

        assert!(result.is_ok());
        let edges = result.unwrap();
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_extract_impressed_user_edges_invalid_structure() {
        let bad = json!({
            "data": {}
        });

        let result = extract_impressed_user_edges(&bad);
        assert!(matches!(result, Err(WantedlyJsonError::InvalidStructure)));
    }
}
