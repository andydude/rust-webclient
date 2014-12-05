extern crate webclient;

use std::io::Reader;
use std::io::stdin;

use webclient::webclient::digest;
use webclient::webclient::digest::types::HashAlgorithm;

pub fn hash_algorithm_from_lower(name: ~str) -> Result<~HashAlgorithm, &'static str> {
    if name == ~"-md5" {
        Ok(digest::md5::md5_new())
    } else if name == ~"-sha1" {
        Ok(digest::sha1::sha1_new())
    } else if name == ~"-sha224" {
        Ok(digest::sha2::sha224_new())
    } else if name == ~"-sha256" {
        Ok(digest::sha2::sha256_new())
    } else if name == ~"-sha384" {
        Ok(digest::sha2::sha384_new())
    } else if name == ~"-sha512" {
        Ok(digest::sha2::sha512_new())
    } else if name == ~"-sha512224" {
        Ok(digest::sha2::sha512224_new())
    } else if name == ~"-sha512256" {
        Ok(digest::sha2::sha512256_new())
    } else {
        Err("invalid hash algorithm")
    }
}

fn main() {
    // get message
    let mut reader = stdin();
    let message = reader.read_to_end().unwrap();
    let msg = message.as_slice();

    // get hash algorithm
    let command: ~str = std::os::args()[1];
    let mut hasher = hash_algorithm_from_lower(command).unwrap();

    // compute hash
    let hash = hasher.hash(msg);
    for byte_i in range(0u, hash.len()) {
        print!("{:02x}", hash[byte_i]);
    }
    println!("");
}
