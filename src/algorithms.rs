use sha2::{Digest, Sha256, Sha512};

pub fn create_hash(line: &String, hash_length: usize) -> String {
    match hash_length {
        // MD5
        32 => {
            let hashvalue = md5::Md5::digest(line.as_bytes());
            format!("{:x}", hashvalue)
        }

        // SHA-1
        40 => {
            use sha1::Sha1;
            let mut hasher = Sha1::new();
            hasher.update(line.as_bytes());
            format!("{}", hasher.digest().to_string())
        }

        // RipeMD320
        80 => {
            use ripemd320::Ripemd320;
            let result = Ripemd320::digest(line.as_bytes());
            format!("{:x}", result)
        }

        // SHA-256
        64 => {
            let mut hasher = Sha256::new();
            hasher.update(line.as_bytes());
            format!("{:x}", hasher.finalize())
        }

        // SHA-512
        128 => {
            let mut hasher = Sha512::new();
            hasher.update(line.as_bytes());
            format!("{:x}", hasher.finalize())
        }

        _ => "".to_string(),
    }
}
