use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut all_files: Vec<String> = Vec::new();

    if args.len() < 2 {
        eprintln!("Usage: {} <folder_path>", args[0]);
        std::process::exit(1);
    }

    let folder_path = &args[1];

    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        if entry_path.is_file() && entry_path.extension().map_or(false, |ext| ext == "js") {
            all_files.push(entry_path.display().to_string());
        }
    }

    let log_regex = Regex::new(r"console\.log\([^)]*\);?").unwrap();

    for file in &all_files {
        let content = fs::read_to_string(file).expect("Could not read file");
        let modified_content = log_regex.replace_all(&content, "");
        fs::write(file, modified_content.as_bytes()).expect("Could not write file");
    }

    println!("Processed JavaScript files in '{}':", folder_path);
    for file in all_files {
        println!("{}", file);
    }
}
