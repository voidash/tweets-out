use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, Pool};

static DB_URL: &str = "sqlite://posts.db";

pub struct Database(pub Pool<Sqlite>);

impl Database {
    pub async fn new() -> Self {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            println!("Creating Database {}", DB_URL);
            match Sqlite::create_database(DB_URL).await {
                Ok(_) => {
                    println!("Database Created Successfully")
                },
                Err(_) => todo!(),
            } 
        } else {
                println!("Database Already Exists");
        }


        let db = SqlitePool::connect(DB_URL).await.unwrap();

        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let migrations = std::path::Path::new(&crate_dir).join("./migrations");

        let migrations_results = sqlx::migrate::Migrator::new(migrations)
            .await
            .unwrap()
            .run(&db)
            .await;

        if let Err(err) = migrations_results {
            panic!("Error Migrating : {}", err);
        }

        Self (db)
    }
}

