use axum::Router;
use db::check_connection;
use dotenvy::dotenv;
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
    let path = "local_data/profile_sources/wantedly/raw/20251123132822.json";

    import_wantedly_profile_views_from_file(&pool, path, chrono::Utc::now()).await?;

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
