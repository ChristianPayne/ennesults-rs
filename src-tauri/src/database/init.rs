use crate::database::{connect_to_database, DatabaseError};
use surrealdb::{engine::local::Db, Surreal};
use tauri::AppHandle;

#[tauri::command]
pub async fn create_database(app_handle: AppHandle) -> Result<(), DatabaseError> {
    println!("🔄 Starting database creation...");

    let db = connect_to_database(&app_handle).await?;
    println!("✅ Database connection established");

    // Test the connection by running a simple query
    match db.query("INFO FOR DB").await {
        Ok(_) => println!("✅ Database connection verified"),
        Err(e) => {
            println!("❌ Database connection test failed: {:?}", e);
            return Err(DatabaseError::SurrealError(e));
        }
    }

    define_announcements_table(&db).await?;
    define_comebacks_table(&db).await?;
    define_insults_table(&db).await?;
    define_users_table(&db).await?;

    println!("🎉 All tables created successfully!");
    Ok(())
}

pub async fn define_announcements_table(db: &Surreal<Db>) -> Result<(), DatabaseError> {
    println!("🔄 Creating announcements table...");

    let _result = db
        .query(
            "DEFINE TABLE announcements TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
            DEFINE FIELD `value` ON announcements TYPE string PERMISSIONS FULL;
            DEFINE FIELD id ON announcements TYPE string PERMISSIONS FULL;",
        )
        .await
        .map_err(|e| {
            println!("❌ Failed to create announcements table: {:?}", e);
            DatabaseError::SurrealError(e)
        })?;

    // Check if the table was actually created
    match db.query("INFO FOR TABLE announcements").await {
        Ok(_) => println!("✅ Announcements table verified"),
        Err(e) => {
            println!("❌ Announcements table verification failed: {:?}", e);
            return Err(DatabaseError::SurrealError(e));
        }
    }

    Ok(())
}

pub async fn define_comebacks_table(db: &Surreal<Db>) -> Result<(), DatabaseError> {
    println!("🔄 Creating comebacks table...");

    let _result = db
        .query(
            "DEFINE TABLE comebacks TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
            DEFINE FIELD `value` ON comebacks TYPE string PERMISSIONS FULL;
            DEFINE FIELD id ON comebacks TYPE string PERMISSIONS FULL;",
        )
        .await
        .map_err(|e| {
            println!("❌ Failed to create comebacks table: {:?}", e);
            DatabaseError::SurrealError(e)
        })?;

    // Check if the table was actually created
    match db.query("INFO FOR TABLE comebacks").await {
        Ok(_) => println!("✅ Comebacks table verified"),
        Err(e) => {
            println!("❌ Comebacks table verification failed: {:?}", e);
            return Err(DatabaseError::SurrealError(e));
        }
    }

    Ok(())
}

pub async fn define_insults_table(db: &Surreal<Db>) -> Result<(), DatabaseError> {
    println!("🔄 Creating insults table...");

    let _result = db
        .query(
            "DEFINE TABLE insults TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
            DEFINE FIELD `value` ON insults TYPE string PERMISSIONS FULL;
            DEFINE FIELD id ON insults TYPE string PERMISSIONS FULL;
            DEFINE FIELD tags ON insults TYPE array<string> PERMISSIONS FULL;
            DEFINE FIELD tags[*] ON insults TYPE string PERMISSIONS FULL;",
        )
        .await
        .map_err(|e| {
            println!("❌ Failed to create insults table: {:?}", e);
            DatabaseError::SurrealError(e)
        })?;

    // Check if the table was actually created
    match db.query("INFO FOR TABLE insults").await {
        Ok(_) => println!("✅ Insults table verified"),
        Err(e) => {
            println!("❌ Insults table verification failed: {:?}", e);
            return Err(DatabaseError::SurrealError(e));
        }
    }

    Ok(())
}

pub async fn define_users_table(db: &Surreal<Db>) -> Result<(), DatabaseError> {
    println!("🔄 Creating users table...");

    let _result = db
        .query(
            "DEFINE TABLE users TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
            DEFINE FIELD consented ON users TYPE bool DEFAULT false PERMISSIONS FULL;
            DEFINE FIELD id ON users TYPE string PERMISSIONS FULL;
            DEFINE FIELD last_seen ON users TYPE datetime PERMISSIONS FULL;
            DEFINE FIELD lurk ON users TYPE bool DEFAULT false PERMISSIONS FULL;
            DEFINE FIELD username ON users TYPE string PERMISSIONS FULL;",
        )
        .await
        .map_err(|e| {
            println!("❌ Failed to create users table: {:?}", e);
            DatabaseError::SurrealError(e)
        })?;

    // Check if the table was actually created
    match db.query("INFO FOR TABLE users").await {
        Ok(_) => println!("✅ Users table verified"),
        Err(e) => {
            println!("❌ Users table verification failed: {:?}", e);
            return Err(DatabaseError::SurrealError(e));
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn check_tables(app_handle: AppHandle) -> Result<String, DatabaseError> {
    println!("🔍 Checking existing tables...");

    let db = connect_to_database(&app_handle).await?;

    let mut result = String::new();
    result.push_str("Database Tables:\n");

    // Check each table
    let tables = ["announcements", "comebacks", "insults", "users"];

    for table in tables {
        match db.query(format!("INFO FOR TABLE {}", table)).await {
            Ok(_) => {
                result.push_str(&format!("✅ {} - EXISTS\n", table));
                println!("✅ Table {} exists", table);
            }
            Err(e) => {
                result.push_str(&format!("❌ {} - NOT FOUND ({:?})\n", table, e));
                println!("❌ Table {} not found: {:?}", table, e);
            }
        }
    }

    // Also check what tables actually exist
    match db.query("INFO FOR DB").await {
        Ok(response) => {
            result.push_str("\nDatabase info response received\n");
            // Just indicate we got a response without trying to parse it
        }
        Err(e) => {
            result.push_str(&format!("\nFailed to get database info: {:?}\n", e));
        }
    }

    Ok(result)
}
