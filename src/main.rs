use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let reader = BufReader::new(File::open("cmp").expect("Cannot open cmp"));

    let base = Path::new("compare");
    // let mut write_file: String = "a".to_string();
    let mut file_path = Path::new("").to_owned();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("@@") {
            let write_file = line.split("@@").last().unwrap().trim().replace(' ', "_");
            println!("{}", write_file);
            file_path = base.join(write_file);
            File::create(file_path.clone()).unwrap();
        }

        let mut file = File::options()
            .append(true)
            .open(file_path.clone())
            .unwrap();
        file.write_all(line.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}
