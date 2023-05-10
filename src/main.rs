use serde_json::Value;
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
    let file_paths: Vec<walkdir::DirEntry> = WalkDir::new(folder)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".zst"))
        .collect();

    let path = Path::new("clean_de.jsonl");
    let clean_file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let mut writer = BufWriter::new(clean_file);

    for file in file_paths {
        let decoder = {
            let file = fs::File::open(file.path()).unwrap();
            zstd::Decoder::new(file).unwrap()
        };
        let reader = BufReader::new(decoder);
        for line in reader.lines() {
            let v: Value = serde_json::from_str(&line.unwrap()).unwrap();
            writeln!(writer, "{}\n", v["content"]).unwrap();
        }
    }
}
