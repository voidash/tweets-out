use std::process::Command;
use std::io::*;
use std::fs;


pub fn initial_check() {
    if !match Command::new("ripdrag").arg("h").output() {
        Ok(_) => true,
        Err(_) => false,
    } {
        panic!("Didn't find Ripdrag, Install RipDrag first");
    }

    if !match Command::new("firefox").arg("-h").output() {
        Ok(_) => true,
        Err(_) => false,
    } {
        panic!("Other ungodly browsers are not supported");
    }
}

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
