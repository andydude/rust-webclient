extern crate webclient;
use webclient::webclient::digest;

fn main() {
    let command: ~str = std::os::args()[1];

    if command == ~"-md5" {
        digest::md5::main()
    } else if command == ~"-sha1" {
        digest::sha1::main()
    } else if command == ~"-sha256" {
        digest::sha256::main()
    } else if command == ~"-sha512" {
        digest::sha512::main()
    }
}
