use std::process::Command;

use sqlx::{Sqlite, Pool, sqlite::SqliteQueryResult, FromRow};

use anyhow::Result;

mod database; 
mod emoji;
mod utils;
mod posts;

use database::Database;
use posts::Post; 


#[tokio::main]
async fn main() -> Result<()> {
    let db = Database::new().await.0;

    let new_post = Post::interactive_input();


    if let Some(ref path) = new_post.image_path {
        Command::new("open")
        .arg(path)
        .spawn()
        .expect("Failed to start firefox");
    }

    Command::new("firefox")
        .args(["--url", "twitter.com", "--url", "linkedin.com"])
        .spawn()
        .expect("Failed to start firefox");

    println!("{:?}", new_post.insert_post(&db).await);

    println!("{:?}", Post::view_posts(&db).await);
    Ok(())
}
