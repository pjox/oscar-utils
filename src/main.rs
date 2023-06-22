use oscar_io::v3::Document;
use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};
use walkdir::{DirEntry, WalkDir};

fn main() {
    let args: Vec<String> = env::args().collect();

    let folder = &args[1];
    let file_paths: Vec<DirEntry> = WalkDir::new(folder)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".zst"))
        .collect();

    let path = Path::new(&args[2]);
    let clean_file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let mut writer = BufWriter::new(clean_file);

    let mut clean_count = 0;
    let mut tiny_count = 0;
    let mut short_sentences_count = 0;
    let mut header_count = 0;
    let mut footer_count = 0;
    let mut noisy_count = 0;

    for file in file_paths {
        let decoder = {
            let file = fs::File::open(file.path()).unwrap();
            zstd::Decoder::new(file).unwrap()
        };
        let reader = BufReader::new(decoder);
        for line in reader.lines() {
            let doc = serde_json::from_str::<Document>(&line.unwrap()).unwrap();
            if is_clean(&doc) && clean_count < 10 {
                writeln!(writer, "{}\t{}\tclean", doc.warc_id(), doc.url().unwrap()).unwrap();
                clean_count += 1;
                continue;
            }
            if is_tiny(&doc) && tiny_count < 10 {
                writeln!(writer, "{}\t{}\ttiny", doc.warc_id(), doc.url().unwrap()).unwrap();
                tiny_count += 1;
                continue;
            }
            if is_short_sentences(&doc) && short_sentences_count < 10 {
                writeln!(writer, "{}\t{}\ttiny", doc.warc_id(), doc.url().unwrap()).unwrap();
                short_sentences_count += 1;
                continue;
            }
            if is_header(&doc) && header_count < 10 {
                writeln!(writer, "{}\t{}\theader", doc.warc_id(), doc.url().unwrap()).unwrap();
                header_count += 1;
                continue;
            }
            if is_footer(&doc) && footer_count < 10 {
                writeln!(writer, "{}\t{}\tfooter", doc.warc_id(), doc.url().unwrap()).unwrap();
                footer_count += 1;
                continue;
            }
            if is_noisy(&doc) && noisy_count < 10 {
                writeln!(writer, "{}\t{}\tnoisy", doc.warc_id(), doc.url().unwrap()).unwrap();
                noisy_count += 1;
                continue;
            }
            if clean_count >= 10
                && tiny_count >= 10
                && short_sentences_count >= 10
                && header_count >= 10
                && footer_count >= 10
                && noisy_count >= 10
            {
                break;
            }
        }
    }
}

fn is_tiny(doc: &Document) -> bool {
    match doc.metadata().annotation() {
        Some(categories) => {
            if categories.contains(&"tiny".to_string()) {
                return true;
            }
        }
        None => return false,
    }
    false
}

fn is_short_sentences(doc: &Document) -> bool {
    match doc.metadata().annotation() {
        Some(categories) => {
            if categories.contains(&"short_sentences".to_string()) {
                return true;
            }
        }
        None => return false,
    }
    false
}

fn is_header(doc: &Document) -> bool {
    match doc.metadata().annotation() {
        Some(categories) => {
            if categories.contains(&"header".to_string()) {
                return true;
            }
        }
        None => return false,
    }
    false
}

fn is_footer(doc: &Document) -> bool {
    match doc.metadata().annotation() {
        Some(categories) => {
            if categories.contains(&"footer".to_string()) {
                return true;
            }
        }
        None => return false,
    }
    false
}

fn is_noisy(doc: &Document) -> bool {
    match doc.metadata().annotation() {
        Some(categories) => {
            if categories.contains(&"footer".to_string()) {
                return true;
            }
        }
        None => return false,
    }
    false
}

fn is_clean(doc: &Document) -> bool {
    if doc.metadata().annotation().is_some() {
        return false;
    }
    match doc.metadata().categories() {
        Some(categories) => {
            if categories.contains(&"adult".to_string())
                || categories.contains(&"agressif".to_string())
                || categories.contains(&"cryptojacking".to_string())
                || categories.contains(&"malware".to_string())
                || categories.contains(&"mixed_adult".to_string())
            {
                return false;
            }
        }
        None => (),
    }
    if doc.metadata().harmful_pp() < Some(8.0) {
        return false;
    }
    true
}
