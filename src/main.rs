mod banner;

use instant::Instant;
use md5::Digest;
// use rayon::prelude::*;
use std::env;

/*
    example hash(MD5):
    e890f806dfd189052ca7b39ac29da142 -> PASSWORD1
*/

// fn crack(wordlist_string: &mut String) -> String {
//     // let wordlist_string = wordlist_string.unwrap().clone();
//     let hashvalue = md5::Md5::digest(wordlist_string.as_bytes());
//     format!("{:x}", hashvalue)
// }

fn main() -> std::io::Result<()> {
    banner::display_banner();

    let now = Instant::now();
    let mut found: bool = false;

    // TODO: use a real fucking argument parser.
    let args: Vec<String> = env::args().collect();
    let wordlist_file = &args[1];
    let hash_input = args[2].parse::<String>().unwrap();

    println!("📋 Wordlist: {}", wordlist_file);

    // TODO: check if the hash is already cracked in the local crack repository and also in the online repository.
    // local repository: ~/.ruo/ruo.saved

    let mut reader = my_reader::BufReader::open(wordlist_file)?;
    let mut buffer = String::new();

    while let Some(line) = reader.read_line(&mut buffer) {
        let wordlist_string = line?.trim();
        let hashvalue = md5::Md5::digest(wordlist_string.as_bytes());

        let formatted_hash = format!("{:x}", hashvalue);

        if formatted_hash == hash_input {
            println!("🤍 Cracked! {} -> {}", formatted_hash, wordlist_string);
            found = true;
            break;
        }
    }

    if found {
        println!(
            "✔️ Session completed in {}s. Hash found!",
            now.elapsed().as_secs()
        );
    } else {
        println!(
            "❌ Session completed in {}s. Hash not found.",
            now.elapsed().as_secs()
        );
    }

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
