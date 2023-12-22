use crate::utils::emojify;

use serde::{Serialize,Deserialize};
use sqlx::{Sqlite, Pool, sqlite::SqliteQueryResult, FromRow};

use anyhow::Result;
use dialoguer::{Input, theme::ColorfulTheme, Select};
use chrono::{DateTime, Utc};
use std::fs;

use crate::utils::move_files;

#[derive(Clone, FromRow, Debug, Serialize, Deserialize, Default)]
pub struct Post {
    pub date: i64,
    pub title: Option<String>,
    pub description: String,
    pub image_path: Option<String>,
}

impl Post {
    pub async fn insert_post(&self,db: &Pool<Sqlite>) -> Result<SqliteQueryResult>{
       let result = sqlx::query("INSERT INTO Posts(date,title,description,image_path) VALUES ($1, $2, $3, $4)")
           .bind(&self.date)
           .bind(&self.title)
           .bind(&self.description)
           .bind(&self.image_path)
           .execute(db).await;

       Ok(result?)
    }

    pub async fn view_posts(db: &Pool<Sqlite>) -> Result<Vec<Self>> {
        let post_results = sqlx::query_as::<_,Post>("Select * from Posts")
                        .fetch_all(db)
                        .await?;

        Ok(post_results)
    }

    pub fn interactive_input() -> Self {
         // Get the current date and time
        let current_date: DateTime<Utc> = Utc::now();
        let current_date_i64 = current_date.timestamp();

        // Use RustboxTheme for styling
        let theme = ColorfulTheme::default();

        // Ask for input for each field
        let title: Option<String> = Input::<String>::with_theme(&theme)
            .with_prompt("Enter the title (optional)")
            .interact()
            .ok();

        let description: String = Input::<String>::with_theme(&theme)
            .with_prompt("Enter the description")
            .interact()
            .expect("Failed to get input for description");

        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let image_path = std::path::Path::new(&crate_dir).join("images").join(current_date_i64.to_string());


        fs::create_dir(&image_path).expect("Failed to create directory");

        let fetch_images = Select::with_theme(&theme)
            .with_prompt("Are we fetching Image?")
            .item("Yes")
            .item("No")
            .interact()
            .unwrap();

        if fetch_images == 0 {
        let image_to_fetch_from: String = Input::<String>::with_theme(&theme)
            .with_prompt("Path to fetch Image from")
            .with_initial_text(format!("{}/Pictures/buffer/", std::env::var("HOME").unwrap()).to_string())
            .validate_with(|input: &String| -> Result<(),&str> {
                if std::path::PathBuf::from(input).exists() {
                    return Ok(());
                }
                Err("Not a valid path")
            })
            .interact()
            .expect("Failed to get input for description");


            move_files(&image_to_fetch_from,&image_path.clone().into_os_string().into_string().unwrap()).unwrap();
        }


        // Return a new Post instance
        Post {
            date: current_date_i64,
            title: {if let Some(txt) = title 
                    {Some(emojify(&txt).unwrap())} 
                else 
                    {title}},
            description: emojify(&description).unwrap(),
            image_path: Some(image_path.into_os_string().into_string().unwrap())    
        }
    }
}

