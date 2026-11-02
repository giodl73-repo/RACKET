use std::io::{self, Read};
fn main() {
    let frames = std::env::args().nth(1).unwrap_or_else(|| "3".into());
    let mut text = String::new();
    if let Err(error) = io::stdin()
        .take((racket_workbench::MAX_BYTES + 1) as u64)
        .read_to_string(&mut text)
    {
        eprintln!("UTF-8 input: {error}");
        std::process::exit(2);
    }
    println!("{}", racket_workbench::inspect_frames(&text, &frames));
}
