extern crate webclient;

use std::io::Reader;
use std::io::stdin;

use webclient::webclient::digest;
use webclient::webclient::digest::types::HashAlgorithm;

pub fn hash_algorithm_from_lower(name: &str) -> Option<HashAlgorithm> {
    if name == "-md5" {
        Some(digest::md5::md5_new())
    } else if name == "-sha1" {
        Some(digest::sha1::sha1_new())
    } else if name == "-sha224" {
        Some(digest::sha2::sha224_new())
    } else if name == "-sha256" {
        Some(digest::sha2::sha256_new())
    } else if name == "-sha384" {
        Some(digest::sha2::sha384_new())
    } else if name == "-sha512" {
        Some(digest::sha2::sha512_new())
    } else if name == "-sha512224" {
        Some(digest::sha2::sha512224_new())
    } else if name == "-sha512256" {
        Some(digest::sha2::sha512256_new())
    } else {
        None
    }
}

fn main() {
    // get message
    let mut reader = stdin();
    let message = reader.read_to_end().unwrap();
    let msg = message.as_slice();

    // get hash algorithm
    let command: &str = std::os::args()[1];
    let mut hasher = hash_algorithm_from_lower(command).unwrap();

    // compute hash
    let bytes = hasher.hash(msg);
    for byte in bytes.into_iter() {
        print!("{:02x}", byte);
    }
    println!("");
}
