mod banner;

// use instant::Instant;
use lazy_static::lazy_static;
use md5::Digest;
use std::env;
use std::process;
use std::sync::Mutex;
use std::thread;

/*
    example hash(MD5):
    e890f806dfd189052ca7b39ac29da142 -> PASSWORD1
*/

// fn crack(wordlist_string: &mut String) -> String {
//     // let wordlist_string = wordlist_string.unwrap().clone();
//     let hashvalue = md5::Md5::digest(wordlist_string.as_bytes());
//     format!("{:x}", hashvalue)
// }

lazy_static! {
    static ref HASH_DICT: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref HASH_INPUT: String = env::args().nth(2).unwrap();
}

fn crack() {
    // parallization
    HASH_DICT.lock().iter().for_each(|lines| {
        for line in lines.iter() {
            thread::spawn(move || {
                let wordlist_string = line.trim();
                let hashvalue = md5::Md5::digest(wordlist_string.as_bytes());
                let formatted_hash = format!("{:x}", hashvalue);

                if formatted_hash == *HASH_INPUT {
                    println!("🤍 Cracked! {} -> {}", formatted_hash, wordlist_string);
                    process::exit(0);
                }
            });
        }
    });
}

fn main() -> std::io::Result<()> {
    banner::display_banner();

    // let now = Instant::now();

    // TODO: use a real fucking argument parser.
    let args: Vec<String> = env::args().collect();
    let wordlist_file = &args[1];

    // println!("📋 Wordlist: {}", wordlist_file);

    // TODO: check if the hash is already cracked in the local crack repository and also in the online repository.
    // local repository: ~/.ruo/ruo.saved

    let mut reader = my_reader::BufReader::open(wordlist_file)?;
    let mut buffer = String::new();

    while let Some(line) = reader.read_line(&mut buffer) {
        // HASH_DICT.lock().unwrap().push(line?.trim().to_string());
    }
    println!("loaded the wordlist file.");

    crack();

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
