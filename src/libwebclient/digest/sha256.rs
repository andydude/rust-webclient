use std::io::Reader;
use std::io::stdin;

use webclient::bits::u32;
use webclient::bits::u64;

//
// BEGIN public API
//

pub static sha256_constant_pool: [u32, .. 64] = [
	0x428a2f98u32, // floor(mod(cbrt(2), 1)*2**32)
	0x71374491u32, // floor(mod(cbrt(3), 1)*2**32)
	0xb5c0fbcfu32, // floor(mod(cbrt(5), 1)*2**32)
	0xe9b5dba5u32, // floor(mod(cbrt(7), 1)*2**32)
	0x3956c25bu32, // floor(mod(cbrt(11), 1)*2**32)
	0x59f111f1u32, // floor(mod(cbrt(13), 1)*2**32)
	0x923f82a4u32, // floor(mod(cbrt(17), 1)*2**32)
	0xab1c5ed5u32, // floor(mod(cbrt(19), 1)*2**32)
	0xd807aa98u32, // floor(mod(cbrt(23), 1)*2**32)
	0x12835b01u32, // floor(mod(cbrt(29), 1)*2**32)
	0x243185beu32, // floor(mod(cbrt(31), 1)*2**32)
	0x550c7dc3u32, // floor(mod(cbrt(37), 1)*2**32)
	0x72be5d74u32, // floor(mod(cbrt(41), 1)*2**32)
	0x80deb1feu32, // floor(mod(cbrt(43), 1)*2**32)
	0x9bdc06a7u32, // floor(mod(cbrt(47), 1)*2**32)
	0xc19bf174u32, // floor(mod(cbrt(53), 1)*2**32)
	0xe49b69c1u32, // floor(mod(cbrt(59), 1)*2**32)
	0xefbe4786u32, // floor(mod(cbrt(61), 1)*2**32)
	0x0fc19dc6u32, // floor(mod(cbrt(67), 1)*2**32)
	0x240ca1ccu32, // floor(mod(cbrt(71), 1)*2**32)
	0x2de92c6fu32, // floor(mod(cbrt(73), 1)*2**32)
	0x4a7484aau32, // floor(mod(cbrt(79), 1)*2**32)
	0x5cb0a9dcu32, // floor(mod(cbrt(83), 1)*2**32)
	0x76f988dau32, // floor(mod(cbrt(89), 1)*2**32)
	0x983e5152u32, // floor(mod(cbrt(97), 1)*2**32)
	0xa831c66du32, // floor(mod(cbrt(101), 1)*2**32)
	0xb00327c8u32, // floor(mod(cbrt(103), 1)*2**32)
	0xbf597fc7u32, // floor(mod(cbrt(107), 1)*2**32)
	0xc6e00bf3u32, // floor(mod(cbrt(109), 1)*2**32)
	0xd5a79147u32, // floor(mod(cbrt(113), 1)*2**32)
	0x06ca6351u32, // floor(mod(cbrt(127), 1)*2**32)
	0x14292967u32, // floor(mod(cbrt(131), 1)*2**32)
	0x27b70a85u32, // floor(mod(cbrt(137), 1)*2**32)
	0x2e1b2138u32, // floor(mod(cbrt(139), 1)*2**32)
	0x4d2c6dfcu32, // floor(mod(cbrt(149), 1)*2**32)
	0x53380d13u32, // floor(mod(cbrt(151), 1)*2**32)
	0x650a7354u32, // floor(mod(cbrt(157), 1)*2**32)
	0x766a0abbu32, // floor(mod(cbrt(163), 1)*2**32)
	0x81c2c92eu32, // floor(mod(cbrt(167), 1)*2**32)
	0x92722c85u32, // floor(mod(cbrt(173), 1)*2**32)
	0xa2bfe8a1u32, // floor(mod(cbrt(179), 1)*2**32)
	0xa81a664bu32, // floor(mod(cbrt(181), 1)*2**32)
	0xc24b8b70u32, // floor(mod(cbrt(191), 1)*2**32)
	0xc76c51a3u32, // floor(mod(cbrt(193), 1)*2**32)
	0xd192e819u32, // floor(mod(cbrt(197), 1)*2**32)
	0xd6990624u32, // floor(mod(cbrt(199), 1)*2**32)
	0xf40e3585u32, // floor(mod(cbrt(211), 1)*2**32)
	0x106aa070u32, // floor(mod(cbrt(223), 1)*2**32)
	0x19a4c116u32, // floor(mod(cbrt(227), 1)*2**32)
	0x1e376c08u32, // floor(mod(cbrt(229), 1)*2**32)
	0x2748774cu32, // floor(mod(cbrt(233), 1)*2**32)
	0x34b0bcb5u32, // floor(mod(cbrt(239), 1)*2**32)
	0x391c0cb3u32, // floor(mod(cbrt(241), 1)*2**32)
	0x4ed8aa4au32, // floor(mod(cbrt(251), 1)*2**32)
	0x5b9cca4fu32, // floor(mod(cbrt(257), 1)*2**32)
	0x682e6ff3u32, // floor(mod(cbrt(263), 1)*2**32)
	0x748f82eeu32, // floor(mod(cbrt(269), 1)*2**32)
	0x78a5636fu32, // floor(mod(cbrt(271), 1)*2**32)
	0x84c87814u32, // floor(mod(cbrt(277), 1)*2**32)
	0x8cc70208u32, // floor(mod(cbrt(281), 1)*2**32)
	0x90befffau32, // floor(mod(cbrt(283), 1)*2**32)
	0xa4506cebu32, // floor(mod(cbrt(293), 1)*2**32)
	0xbef9a3f7u32, // floor(mod(cbrt(307), 1)*2**32)
	0xc67178f2u32  // floor(mod(cbrt(311), 1)*2**32)
];

