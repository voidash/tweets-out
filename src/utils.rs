use std::process::Command;
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
