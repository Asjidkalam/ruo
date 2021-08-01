# `Ruo` 🧁
Ruo is a dictionary-based password cracker written in rust 🦀. The primary purpose is to crack weak hashes/commonly used passwords.

Cracked passwords will be printed to the terminal and saved in the file `$HOME/.ruo/hashes.saved`. The `$HOME/.ruo/hashes.saved` file is also used to not load password hashes that you already cracked when you run ruo the next time.

<p align="left">
	<img src="https://img.shields.io/badge/version-0.1.0-blue.svg" title="version" alt="version">
</p>


## Available algorithms


| Name        | Algorithm  | Crates.io |
|-------------|------------|-----------|
| `md5`       | MD5        | [![crates.io](https://img.shields.io/crates/v/md5.svg)](https://crates.io/crates/md5)      |
| `sha1`      | SHA-1      | [![crates.io](https://img.shields.io/crates/v/sha1.svg)](https://crates.io/crates/sha-1)   |
| `sha256`    | SHA-2 256  | [![crates.io](https://img.shields.io/crates/v/sha256.svg)](https://crates.io/crates/sha2)  |
| `sha512`    | SHA-2 512  | [![crates.io](https://img.shields.io/crates/v/sha256.svg)](https://crates.io/crates/sha2)  |
|`ripemd320`  | RIPEMD320  | [![crates.io](https://img.shields.io/crates/v/ripemd320.svg)](https://crates.io/crates/ripemd320)|


## Build From Source

### Prerequisites

You'll need the following tools to build from source:

* [Rust](https://www.rust-lang.org/en-US/install.html)  
* `Cargo` 


### Building

Clone the repository and use cargo to generate a release build.
```sh
$ git clone https://github.com/Asjidkalam/ruo.git
$ cd ruo/
$ cargo build --release
```

## Usage
```sh
$ ./target/release/ruo <WORDLIST-FILE-PATH> <HASH-TO-CRACK>
```

### Example
```
🔫 ruo v0.1
Loaded SHA-256 hash.
Loaded the wordlist file in 1838 millisecs.
🤍 Cracked! 244f28ce3685167745ad3a7f1760fd4483bbbb3fd150b9087b95442d4d6fd905 -> "PASSWORD1" in 6 millisecs
```

## Contribute

* Suggest a feature / Report a bug 
* More algorithms
* Better optimization
* Help me document the code :)

## License
This project is licensed under the terms of the MIT license. 
Check the [LICENSE](LICENSE.md) file out for license rights and limitations.

🍰
