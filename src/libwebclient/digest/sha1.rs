use std::io::Reader;
use std::io::stdin;

use webclient::bits::u32;
use webclient::bits::u64;

//
// BEGIN public API
//

pub static md5_initial_hash: [u32, .. 5] = [
	0x67452301u32, // digits are (34*n + 1) where n = 3, 2, 1, 0
	0xefcdab89u32, // digits are (34*n + 1) where n = 7, 6, 5, 4
	0x98badcfeu32, // digits are (34*n + 16) where n = 4, 5, 6, 7
	0x10325476u32, // digits are (34*n + 16) where n = 0, 1, 2, 3
	0xc3d2e1f0u32  // digits are (15*n) where n = 13, 14, 15, 16
];

pub static md5_constant_pool: [u32, .. 4] = [
	0x5a827999u32, // digits of floor(sqrt(2)*2^30)
	0x6ed9eba1u32, // digits of floor(sqrt(3)*2^30)
	0x8f1bbcdcu32, // digits of floor(sqrt(5)*2^30)
	0xca62c1d6u32  // digits of floor(sqrt(10)*2^30)
];

pub fn sha1_begin() -> ~[u32] {
    // FIPS-180-4 SS 6.1.1.1 initial hash value
    return ~[
        md5_initial_hash[0],
        md5_initial_hash[1],
        md5_initial_hash[2],
        md5_initial_hash[3],
        md5_initial_hash[4]
    ];
}

pub fn sha1_pad(msg: &mut Vec<u8>) {
    // FIPS-180-4 SS 6.1.1.2 message is padded
    let len = msg.len();
    msg.push(0x80u8);
    for _ in range(0, (55 - len) % 64) {
        msg.push(0u8);
    }
    let pad = u64::to_be((len as u64)*8);
    for t in range(0u, 8u) {
        msg.push(pad[t]);
    }
}

pub fn sha1_update_bytes(hash: &mut[u32], m: &[u8]) {
    assert!(hash.len() == 5);
    assert!(m.len() == 64);

    // FIPS-180-4 SS 6.1.2.1 prepare message schedule
    let mut w = [0u32, ..80];
    for t in range(0u, 16u) {
        w[t] = u32::from_be(m.slice(4*t, 4*t+4)); 

    }
    for t in range(16u, 80u) {
        w[t] = u32::rotl(w[t-3]^w[t-8]^w[t-14]^w[t-16], 1);
    }

    sha1_update_words(hash, w.as_slice());
}

pub fn sha1_update_words(hash: &mut[u32], w: &[u32]) {
    assert!(hash.len() == 5);
    assert!(w.len() == 80);

    // FIPS-180-4 SS 6.1.2.2 initialize working variables
    let mut temp: u32;
    let mut a: u32 = hash[0];
    let mut b: u32 = hash[1];
    let mut c: u32 = hash[2];
    let mut d: u32 = hash[3];
    let mut e: u32 = hash[4];

    // FIPS-180-4 SS 4.1.1 functions
    // FIPS-180-4 SS 4.2.1 constants
    // FIPS-180-4 SS 6.1.2.3
    for t in range(0u, 20u) {
        temp = u32::rotl(a, 5) + e + w[t] + md5_constant_pool[0] + u32::bool3ary_202(b, c, d);
        e = d; d = c;
        c = u32::rotl(b, 30); 
        b = a; a = temp;
    }
    for t in range(20u, 40u) {
        temp = u32::rotl(a, 5) + e + w[t] + md5_constant_pool[1] + u32::bool3ary_150(b, c, d);
        e = d; d = c;
        c = u32::rotl(b, 30); 
        b = a; a = temp;
    }
    for t in range(40u, 60u) {
        temp = u32::rotl(a, 5) + e + w[t] + md5_constant_pool[2] + u32::bool3ary_232(b, c, d);
        e = d; d = c;
        c = u32::rotl(b, 30); 
        b = a; a = temp;
    }
    for t in range(60u, 80u) {
        temp = u32::rotl(a, 5) + e + w[t] + md5_constant_pool[3] + u32::bool3ary_150(b, c, d);
        e = d; d = c;
        c = u32::rotl(b, 30); 
        b = a; a = temp;
    }

    // FIPS-180-4 SS 6.1.2.4
    hash[0] += a;
    hash[1] += b;
    hash[2] += c;
    hash[3] += d;
    hash[4] += e;
}

pub fn sha1_hash(msg: &mut Vec<u8>) -> ~[u32] {
    let mut hash = sha1_begin();

    sha1_pad(msg);

    for block in msg.as_slice().chunks(64) {
        sha1_update_bytes(hash, block);
    }

    return hash;
}

pub fn sha1_dump(hash: &[u32]) {
    println!("{:08x}{:08x}{:08x}{:08x}{:08x}", 
             hash[0],
             hash[1], 
             hash[2], 
             hash[3], 
             hash[4]);
}

//
// END public API
//

pub fn main() {
    let mut reader = stdin();
    let mut msg = reader.read_to_end().unwrap();
    let hash = sha1_hash(&mut msg);
    sha1_dump(hash);
}
