use std::process::Command;
use arboard::Clipboard;

use std::fs;
use anyhow::Result;

mod database; 
mod utils;
mod posts;
use config::{Config,File};

use database::Database;
use posts::Post;

use crate::utils::move_files; 

const CONFIG_FILE_PATH : &str = "./Post.toml";


#[tokio::main]
async fn main() -> Result<()> {
    let db = Database::new().await.0;

    let post_configuration = Config::builder()
                .add_source(File::with_name(CONFIG_FILE_PATH))
                .build();


    let mut new_post = Post::default();

    match post_configuration {
        Ok(conf)  => {
            if let Ok(mut conf) = conf.try_deserialize::<Post>() {
                conf.date = chrono::Utc::now().timestamp();

            if let Some(ref path) = conf.image_path {
                let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
                let image_path = std::path::Path::new(&crate_dir).join("images").join(conf.date.to_string());


                fs::create_dir(&image_path).expect("Failed to create directory");
                move_files(path,&image_path.clone().into_os_string().into_string().unwrap()).unwrap();

                conf.image_path = Some(image_path.into_os_string().into_string().unwrap());
            }
            new_post = conf;
            }
        },
        Err(_) => {
            new_post = Post::interactive_input();
        }
    }


    if let Some(ref path) = new_post.image_path {
        Command::new("open")
        .arg(path)
        .spawn()
        .expect("Error opening the file explorer");
    }

    Command::new("firefox")
        .args(["--url", "twitter.com", "--url", "linkedin.com"])
        .spawn()
        .expect("Failed to start firefox");



    println!("{:?}", new_post.insert_post(&db).await);

    let mut clipboard = Clipboard::new().unwrap();
    match new_post.title {
        Some(txt) => clipboard.set_text(format!("{} \n {}",txt, new_post.description )).unwrap(),
        None => clipboard.set_text(format!("{}",new_post.description)).unwrap()
    }


    println!("{:?}", Post::view_posts(&db).await);
    Ok(())
}
