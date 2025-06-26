use serde::Serialize;
use surrealdb::{
    engine::local::{Db, RocksDb},
    opt::auth::Root,
    Surreal,
};
use tauri::{AppHandle, Manager};

pub mod init;

#[derive(Debug, Serialize)]
pub enum DatabaseError {
    ConnectionError(String),
    SurrealError(surrealdb::Error),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}

pub async fn connect_to_database(
    app_handle: &AppHandle,
) -> Result<surrealdb::Surreal<Db>, DatabaseError> {
    let resource_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

    let full_path = format!(
        "{}/{}",
        resource_path.to_str().expect("Can't convert to str"),
        "database"
    );

    // Create database connection using RocksDB
    let db = Surreal::new::<RocksDb>(full_path).await.map_err(|e| {
        println!("❌ Failed to create database connection: {:?}", e);
        DatabaseError::SurrealError(e)
    })?;

    // Set namespace and database
    match db.use_ns("ennesults").use_db("dev").await {
        Ok(_) => (),
        Err(e) => {
            println!("❌ Failed to set namespace/database: {:?}", e);
            return Err(DatabaseError::SurrealError(e));
        }
    }

    // Sign in
    match db
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
    {
        Ok(_) => (),
        Err(e) => {
            println!("❌ Database signin failed: {:?}", e);
            return Err(DatabaseError::SurrealError(e));
        }
    }

    Ok(db)
}
