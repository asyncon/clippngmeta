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
    let args = Args::parse();

    let reader = match File::open(args.path) {
        Ok(file) => match Decoder::new(file).read_info() {
            Ok(reader) => reader,
            Err(e) => panic!("unexpected error: {}", e),
        },
        Err(e) => panic!("unexpected error: {}", e),
    };

    let mut map = serde_yaml::Mapping::new();

    let info = &reader.info();

    for text_chunk in &info.uncompressed_latin1_text {
        map.insert(
            serde_yaml::Value::String(text_chunk.keyword.clone()), 
            parse(text_chunk.text.clone())
        );
    }

    for text_chunk in &info.compressed_latin1_text {
        match text_chunk.get_text() {
            Ok(text) => map.insert(serde_yaml::Value::String(text_chunk.keyword.clone()), parse(text)),
            Err(e) => panic!("unexpected error: {}", e),
        };
    }

    for text_chunk in &info.utf8_text {
        match text_chunk.get_text() {
            Ok(text) => map.insert(serde_yaml::Value::String(text_chunk.keyword.clone()), parse(text)),
            Err(e) => panic!("unexpected error: {}", e),
        };
    }

    let the_string = match serde_yaml::to_string(&map) {
        Ok(s) => s,
        Err(e) => panic!("unexpected error: {}", e),
    };

    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => clipboard,
        Err(e) => panic!("unexpected error: {}", e),
    };

    match clipboard.set_text(the_string) {
        Ok(_) => return,
        Err(e) => panic!("unexpected error: {}", e),
    }
}

fn parse(text: String) -> serde_yaml::Value {
    match serde_yaml::from_str(&text) {
        Ok(s) => s,
        Err(_) => serde_yaml::Value::String(text),
    }
}
