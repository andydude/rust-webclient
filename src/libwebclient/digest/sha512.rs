use std::io::Reader;
use std::io::stdin;

use webclient::bits::u64;

//
// BEGIN public API
//

pub static sha512_constant_pool: [u64, .. 80] = [
	0x428a2f98d728ae22u64, // floor(mod(cbrt(2), 1)*2**64)
	0x7137449123ef65cdu64, // floor(mod(cbrt(3), 1)*2**64)
	0xb5c0fbcfec4d3b2fu64, // floor(mod(cbrt(5), 1)*2**64)
	0xe9b5dba58189dbbcu64, // floor(mod(cbrt(7), 1)*2**64)
	0x3956c25bf348b538u64, // floor(mod(cbrt(11), 1)*2**64)
	0x59f111f1b605d019u64, // floor(mod(cbrt(13), 1)*2**64)
	0x923f82a4af194f9bu64, // floor(mod(cbrt(17), 1)*2**64)
	0xab1c5ed5da6d8118u64, // floor(mod(cbrt(19), 1)*2**64)
	0xd807aa98a3030242u64, // floor(mod(cbrt(23), 1)*2**64)
	0x12835b0145706fbeu64, // floor(mod(cbrt(29), 1)*2**64)
	0x243185be4ee4b28cu64, // floor(mod(cbrt(31), 1)*2**64)
	0x550c7dc3d5ffb4e2u64, // floor(mod(cbrt(37), 1)*2**64)
	0x72be5d74f27b896fu64, // floor(mod(cbrt(41), 1)*2**64)
	0x80deb1fe3b1696b1u64, // floor(mod(cbrt(43), 1)*2**64)
	0x9bdc06a725c71235u64, // floor(mod(cbrt(47), 1)*2**64)
	0xc19bf174cf692694u64, // floor(mod(cbrt(53), 1)*2**64)
	0xe49b69c19ef14ad2u64, // floor(mod(cbrt(59), 1)*2**64)
	0xefbe4786384f25e3u64, // floor(mod(cbrt(61), 1)*2**64)
	0x0fc19dc68b8cd5b5u64, // floor(mod(cbrt(67), 1)*2**64)
	0x240ca1cc77ac9c65u64, // floor(mod(cbrt(71), 1)*2**64)
	0x2de92c6f592b0275u64, // floor(mod(cbrt(73), 1)*2**64)
	0x4a7484aa6ea6e483u64, // floor(mod(cbrt(79), 1)*2**64)
	0x5cb0a9dcbd41fbd4u64, // floor(mod(cbrt(83), 1)*2**64)
	0x76f988da831153b5u64, // floor(mod(cbrt(89), 1)*2**64)
	0x983e5152ee66dfabu64, // floor(mod(cbrt(97), 1)*2**64)
	0xa831c66d2db43210u64, // floor(mod(cbrt(101), 1)*2**64)
	0xb00327c898fb213fu64, // floor(mod(cbrt(103), 1)*2**64)
	0xbf597fc7beef0ee4u64, // floor(mod(cbrt(107), 1)*2**64)
	0xc6e00bf33da88fc2u64, // floor(mod(cbrt(109), 1)*2**64)
	0xd5a79147930aa725u64, // floor(mod(cbrt(113), 1)*2**64)
	0x06ca6351e003826fu64, // floor(mod(cbrt(127), 1)*2**64)
	0x142929670a0e6e70u64, // floor(mod(cbrt(131), 1)*2**64)
	0x27b70a8546d22ffcu64, // floor(mod(cbrt(137), 1)*2**64)
	0x2e1b21385c26c926u64, // floor(mod(cbrt(139), 1)*2**64)
	0x4d2c6dfc5ac42aedu64, // floor(mod(cbrt(149), 1)*2**64)
	0x53380d139d95b3dfu64, // floor(mod(cbrt(151), 1)*2**64)
	0x650a73548baf63deu64, // floor(mod(cbrt(157), 1)*2**64)
	0x766a0abb3c77b2a8u64, // floor(mod(cbrt(163), 1)*2**64)
	0x81c2c92e47edaee6u64, // floor(mod(cbrt(167), 1)*2**64)
	0x92722c851482353bu64, // floor(mod(cbrt(173), 1)*2**64)
	0xa2bfe8a14cf10364u64, // floor(mod(cbrt(179), 1)*2**64)
	0xa81a664bbc423001u64, // floor(mod(cbrt(181), 1)*2**64)
	0xc24b8b70d0f89791u64, // floor(mod(cbrt(191), 1)*2**64)
	0xc76c51a30654be30u64, // floor(mod(cbrt(193), 1)*2**64)
	0xd192e819d6ef5218u64, // floor(mod(cbrt(197), 1)*2**64)
	0xd69906245565a910u64, // floor(mod(cbrt(199), 1)*2**64)
	0xf40e35855771202au64, // floor(mod(cbrt(211), 1)*2**64)
	0x106aa07032bbd1b8u64, // floor(mod(cbrt(223), 1)*2**64)
	0x19a4c116b8d2d0c8u64, // floor(mod(cbrt(227), 1)*2**64)
	0x1e376c085141ab53u64, // floor(mod(cbrt(229), 1)*2**64)
	0x2748774cdf8eeb99u64, // floor(mod(cbrt(233), 1)*2**64)
	0x34b0bcb5e19b48a8u64, // floor(mod(cbrt(239), 1)*2**64)
	0x391c0cb3c5c95a63u64, // floor(mod(cbrt(241), 1)*2**64)
	0x4ed8aa4ae3418acbu64, // floor(mod(cbrt(251), 1)*2**64)
	0x5b9cca4f7763e373u64, // floor(mod(cbrt(257), 1)*2**64)
	0x682e6ff3d6b2b8a3u64, // floor(mod(cbrt(263), 1)*2**64)
	0x748f82ee5defb2fcu64, // floor(mod(cbrt(269), 1)*2**64)
	0x78a5636f43172f60u64, // floor(mod(cbrt(271), 1)*2**64)
	0x84c87814a1f0ab72u64, // floor(mod(cbrt(277), 1)*2**64)
	0x8cc702081a6439ecu64, // floor(mod(cbrt(281), 1)*2**64)
	0x90befffa23631e28u64, // floor(mod(cbrt(283), 1)*2**64)
	0xa4506cebde82bde9u64, // floor(mod(cbrt(293), 1)*2**64)
	0xbef9a3f7b2c67915u64, // floor(mod(cbrt(307), 1)*2**64)
	0xc67178f2e372532bu64, // floor(mod(cbrt(311), 1)*2**64)
	0xca273eceea26619cu64, //
	0xd186b8c721c0c207u64, //
	0xeada7dd6cde0eb1eu64, //
	0xf57d4f7fee6ed178u64, //
	0x06f067aa72176fbau64, //
	0x0a637dc5a2c898a6u64, //
	0x113f9804bef90daeu64, //
	0x1b710b35131c471bu64, //
	0x28db77f523047d84u64, //
	0x32caab7b40c72493u64, //
	0x3c9ebe0a15c9bebcu64, //
	0x431d67c49c100d4cu64, //
	0x4cc5d4becb3e42b6u64, //
	0x597f299cfc657e2au64, //
	0x5fcb6fab3ad6faecu64, //
	0x6c44198c4a475817u64  //
];

