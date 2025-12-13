use axum::{Router, extract::path};
use db::check_connection;
use dotenvy::dotenv;
use std::fs;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

mod config;
mod error;
mod migrate;
mod routes;

mod infra;

use infra::usecase::import_wantedly_profile_views::import_wantedly_profile_views_from_file;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("fatal error: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // .env読み込み
    dotenv().ok();

    // DB接続確認
    let db_url = config::build_database_url_from_env()?;
    let pool = db::establish_connection(&db_url).await?;

    if let Err(e) = check_connection(&pool).await {
        eprintln!("database connection test failed: {}", e);
        return Err(e.into());
    }
    println!("database connection test succeeded");

    // マイグレーション実行
    if let Err(e) = migrate::run_migrations(&pool).await {
        eprintln!("failed to run migrations: {}", e);
        return Err(e.into());
    }
    println!("database migrations applied successfully");

    // ログ初期化（最低限）
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("start Logging successfully");

    // Wantedlyのプロフィールビュー生データをJSONファイルから読み込み、DBに挿入する例
    let dir_path = "local_data/profile_sources/wantedly/raw";

    let file_names = fs::read_dir(dir_path)?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect::<Vec<_>>();

    for file_name in file_names {
        let snapshot_at = filename_to_utc_from_jst(&file_name).ok_or("invalid filename format")?;
        let path = format!("{}/{}", dir_path, file_name);

        import_wantedly_profile_views_from_file(&pool, &path, snapshot_at).await?;
    }

    // ルータ定義
    let app: Router = routes::router();

    // アドレス
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on http://{}", addr);

    // 起動
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};

fn filename_to_utc_from_jst(filename: &str) -> Option<DateTime<Utc>> {
    let stem = filename.strip_suffix(".json")?;
    let naive_local = NaiveDateTime::parse_from_str(stem, "%Y%m%d%H%M%S").ok()?;

    let jst = FixedOffset::east_opt(9 * 3600)?;
    let jst_dt = jst.from_local_datetime(&naive_local).single()?;

    let utc_dt = jst_dt.with_timezone(&Utc);
    Some(utc_dt)
}
fn filename_to_utc(filename: &str) -> Option<DateTime<Utc>> {
    let stem = filename.strip_suffix(".json")?;
    let naive = NaiveDateTime::parse_from_str(stem, "%Y%m%d%H%M%S").ok()?;
    Some(Utc.from_utc_datetime(&naive))
}
