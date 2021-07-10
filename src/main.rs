mod banner;

use instant::Instant;
use lazy_static::lazy_static;
use md5::Digest;
use rayon::prelude::*;
use std::env;
use std::process;

use indicatif::ProgressBar;

/*
    example hash(MD5):
    e890f806dfd189052ca7b39ac29da142 -> PASSWORD1
*/

lazy_static! {
    static ref HASH_INPUT: String = env::args().nth(2).unwrap();
}

static mut LOAD_TIME: u128 = 0;

fn crack(line: String, now: std::time::Instant) {
    let hashvalue = md5::Md5::digest(line.as_bytes());
    let formatted_hash = format!("{:x}", hashvalue);

    if formatted_hash == *HASH_INPUT {
        unsafe {
            println!(
                "🤍 Cracked! {} -> \"{}\" in {} millisecs",
                formatted_hash,
                line,
                now.elapsed().as_millis() - LOAD_TIME
            );
        }
        process::exit(0);
    }
}

fn main() -> std::io::Result<()> {
    banner::display_banner();

    let mut hash_dict: Vec<String> = vec![];

    // TODO: use a real fucking argument parser.
    let args: Vec<String> = env::args().collect();
    let wordlist_file = &args[1];

    println!("📋 Wordlist: {}", wordlist_file);

    // TODO: check if the hash is already cracked in the local crack repository and also in the online repository.
    // local repository: ~/.ruo/ruo.saved

    let mut reader = my_reader::BufReader::open(wordlist_file)?;
    let mut buffer = String::new();

    let now = Instant::now();

    while let Some(line) = reader.read_line(&mut buffer) {
        hash_dict.push(line?.trim_end().to_string());
    }

    unsafe {
        LOAD_TIME = now.elapsed().as_millis();
        println!("loaded the wordlist file in {} millisecs.", LOAD_TIME);
    }

    let pb = ProgressBar::new(hash_dict.len() as u64);

    for _ in 0..hash_dict.len() {
        hash_dict.par_iter().for_each(|lines| {
            let line = lines.clone();
            crack(line, now);
        });
        pb.inc(1);
    }
    pb.finish_with_message("Done!");

    Ok(())
}

// reusing the same buffer for each String
mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}
