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

macro_rules! unexpected {
    ($m:expr, $name:ident, $ok:expr) => {
        match $m {
            Ok($name) => $ok,
            Err(e) => panic!("unexpected error: {}", e),
        }
    };
}

macro_rules! mapadd {
    ($map:ident, $text_chunk:ident, $text:expr) => {
        $map.insert(serde_yaml::Value::String($text_chunk.keyword.clone()), parse($text))
    };
}

fn main() {
    let args = Args::parse();

    let reader = unexpected!(File::open(args.path), file, unexpected!(
        Decoder::new(file).read_info(), reader, reader
    ));

    let mut map = serde_yaml::Mapping::new();

    let info = &reader.info();

    for text_chunk in &info.uncompressed_latin1_text {
        mapadd!(map, text_chunk, text_chunk.text.clone());
    }

    for text_chunk in &info.compressed_latin1_text {
        unexpected!(text_chunk.get_text(), text, mapadd!(map, text_chunk, text));
    }

    for text_chunk in &info.utf8_text {
        unexpected!(text_chunk.get_text(), text, mapadd!(map, text_chunk, text));
    }

    let the_string = unexpected!(serde_yaml::to_string(&map), s, s);

    let mut clipboard = unexpected!(Clipboard::new(), c, c);

    unexpected!(clipboard.set_text(the_string), _s, return)
}

fn parse(text: String) -> serde_yaml::Value {
    match serde_yaml::from_str(&text) {
        Ok(s) => s,
        Err(_) => serde_yaml::Value::String(text),
    }
}
