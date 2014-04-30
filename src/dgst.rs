extern crate webclient;

use std::io::Reader;
use std::io::stdin;

use webclient::webclient::digest;
use webclient::webclient::digest::types::HashAlgorithm;

pub fn hash_algorithm_from_lower(name: ~str) -> Result<~HashAlgorithm, &'static str> {
    return if name == ~"-md5" {
        Ok(digest::md5::md5_new())
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
    let mut hasher: ~HashAlgorithm = hash_algorithm_from_lower(command).unwrap();

    // compute hash
    let hash = hasher.hash(msg);
    for byte in hash.iter() {
        print!("{:02x}", *byte);
    }
    println!("");

    //if command == ~"-md5" {
    //    digest::md5::main()
    //} else if command == ~"-sha1" {
    //    digest::sha1::main()
    //} else if command == ~"-sha224" {
    //    digest::sha2::main_224()
    //} else if command == ~"-sha256" {
    //    digest::sha2::main_256()
    //} else if command == ~"-sha384" {
    //    digest::sha2::main_384()
    //} else if command == ~"-sha512" {
    //    digest::sha2::main_512()
    //} else if command == ~"-sha512224" {
    //    digest::sha2::main_512_224()
    //} else if command == ~"-sha512256" {
    //    digest::sha2::main_512_256()
    //}
}