pub static sha512_initial_hash: [u64, .. 8] = [
    0x6a09e667f3bcc908u64, // floor(mod(sqrt(2), 1)*2**64)
    0xbb67ae8584caa73bu64, // floor(mod(sqrt(3), 1)*2**64)
    0x3c6ef372fe94f82bu64, // floor(mod(sqrt(5), 1)*2**64)
    0xa54ff53a5f1d36f1u64, // floor(mod(sqrt(7), 1)*2**64)
    0x510e527fade682d1u64, // floor(mod(sqrt(11), 1)*2**64)
    0x9b05688c2b3e6c1fu64, // floor(mod(sqrt(13), 1)*2**64)
    0x1f83d9abfb41bd6bu64, // floor(mod(sqrt(17), 1)*2**64)
    0x5be0cd19137e2179u64  // floor(mod(sqrt(19), 1)*2**64)
];

pub fn sha512_begin() -> ~[u64] {
    // FIPS-180-4 SS 5.3.3 initial hash value
    // FIPS-180-4 SS 6.2.1.1 initial hash value
    return ~[
        sha512_initial_hash[0],
        sha512_initial_hash[1],
        sha512_initial_hash[2],
        sha512_initial_hash[3],
        sha512_initial_hash[4],
        sha512_initial_hash[5],
        sha512_initial_hash[6],
        sha512_initial_hash[7]
    ];
}

