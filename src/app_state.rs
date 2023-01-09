use sea_orm::{DatabaseConnection};

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection
}