pub static sha256_initial_hash: [u32, .. 8] = [
    0x6a09e667u32, // floor(mod(sqrt(2), 1)*2**32)
    0xbb67ae85u32, // floor(mod(sqrt(3), 1)*2**32)
    0x3c6ef372u32, // floor(mod(sqrt(5), 1)*2**32)
    0xa54ff53au32, // floor(mod(sqrt(7), 1)*2**32)
    0x510e527fu32, // floor(mod(sqrt(11), 1)*2**32)
    0x9b05688cu32, // floor(mod(sqrt(13), 1)*2**32)
    0x1f83d9abu32, // floor(mod(sqrt(17), 1)*2**32)
    0x5be0cd19u32  // floor(mod(sqrt(19), 1)*2**32)
];

pub fn sha256_begin() -> ~[u32] {
    // FIPS-180-4 SS 5.3.3 initial hash value
    // FIPS-180-4 SS 6.2.1.1 initial hash value
    return ~[
        sha256_initial_hash[0],
        sha256_initial_hash[1],
        sha256_initial_hash[2],
        sha256_initial_hash[3],
        sha256_initial_hash[4],
        sha256_initial_hash[5],
        sha256_initial_hash[6],
        sha256_initial_hash[7]
    ];
}

pub fn sha256_pad(msg: &mut Vec<u8>) {
    // FIPS-180-4 SS 6.2.1.2 message is padded
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

/* FIPS-180-4 represents this function with GREEK CAPITAL LETTER SIGMA, sub 0:
 * <math><msub><mi>&#x03A3;</mi><mn>0</mn></msub></math>
 */
pub fn sha256_sigma0(x: u32) -> u32 {
    u32::rotr(x, 2) ^ u32::rotr(x, 13) ^ u32::rotr(x, 22)
}

/* FIPS-180-4 represents this function with GREEK CAPITAL LETTER SIGMA, sub 1:
 * <math><msub><mi>&#x03A3;</mi><mn>1</mn></msub></math>
 */
pub fn sha256_sigma1(x: u32) -> u32 {
    u32::rotr(x, 6) ^ u32::rotr(x, 11) ^ u32::rotr(x, 25)
}

/* FIPS-180-4 represents this function with GREEK SMALL LETTER SIGMA, sub 0:
 * <math><msub><mi>&#x03C3;</mi><mn>0</mn></msub></math>
 */
pub fn sha256_sig0(x: u32) -> u32 {
    u32::rotr(x, 7) ^ u32::rotr(x, 18) ^ u32::shr(x, 3)
}

/* FIPS-180-4 represents this function with GREEK SMALL LETTER SIGMA, sub 1:
 * <math><msub><mi>&#x03C3;</mi><mn>1</mn></msub></math>
 */
pub fn sha256_sig1(x: u32) -> u32 {
    u32::rotr(x, 17) ^ u32::rotr(x, 19) ^ u32::shr(x, 10)
}

//pub fn sha512_sigma0(x: u64) -> u64 {
//    u32::rotr(x, 28) ^ u32::rotr(x, 34) ^ u32::rotr(x, 39)
//}
//
//pub fn sha512_sigma1() -> u64 {
//    u32::rotr(x, 14) ^ u32::rotr(x, 18) ^ u32::rotr(x, 41)
//}
//
//pub fn sha512_sig0() -> u64 {
//    u32::rotr(x, 1) ^ u32::rotr(x, 8) ^ u32::shr(x, 7)
//}
//
//pub fn sha512_sig1() -> u64 {
//    u32::rotr(x, 19) ^ u32::rotr(x, 61) ^ u32::shr(x, 6)
//}

pub fn sha256_update_bytes(hash: &mut[u32], m: &[u8]) {
    assert!(hash.len() == 8);
    assert!(m.len() == 64);

    // FIPS-180-4 SS 6.1.2.1 prepare message schedule
    let mut w = [0u32, ..64];
    for t in range(0u, 16u) {
        w[t] = u32::from_be(m.slice(4*t, 4*t+4));

    }
    for t in range(16u, 64u) {
        w[t] = sha256_sig1(w[t-2]) + w[t-7] + sha256_sig0(w[t-15]) + w[t-16];
    }

    sha256_update_words(hash, w.as_slice());
}

pub fn sha256_update_words(hash: &mut[u32], w: &[u32]) {
    assert!(hash.len() == 8);
    assert!(w.len() == 64);

    // FIPS-180-4 SS 6.1.2.2 initialize working variables
    let mut temp1: u32;
    let mut temp2: u32;
    let mut a: u32 = hash[0];
    let mut b: u32 = hash[1];
    let mut c: u32 = hash[2];
    let mut d: u32 = hash[3];
    let mut e: u32 = hash[4];
    let mut f: u32 = hash[5];
    let mut g: u32 = hash[6];
    let mut h: u32 = hash[7];

    // FIPS-180-4 SS 4.1.1 functions
    // FIPS-180-4 SS 4.2.2 constants
    // FIPS-180-4 SS 6.1.2.3
    for t in range(0u, 64u) {
        temp1 = h + sha256_sigma1(e) + u32::bool3ary_202(e, f, g) + sha256_constant_pool[t] + w[t];
        temp2 = sha256_sigma0(a) + u32::bool3ary_232(a, b, c);
        h = g; g = f; f = e; e = d + temp1; d = c; c = b; b = a;
        a = temp1 + temp2;
    }

    // FIPS-180-4 SS 6.1.2.4
    hash[0] += a;
    hash[1] += b;
    hash[2] += c;
    hash[3] += d;
    hash[4] += e;
    hash[5] += f;
    hash[6] += g;
    hash[7] += h;
}

pub fn sha256_hash(msg: &mut Vec<u8>) -> ~[u32] {
    let mut hash = sha256_begin();

    sha256_pad(msg);

    for block in msg.as_slice().chunks(64) {
        sha256_update_bytes(hash, block);
    }

    return hash;
}

pub fn sha256_dump(hash: &[u32]) {
    println!("{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}",
             hash[0],
             hash[1],
             hash[2],
             hash[3],
             hash[4],
             hash[5],
             hash[6],
             hash[7]);
}

//
// END public API
//

pub fn main() {
    let mut reader = stdin();
    let mut msg = reader.read_to_end().unwrap();
    let hash = sha256_hash(&mut msg);
    sha256_dump(hash);
}
