use oscar_io::v3::Document;
use std::{
    env,
    fs::{self},
    io::{BufRead, BufReader},
};
use walkdir::{DirEntry, WalkDir};
use words_count::WordsCount;

fn main() {
    let args: Vec<String> = env::args().collect();

    let folder = &args[1];
    let file_paths: Vec<DirEntry> = WalkDir::new(folder)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".zst"))
        .collect();

    let mut total_count = WordsCount {
        words: 0,
        characters: 0,
        cjk: 0,
        whitespaces: 0,
    };

    for file in file_paths {
        let decoder = {
            let file = fs::File::open(file.path()).unwrap();
            zstd::Decoder::new(file).unwrap()
        };
        let reader = BufReader::new(decoder);
        let mut file_count = WordsCount {
            words: 0,
            characters: 0,
            cjk: 0,
            whitespaces: 0,
        };
        for line in reader.lines() {
            let doc = serde_json::from_str::<Document>(&line.unwrap()).unwrap();
            let count = words_count::count(doc.content());
            file_count += count;
        }
        total_count += file_count;
    }
    println!("{:?}", total_count)
}
