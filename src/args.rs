use clap::{Parser,Subcommand};
use std::path::PathBuf;

#[derive(Debug,Parser)]
#[command(name = "Tweets out")]
#[command(name = "Ashish Thapa <ashish.thapa477@gmail.com>")]
#[command(about = "personal knowledge management system")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(short, long)]
    pub list_unposted: bool,
    #[arg(short, long)]
    pub dump_json: bool,
    #[arg(short('a'),long, conflicts_with = "list_unposted")]
    pub list_all: bool,
    #[arg(short,long, conflicts_with_all = ["list_unposted", "list_all"])]
    pub post_unposted: bool,
}

#[derive(Debug,Subcommand)]
pub enum Commands {
    Post {
        #[arg(short,long)]
        title: Option<String>,
        #[arg(short,long)]
        description: String,
        #[arg(short,long)]
        post_now: bool,
    },
    Interactive{
    },
    File {
        #[arg(short,long)]
        path: Option<PathBuf>
    }
}
