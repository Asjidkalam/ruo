mod algorithms;
mod banner;

use instant::Instant;
use lazy_static::lazy_static;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process;

lazy_static! {
    static ref HASH_INPUT: String = env::args().nth(2).unwrap();
    static ref LOCAL_HASH_PATH: String =
        format!("{}/.ruo/hashes.saved", env::home_dir().unwrap().display());
}

/*
    ruo currently supports: MD5, SHA1, RipeMD320, SHA256 and SHA512
*/

static mut LOAD_TIME: u128 = 0;

#[derive(Serialize, Deserialize)]
struct LocalHash {
    hash: String,
    plaintext: String,
}

fn crack(line: String, hash_len: usize, now: std::time::Instant) {
    let formatted_hash: String = algorithms::create_hash(&line, hash_len);

    if formatted_hash == *HASH_INPUT {
        unsafe {
            println!(
                "🤍 Cracked! {} -> \"{}\" in {} millisecs",
                formatted_hash,
                line,
                now.elapsed().as_millis() - LOAD_TIME
            );
        }

        // save the hash locally
        let mut local_hive: Vec<LocalHash> = Vec::new();
        let new_hash = LocalHash {
            hash: formatted_hash,
            plaintext: line,
        };
        local_hive.push(new_hash);
        let json = serde_json::to_string(&local_hive).unwrap();

        fs::write(&*LOCAL_HASH_PATH, &json).expect("Unable to write locally.");

        process::exit(0);
    }
}

fn main() -> std::io::Result<()> {
    banner::display_banner();

    let mut hash_dict: Vec<String> = vec![];

    // TODO: use a real fucking argument parser.
    let args: Vec<String> = env::args().collect();
    let wordlist_file = &args[1];

    // println!("📋 Wordlist: {}", wordlist_file);

    // check for saved hashes locally
    // BUGS: file is overwriting, not appending.
    // TODO: read the entire local file to memory before writing.
    let f = fs::File::open(&*LOCAL_HASH_PATH);
    let _ = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match fs::File::create(&*LOCAL_HASH_PATH) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => panic!("Unexpected error!"),
        },
    };

    let json_file_path = Path::new(&*LOCAL_HASH_PATH);

    let data = fs::read_to_string(json_file_path).unwrap();

    let mut local_hive: Vec<LocalHash> = Vec::new();
    if fs::metadata(json_file_path).unwrap().len() != 0 {
        local_hive = serde_json::from_str(&data)?;
    }

    for hash_object in local_hive {
        // check if the current hash matches to the hash in the local_hive.
        if hash_object.hash == *HASH_INPUT {
            println!(
                "🤍 Saved hash found! {} -> \"{}\"",
                hash_object.hash, hash_object.plaintext
            );
        }
    }

    // sanity check
    let valid_lens = vec![32, 40, 80, 64, 128];
    let hash_len = HASH_INPUT.len();
    if !valid_lens.contains(&hash_len) {
        println!("❌ Invalid hash length!");
        process::exit(1);
    }

    for alg_len in valid_lens.iter() {
        if alg_len == &hash_len {
            match alg_len {
                32 => {
                    println!("Loaded MD5 hash.");
                }
                40 => {
                    println!("Loaded SHA-1 hash.");
                }
                80 => {
                    println!("Loaded RipeMD320 hash.");
                }
                64 => {
                    println!("Loaded SHA-256 hash.");
                }
                128 => {
                    println!("Loaded SHA-512 hash.");
                }
                _ => {
                    break;
                }
            }
        }
    }

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

    hash_dict.par_iter().for_each(|lines| {
        let line = lines.clone();
        crack(line, hash_len, now);
    });

    Ok(())
}

// reusing the same buffer.
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
