use std::{io::{Seek, Write}, os::unix::fs::MetadataExt};

const MB: u64 = 1_000_000;

pub fn log_to_file(msg: String) -> usize {
    let path = std::path::Path::new("logging_file.log");
    let mut file = std::fs::File::options().append(true).create(true).open(path).expect("open log file");
    if file.metadata().expect("metadata").size() > 512*MB {
        file.set_len(0).unwrap();
        file.rewind().unwrap();
    }
    writeln!(file, "{}", &msg).unwrap();
    return msg.len() + 1
}

