use std::fs::File;
use arboard::Clipboard;
use clap::Parser;
use png::Decoder;

/// Copy image metadata to clipboard
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the image file
    path: String,
}

fn main() {
    match run() {
        None => return,
        Some(e) => panic!("unexpected error: {:?}", e),
    }
}

fn run() -> Option<(std::io::Error, serde_yaml::Error, png::DecodingError, arboard::Error)> {
    let args = Args::parse();

    let reader = Decoder::new(File::open(args.path).ok()?).read_info().ok()?;

    let mut map = serde_yaml::Mapping::new();

    let info = &reader.info();

    for text_chunk in &info.uncompressed_latin1_text {
        map.insert(serde_yaml::Value::String(text_chunk.keyword.clone()), parse(text_chunk.text.clone()));
    }

    for text_chunk in &info.compressed_latin1_text {
        map.insert(serde_yaml::Value::String(text_chunk.keyword.clone()), parse(text_chunk.get_text().ok()?));
    }

    for text_chunk in &info.utf8_text {
        map.insert(serde_yaml::Value::String(text_chunk.keyword.clone()), parse(text_chunk.get_text().ok()?));
    }

    Clipboard::new().ok()?.set_text(serde_yaml::to_string(&map).ok()?).ok()?;

    return None
}

fn parse(text: String) -> serde_yaml::Value {
    match serde_yaml::from_str(&text) {
        Ok(s) => s,
        Err(_) => serde_yaml::Value::String(text),
    }
}
