use dialoguer::Input;
use chrono::{Local, DateTime};
use std::process::Command;

struct Post{
    date: DateTime<Local>,
    description: String,
}

fn is_command_available(command: &str) -> bool{
    match Command::new(command).output() {
        Ok(_) => {true},
        Err(_) => {false}
    }
}

fn initial_check() {
    if !is_command_available("ripdrag") {
        panic!("Didn't find Ripdrag, Install RipDrag first");
    }
}

fn replace(mut s: &str, mut o: impl Write) -> io::Result<()> {
    // i..j gives ":rocket:"
    // m..n gives "rocket"
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
    o.write_all(s.as_bytes())
}

fn main() {
    println!("Hello, world!");
}
