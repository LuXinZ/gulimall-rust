use sea_orm::DatabaseConnection;

pub mod router;
pub mod service;
pub mod entity;
#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
