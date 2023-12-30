use crate::utils::{emojify, handle_image};

use serde::{Serialize,Deserialize};
use sqlx::{ Pool,FromRow, Postgres, postgres::PgQueryResult};

use anyhow::Result;
use dialoguer::{Input, theme::ColorfulTheme, Select};
use chrono::{DateTime, Utc};


#[derive(Clone, FromRow, Debug, Serialize, Deserialize, Default)]
pub struct Post {
    pub date: i64,
    pub title: Option<String>,
    pub posted: bool,
    pub description: String,
    pub image_path: Option<String>,
}

impl Post {
    pub fn to_array(&self) -> Vec<String> {
        return vec![
            DateTime::from_timestamp(self.date, 0).unwrap().to_string(),
            format!("{:?}", self.title),
            format!("{}", self.posted),
            format!("{}", self.description),
            format!("{:?}", self.image_path),
        ]; 
    }

    pub async fn delete_post(db: &Pool<Postgres>, date: i64) -> Result<PgQueryResult>{
       let result = sqlx::query("DELETE FROM Posts WHERE date = $1")
           .bind(date)
           .execute(db).await;

       Ok(result?)
    }

    pub async fn insert_post(&self,db: &Pool<Postgres>) -> Result<PgQueryResult>{
       let result = sqlx::query("INSERT INTO Posts(date,title,posted,description,image_path) VALUES ($1, $2, $3, $4, $5)")
           .bind(&self.date)
           .bind(&self.title)
           .bind(&self.posted)
           .bind(&self.description)
           .bind(&self.image_path)
           .execute(db).await;

       Ok(result?)
    }
    

    pub async fn update_post(db: &Pool<Postgres>,date: i64) -> Result<Vec<Self>> {
        let post_results = sqlx::query_as::<_,Post>("UPDATE Posts SET posted = true WHERE date = $1")
                        .bind(date)
                        .fetch_all(db)
                        .await?;

        Ok(post_results)
    }

    pub async fn view_posts(db: &Pool<Postgres>) -> Result<Vec<Self>> {
        let post_results = sqlx::query_as::<_,Post>("Select * from Posts")
                        .fetch_all(db)
                        .await?;

        Ok(post_results)
    }

    pub async fn view_unposted_posts(db: &Pool<Postgres>) -> Result<Vec<Self>> {
        let post_results = sqlx::query_as::<_,Post>("Select * from Posts where posted=false")
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

        let post_now: bool = Input::<bool>::with_theme(&theme)
            .with_prompt("Is this being posted Right away?")
            .interact()
            .expect("Failed to get input for description");

        let title: Option<String> = Input::<String>::with_theme(&theme)
            .with_prompt("Enter the title (optional)")
            .interact()
            .ok();


        let description: String = Input::<String>::with_theme(&theme)
            .with_prompt("Enter the description")
            .interact()
            .expect("Failed to get input for description");

        

        let fetch_images = Select::with_theme(&theme)
            .with_prompt("Are we fetching Image?")
            .item("Yes")
            .item("No")
            .interact()
            .unwrap();

        let image_path = if fetch_images == 0 {
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
                Some(handle_image(&format!("{}",current_date_i64), Some(&image_to_fetch_from)).to_str().unwrap().to_string())
            } 
            else {None};


        // Return a new Post instance
        Post {
            date: current_date_i64,
            title: {if let Some(txt) = title 
                    {Some(emojify(&txt).unwrap())} 
                else 
                    {title}},
            posted: post_now,
            description: emojify(&description).unwrap(),
            image_path
        }
    }
}

