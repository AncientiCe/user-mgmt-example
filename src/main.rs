use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use user_mgmt_api::{config::Config, create_user_manager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "auth_example=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Database connection
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost:5432/auth_db".to_string());

    let pool = PgPool::connect(&database_url).await?;

    // Auth configuration
    let auth_config = Config {
        jwt_secret: env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string()),
        jwt_expiry_hours: 24,
    };

    // Create the auth router using your library
    let app = create_user_manager(pool, Some(auth_config)).await?;

    // Start server
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("🚀 Auth API running on http://{}", addr);
    tracing::info!("Available endpoints:");
    tracing::info!("  POST /register");
    tracing::info!("  POST /login");
    tracing::info!("  GET  /me (requires Bearer token)");
    tracing::info!("  GET  /users/:id (requires Bearer token)");
    tracing::info!("  PATCH /users/:id (requires Bearer token)");

    axum::serve(listener, app).await?;

    Ok(())
}