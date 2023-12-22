use std::io::*;

/// Replaces the :emoji: with emojis
fn replace(mut s: &str, mut o: impl Write) -> Result<()> {
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
