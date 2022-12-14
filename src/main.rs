use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use glob::glob;

const BASE_PATH: &str = "compare";

fn main() {
    // regenerate diff files
    if true {
        std::fs::create_dir_all(BASE_PATH).unwrap();
        let reader = BufReader::new(File::open("git_diff").expect("Cannot open git_diff"));
        let mut current_file_path = Path::new("").to_owned();

        // separate diff file into smaller files based
        for line in reader.lines() {
            let line = line.unwrap();
            seperate_diff_into_individual_files(line, &mut current_file_path)
        }
    }

    // filter out trivial diffs
    let glob_path = format!("{}/*", BASE_PATH);
    for file_path in glob(&glob_path).unwrap().filter_map(Result::ok) {
        let is_trivial = is_trivial_diff(&file_path);
        println!(
            "------trivial: {} {}",
            is_trivial,
            file_path.as_path().display()
        );

        if is_trivial {
            std::fs::remove_file(file_path).unwrap();
        }
    }
}

fn is_trivial_diff(file_path: &Path) -> bool {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut is_trivial = true;

    for line in reader.lines() {
        if let Ok(line) = line.as_ref() {
            if !is_interesting_diff(file_path, line) {
                continue;
            }
        }

        is_trivial = false;
    }

    is_trivial
}

fn is_interesting_diff(file_path: &Path, line: &str) -> bool {
    let mut interesting = line.starts_with('-') || line.starts_with('+');

    // &'static core::panic::Location<'static>
    // &'static core::panic::location::Location<'static>
    interesting &= !(line.contains("panic::") && line.contains("::Location"));

    // impl std::panic::UnwindSafe for s2n_quic::provider::endpoint_limits::Outcome
    // impl core::panic::unwind_safe::UnwindSafe for s2n_quic::provider::endpoint_limits::Outcome
    interesting &= !(line.contains("panic::") && line.contains("s2n_quic::provider"));

    // impl !std::panic::RefUnwindSafe for s2n_quic::connection::Handle
    // impl !core::panic::unwind_safe::RefUnwindSafe for s2n_quic::connection::Handle
    interesting &= !(line.contains("panic::") && line.contains("s2n_quic::connection"));

    // impl !std::panic::UnwindSafe for s2n_quic::stream::BidirectionalStream
    // impl !core::panic::unwind_safe::UnwindSafe for s2n_quic::stream::BidirectionalStream
    interesting &= !(line.contains("panic::") && line.contains("s2n_quic::stream"));

    // These are reordered!!!
    // impl s2n_quic::provider::event::Event for s2n_quic::provider::event::events::PlatformTxError
    // impl core::panic::unwind_safe::RefUnwindSafe for s2n_quic::provider::event::events::PlatformTxError
    interesting &= !((line.contains("panic::")
        || line.contains("impl s2n_quic::provider::event::Event"))
        && line.contains("s2n_quic::provider::event"));

    // These are reordered!!!
    // impl<'a> s2n_quic::provider::event::Event for s2n_quic::provider::event::events::VersionInformation<'a>
    // impl<'a> core::panic::unwind_safe::RefUnwindSafe for s2n_quic::provider::event::events::VersionInformation<'a>
    interesting &= !((line.contains("panic::")
        || line.contains("impl<'a> s2n_quic::provider::event::Event"))
        && line.contains("s2n_quic::provider::event"));

    // These are reordered!!!
    // impl s2n_quic::provider::event::Meta for s2n_quic::provider::event::ConnectionMeta
    // impl core::panic::unwind_safe::RefUnwindSafe for s2n_quic::provider::event::ConnectionMeta
    interesting &= !((line.contains("panic::")
        || line.contains("s2n_quic::provider::event::Meta"))
        && line.contains("s2n_quic::provider::event"));

    // These are reordered!!!
    // impl s2n_quic::provider::event::Provider for s2n_quic::provider::event::tracing::Provider
    // impl core::panic::unwind_safe::RefUnwindSafe for s2n_quic::provider::event::tracing::Provider
    interesting &= !((line.contains("panic::")
        || line.contains("s2n_quic::provider::event::Provider"))
        && line.contains("s2n_quic::provider::event"));

    // impl std::panic::RefUnwindSafe for s2n_quic::provider::tls::rustls::rustls::CipherSuite
    // impl core::panic::unwind_safe::UnwindSafe for s2n_quic::provider::tls::rustls::rustls::CipherSuite
    interesting &= !(file_path
        .display()
        .to_string()
        .contains("s2n_quic::provider::tls::rustls::"));

    // impl std::panic::RefUnwindSafe for s2n_quic::provider::tls::s2n_tls::client::Builder
    // impl core::panic::unwind_safe::RefUnwindSafe for s2n_quic::provider::tls::s2n_tls::client::Builder
    interesting &= !(file_path
        .display()
        .to_string()
        .contains("s2n_quic::provider::tls::s2n_tls::client"));

    println!("panic:{} {} {}", !line.contains("panic"), interesting, line,);
    interesting
}

fn seperate_diff_into_individual_files(line: String, current_file_path: &mut PathBuf) {
    if line.starts_with("@@") {
        let write_file = line.split("@@").last().unwrap().trim().replace(' ', "_");
        println!("{}", write_file);

        let base_path = Path::new(BASE_PATH);
        *current_file_path = base_path.join(write_file);
        File::create(current_file_path.clone()).unwrap();
        return;
    }

    let mut file = File::options()
        .append(true)
        .open(current_file_path.clone())
        .unwrap();
    file.write_all(line.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
}
