use sqlx::{
    Pool,
    migrate::MigrateDatabase,
    sqlite::{Sqlite, SqlitePool},
};

const DB_DIR: &str = "db";
const DB_URL: &str = "sqlite:db/rchat.db";

fn create_database_dir() {
    println!("Creating database dir.");
    std::fs::create_dir_all(DB_DIR).map_err(|err| panic!("{err}"));
    println!("Creating database dir. [OK]");
}

async fn create_database() {
    println!("Create database.");
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create database. [OK]"),
            Err(err) => {
                println!("Create database. [ERROR]");
                panic!("{err}")
            }
        }
    }
}

async fn test_database_connection() -> Pool<Sqlite> {
    println!("Connect to database.");
    let pool: Pool<Sqlite> = match SqlitePool::connect(DB_URL).await {
        Ok(pool) => {
            println!("Connect to database. [OK]");
            pool
        }
        Err(err) => {
            println!("Connect to database. [ERROR]");
            panic!("{err}")
        }
    };

    match sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(&pool)
        .await
    {
        Ok(_) => return pool,
        Err(err) => {
            panic!("{err}")
        }
    }
}

async fn init_database(pool: &Pool<Sqlite>) {
    println!("initialize database.");

    println!("Run database migrations..");
    match sqlx::migrate!().run(pool).await {
        Ok(_) => println!("Run database migrations.. [OK]"),
        Err(err) => {
            println!("Run database migrations.. [ERROR]");
            panic!("{err}")
        }
    };
}

/// Create database dir, database and run migrations.
///
/// ## Panics
/// Panics if one of the initialization function fails
///
/// ## Result
/// Returns a SQLite database pool
pub async fn initialize_database() -> Pool<Sqlite> {
    create_database_dir();
    create_database().await;
    let pool = test_database_connection().await;
    init_database(&pool).await;

    pool
}
