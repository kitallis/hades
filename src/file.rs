use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;

pub fn write(path: &Path, contents: &[u8]) {
    let display = path.display();

    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't create {}: {}", display, why),
    };

    match file.write_all(contents) {
        Ok(_) => println!("Wrote: {}", display),
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
    }
}
