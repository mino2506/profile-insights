use axum::Router;
use db::check_connection;
use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

use storage;

mod config;
mod error;
mod migrate;
mod routes;

mod infra;

#[tokio::main]
async fn main() {
    let json = infra::json_loader::load_json_file(
        "local_data/profile_sources/wantedly/raw/20251123132822.json",
    )
    .expect("failed to load JSON file");

    let node = json
        .get("data")
        .and_then(|d| d.get("profileImpressionPage"))
        .and_then(|pi| pi.get("impressedUsers"))
        .and_then(|iu| iu.get("edges"))
        .and_then(|e| e.as_array())
        .expect("invalid JSON structure: expected data.profileViews.nodes as array");

    if let Some(first) = node.first() {
        println!("{}", first);
    }

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