pub fn sha512_pad(msg: &mut Vec<u8>) {
    // FIPS-180-4 SS 6.2.1.2 message is padded
    let len = msg.len();
    msg.push(0x80u8);
    for _ in range(0, (111 - len) % 128) {
        msg.push(0u8);
    }

    // most significant u64 of the u128 size
    for _ in range(0u, 8u) {
        msg.push(0u8);
    }

    // least significant u64 of the u128 size
    let pad = u64::to_be((len as u64)*8);
    for t in range(0u, 8u) {
        msg.push(pad[t]);
    }
}

/* FIPS-180-4 represents this function with GREEK CAPITAL LETTER SIGMA, sub 0:
 * <math><msub><mi>&#x03A3;</mi><mn>0</mn></msub></math>
 */
pub fn sha512_sigma0(x: u64) -> u64 {
    u64::rotr(x, 28) ^ u64::rotr(x, 34) ^ u64::rotr(x, 39)
}

/* FIPS-180-4 represents this function with GREEK CAPITAL LETTER SIGMA, sub 1:
 * <math><msub><mi>&#x03A3;</mi><mn>1</mn></msub></math>
 */
pub fn sha512_sigma1(x: u64) -> u64 {
    u64::rotr(x, 14) ^ u64::rotr(x, 18) ^ u64::rotr(x, 41)
}

/* FIPS-180-4 represents this function with GREEK SMALL LETTER SIGMA, sub 0:
 * <math><msub><mi>&#x03C3;</mi><mn>0</mn></msub></math>
 */
pub fn sha512_sig0(x: u64) -> u64 {
    u64::rotr(x, 1) ^ u64::rotr(x, 8) ^ u64::shr(x, 7)
}

/* FIPS-180-4 represents this function with GREEK SMALL LETTER SIGMA, sub 1:
 * <math><msub><mi>&#x03C3;</mi><mn>1</mn></msub></math>
 */
pub fn sha512_sig1(x: u64) -> u64 {
    u64::rotr(x, 19) ^ u64::rotr(x, 61) ^ u64::shr(x, 6)
}

pub fn sha512_update_bytes(hash: &mut[u64], m: &[u8]) {
    assert!(hash.len() == 8);
    assert!(m.len() == 128);

    // FIPS-180-4 SS 6.1.2.1 prepare message schedule
    let mut w = [0u64, .. 80];
    for t in range(0u, 16u) {
        w[t] = u64::from_be(m.slice(8*t, 8*t+8));

    }
    for t in range(16u, 80u) {
        w[t] = sha512_sig1(w[t-2]) + w[t-7] + sha512_sig0(w[t-15]) + w[t-16];
    }

    sha512_update_words(hash, w.as_slice());
}

pub fn sha512_update_words(hash: &mut[u64], w: &[u64]) {
    assert!(hash.len() == 8);
    assert!(w.len() == 80);

    // FIPS-180-4 SS 6.1.2.2 initialize working variables
    let mut temp1: u64;
    let mut temp2: u64;
    let mut a: u64 = hash[0];
    let mut b: u64 = hash[1];
    let mut c: u64 = hash[2];
    let mut d: u64 = hash[3];
    let mut e: u64 = hash[4];
    let mut f: u64 = hash[5];
    let mut g: u64 = hash[6];
    let mut h: u64 = hash[7];

    // FIPS-180-4 SS 4.1.1 functions
    // FIPS-180-4 SS 4.2.2 constants
    // FIPS-180-4 SS 6.1.2.3
    for t in range(0u, 80u) {
        temp1 = h + sha512_sigma1(e) + u64::bool3ary_202(e, f, g) + sha512_constant_pool[t] + w[t];
        temp2 = sha512_sigma0(a) + u64::bool3ary_232(a, b, c);
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

pub fn sha512_hash(msg: &mut Vec<u8>) -> ~[u64] {
    let mut hash = sha512_begin();

    sha512_pad(msg);

    for block in msg.as_slice().chunks(128) {
        sha512_update_bytes(hash, block);
    }

    return hash;
}

pub fn sha512_dump(hash: &[u64]) {
    println!("{:016x}{:016x}{:016x}{:016x}{:016x}{:016x}{:016x}{:016x}",
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
    let hash = sha512_hash(&mut msg);
    sha512_dump(hash);
}
