use clap::Parser;

use anyhow::Result;

mod database; 
mod utils;
mod posts;
mod table;
pub mod args;
use config::{Config,File};

use database::Database;
use posts::Post;

use crate::{utils::{ post_automation, handle_image, emojify}, table::draw_table}; 


const POST_CONFIG_FILE_PATH : &str = "./Post.toml";


#[tokio::main]
async fn main() -> Result<()> {
    let cli = args::Cli::parse();

    println!("{:?}",cli);
    

    // check   
    let new_post: Option<Post> = match cli.command{
        Some(command_type) => {
            match command_type {
                args::Commands::Post { title, description, post_now } => {
                    let post = Post{
                        title,
                        description: emojify(&description).unwrap(),
                        posted: post_now,
                        ..Default::default()
                    };

                    Some(post)
                },
                args::Commands::Interactive {} => { Some(Post::interactive_input()) },
                args::Commands::File { path } => {
                    
                    let path_for_config = match path {
                            Some(v) => v.into_os_string().into_string().unwrap().clone(), 
                            None => String::from(POST_CONFIG_FILE_PATH) 
                    };
                    println!("{}", path_for_config);

                    let post_configuration = Config::builder()
                                .add_source(File::with_name(&path_for_config))
                                .build();

                    let mut new_post = Post::default();
                    // read from file
                    match post_configuration {
                        Ok(conf)  => {
                            if let Ok(mut conf) = conf.try_deserialize::<Post>() {
                            conf.date = chrono::Utc::now().timestamp();
                            conf.description = emojify(&conf.description).unwrap();

                            if let Some(ref path) = conf.image_path {
                                conf.image_path = Some(handle_image(&format!("{}",conf.date), Some(path)).to_str().unwrap().to_string());
                            }
                            new_post = conf;
                            }

                            Some(new_post)
                        },
                        Err(_) => {
                            panic!("Provide the file or create Post.toml in same directory as the executable");
                        }

                    }
                },
            }
        }, 

        None=>{None},
    };


    let db = Database::new().await.0;
    match new_post {
        Some(new_post) =>  {
                    if new_post.posted == true {
                        post_automation(&new_post).await;
                    } 

                    new_post.insert_post(&db).await?;

        }

        None => {
            // check if users want to view all posts
            if cli.list_all {
                println!("{:#?}", Post::view_posts(&db).await?);
            }else if cli.list_unposted {
                println!("{:#?}", Post::view_unposted_posts(&db).await?);
            }

            if cli.post_unposted {
                let unposted_posts = Post::view_unposted_posts(&db).await?;
                let (selected_state, mode) = draw_table(&unposted_posts).unwrap();

               let selected_post = &unposted_posts[selected_state.selected().unwrap()];
                match mode {

                    table::Mode::Select => {
                       Post::update_post(&db, selected_post.date).await?;
                       post_automation(&selected_post).await;
                    },

                    table::Mode::Delete => {
                       Post::delete_post(&db, selected_post.date).await?;
                    },

                    table::Mode::Default => {},
                }
            }
        }
    } 

    Ok(())
}
