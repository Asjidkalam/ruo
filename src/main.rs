mod algorithms;
mod banner;

use instant::Instant;
use lazy_static::lazy_static;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::prelude::*;
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
    @odinshell - 01/08/2021
*/

// debug
static mut LOAD_TIME: u128 = 0;

#[derive(Serialize, Deserialize)]
struct LocalHash {
    hash: String,
    plaintext: String,
}

fn load_local_hive() -> Vec<LocalHash> {
    let json_file_path = Path::new(&*LOCAL_HASH_PATH);

    let data = fs::read_to_string(json_file_path).unwrap();
    let mut local_hive: Vec<LocalHash> = Vec::new();
    if fs::metadata(json_file_path).unwrap().len() != 0 {
        local_hive = serde_json::from_str(&data).unwrap();
    }
    local_hive
}

fn crack(line: &str, hash_len: usize, now: std::time::Instant) {
    let formatted_hash: String = algorithms::create_hash(line, hash_len);

    if formatted_hash == *HASH_INPUT {
        /*
            LOAD_TIME and unsafe{} is currently used for wordlist load time debug
            will be removed later.
        */
        unsafe {
            println!(
                "🤍 Cracked! {} -> \"{}\" in {} millisecs",
                formatted_hash,
                line,
                now.elapsed().as_millis() - LOAD_TIME
            );
        }

        // loading the hash file, adding the new hash and saving locally.
        let mut local_hive = load_local_hive();

        let new_hash = LocalHash {
            hash: formatted_hash,
            plaintext: line.to_string(),
        };

        local_hive.push(new_hash);
        let json = serde_json::to_string(&local_hive).unwrap();

        let mut file_write = fs::OpenOptions::new()
            .write(true)
            .append(false)
            .open(&*LOCAL_HASH_PATH)
            .unwrap();

        if let Err(e) = writeln!(file_write, "{}", &json) {
            eprintln!("Couldn't write to file: {}", e);
        }

        process::exit(0);
    }
}

fn main() -> std::io::Result<()> {
    banner::display_banner();

    // TODO: argument parser lol.
    let args: Vec<String> = env::args().collect();
    let wordlist_file = &args[1];

    // check for saved hashes locally
    fs::create_dir_all(format!("{}/.ruo", env::home_dir().unwrap().display()))?;

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

    let local_hive = load_local_hive();

    // check if the current hash matches to the hash in the local_hive.
    for hash_object in local_hive {
        if hash_object.hash == *HASH_INPUT {
            println!(
                "🤍 Saved hash found! {} -> \"{}\"",
                hash_object.hash, hash_object.plaintext
            );
            process::exit(0);
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

    let now = Instant::now();

    let file = fs::read_to_string(wordlist_file).unwrap();
    let newline_split = file.split("\n");
    let dict: Vec<&str> = newline_split.collect();

    // debug
    unsafe {
        LOAD_TIME = now.elapsed().as_millis();
        println!("loaded the wordlist file in {} millisecs.", LOAD_TIME);
    }

    // rayon goes brr
    dict.par_iter().for_each(|lines| {
        // let line = lines.clone();
        crack(lines, hash_len, now);
    });

    Ok(())
}
