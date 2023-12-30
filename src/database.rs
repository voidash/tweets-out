use config::{Config, File};
use serde::{Serialize,Deserialize};
use sqlx::{Pool,Postgres};
use sqlx::postgres::PgPoolOptions;

const DEFAULT_CONFIGURATION : &str = "./db.toml";

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    db_url : String
}

pub struct Database(pub Pool<Postgres>);

impl Database {
    pub async fn new() -> Self {
        // if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        //     println!("Creating Database {}", DB_URL);
        //     match Sqlite::create_database(DB_URL).await {
        //         Ok(_) => {
        //             println!("Database Created Successfully")
        //         },
        //         Err(_) => todo!(),
        //     } 
        // } else {
        //         println!("Database Already Exists");
        // }
        
        let post_configuration = Config::builder()
                .add_source(File::with_name(DEFAULT_CONFIGURATION))
                .build().unwrap().try_deserialize::<Configuration>().unwrap();
        
        let db = PgPoolOptions::new().max_connections(3).connect(&post_configuration.db_url).await.unwrap();

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

