use std::path::PathBuf;
use std::process::Command;
use std::io::*;
use std::fs;

use arboard::Clipboard;

use crate::posts::Post;


pub fn move_files(source_folder: &str, destination_folder: &str) -> std::io::Result<()> {
    // Read the contents of the source folder
    let entries = fs::read_dir(source_folder)?;

    // Iterate over the entries in the source folder
    for entry in entries {
        let entry = entry?;
        let source_path = entry.path();

        // Create the destination path by appending the file name to the destination folder
        let destination_path = format!("{}/{}", destination_folder, source_path.file_name().unwrap().to_string_lossy());

        // Attempt to move the file to the destination
        fs::rename(source_path, destination_path)?;
    }
    Ok(())
}

/// Replaces the :emoji: with emojis
pub fn emojify(mut s: &str) -> Result<String> {
    // i..j gives ":rocket:"
    // m..n gives "rocket"
    let mut o = Vec::new();
    while let Some((i, m, n, j)) = s
        .find(':')
        .map(|i| (i, i + 1))
        .and_then(|(i, m)| s[m..].find(':').map(|x| (i, m, m + x, m + x + 1)))
    {
        match emojis::get_by_shortcode(&s[m..n]) {
            Some(emoji) => {
                o.write_all(s[..i].as_bytes())?;
                o.write_all(emoji.as_bytes())?;
                s = &s[j..];
            }
            None => {
                o.write_all(s[..n].as_bytes())?;
                s = &s[n..];
            }
        }
    }
    o.write_all(s.as_bytes()).unwrap();

    Ok(std::str::from_utf8(&o).unwrap().to_string())
}

pub async fn post_automation(new_post: &Post) {
    if let Some(ref path) = new_post.image_path {
        Command::new("open")
        .arg(path)
        .spawn()
        .expect("Error opening the file explorer");
    }

    Command::new("firefox")
        .args(["--url", "twitter.com/compose/tweet", "--url", "linkedin.com"])
        .spawn()
        .expect("Failed to start firefox");




    let mut clipboard = Clipboard::new().unwrap();
    match &new_post.title {
        Some(txt) => clipboard.set_text(format!("{} \n {} \n #{}",txt, new_post.description,new_post.date )).unwrap(),
        None => clipboard.set_text(format!("{} \n #{}",new_post.description, new_post.date)).unwrap()
    }

}

pub fn handle_image(image_name: &str,fetch_image: Option<&str>) -> PathBuf{
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let image_path = std::path::Path::new(&crate_dir).join("images").join(image_name);
    fs::create_dir(&image_path).expect("Failed to create directory");

    if let Some(image_to_fetch_from) = fetch_image {
        move_files(&image_to_fetch_from,&image_path.clone().into_os_string().into_string().unwrap()).unwrap();
    }

    image_path
}


