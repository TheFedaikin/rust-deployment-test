use axum::{extract::State, routing::get, Json, Router};
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, Database, DatabaseConnection, EntityTrait, Schema, Set,
};
use serde_json::Value;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod entity;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = Database::connect("sqlite::memory:").await?;

    let schema = Schema::new(db.get_database_backend());
    let stmt = schema.create_table_from_entity(entity::Entity);
    db.execute(&stmt).await?;

    tracing::info!("created items table");

    let app = Router::new()
        .route("/", get(hello))
        .route("/health", get(health))
        .route("/items", get(list_items).post(create_item))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn health() -> &'static str {
    "ok"
}

async fn list_items(State(db): State<DatabaseConnection>) -> Json<Value> {
    let items = entity::Entity::find().all(&db).await.unwrap_or_default();
    Json(serde_json::to_value(items).unwrap())
}

async fn create_item(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unnamed");
    let item = entity::ActiveModel {
        name: Set(name.to_string()),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };
    let result = item.insert(&db).await.unwrap();
    Json(serde_json::to_value(result).unwrap())
}
