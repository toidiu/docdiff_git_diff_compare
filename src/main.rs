use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

fn main() {
    let reader = BufReader::new(File::open("git_diff").expect("Cannot open git_diff"));

    let base_path = Path::new("compare");

    let mut current_file_path = Path::new("").to_owned();

    // seperate diff file into smaller files based
    for line in reader.lines() {
        let line = line.unwrap();
        seperate_diff_into_individual_files(line, base_path, &mut current_file_path)
    }
}

fn seperate_diff_into_individual_files(
    line: String,
    base_path: &Path,
    current_file_path: &mut PathBuf,
) {
    if line.starts_with("@@") {
        let write_file = line.split("@@").last().unwrap().trim().replace(' ', "_");
        println!("{}", write_file);
        *current_file_path = base_path.join(write_file);
        File::create(current_file_path.clone()).unwrap();
    }

    let mut file = File::options()
        .append(true)
        .open(current_file_path.clone())
        .unwrap();
    file.write_all(line.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
}
