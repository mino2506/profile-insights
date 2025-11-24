use serde_json::Value;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsonLoadError {
    #[error("failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),
}

pub fn load_json_file<P: AsRef<Path>>(path: P) -> Result<Value, JsonLoadError> {
    let json_str = fs::read_to_string(path)?;
    let value: Value = serde_json::from_str(&json_str)?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn load_json_file_success() {
        // 一時ファイル作成
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.json");

        // 正しい JSON を書き込む
        let mut file = fs::File::create(&file_path).unwrap();
        write!(file, r#"{{"hello": "world"}}"#).unwrap();

        // 読み込み
        let value = load_json_file(&file_path).expect("should load json");

        assert_eq!(value["hello"], "world");
    }

    #[test]
    fn load_json_file_io_error() {
        // 存在しないパス
        let result = load_json_file("this_file_does_not_exist.json");

        match result {
            Err(JsonLoadError::Io(_)) => {} // OK
            other => panic!("expected Io error, got: {:?}", other),
        }
    }

    #[test]
    fn load_json_file_json_parse_error() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("invalid.json");

        // 壊れた JSON
        let mut file = fs::File::create(&file_path).unwrap();
        write!(file, r#"{{ invalid json "#).unwrap();

        let result = load_json_file(&file_path);

        match result {
            Err(JsonLoadError::JsonParse(_)) => {} // OK
            other => panic!("expected JsonParse error, got: {:?}", other),
        }
    }
}
