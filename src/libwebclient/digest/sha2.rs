use webclient::bits::u32;
use webclient::bits::u64;
use webclient::digest::types::HashAlgorithm;

//
// BEGIN constants
//

pub static sha224_initial_hash: [u32, .. 8] = [
    0xc1059ed8u32, // floor(mod(sqrt(2), 1)*2**32)
    0x367cd507u32, // floor(mod(sqrt(3), 1)*2**32)
    0x3070dd17u32, // floor(mod(sqrt(5), 1)*2**32)
    0xf70e5939u32, // floor(mod(sqrt(7), 1)*2**32)
    0xffc00b31u32, // floor(mod(sqrt(11), 1)*2**32)
    0x68581511u32, // floor(mod(sqrt(13), 1)*2**32)
    0x64f98fa7u32, // floor(mod(sqrt(17), 1)*2**32)
    0xbefa4fa4u32  // floor(mod(sqrt(19), 1)*2**32)
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

pub static sha384_initial_hash: [u64, .. 8] = [
    0xcbbb9d5dc1059ed8u64, // floor(mod(sqrt(2), 1)*2**64)
    0x629a292a367cd507u64, // floor(mod(sqrt(3), 1)*2**64)
    0x9159015a3070dd17u64, // floor(mod(sqrt(5), 1)*2**64)
    0x152fecd8f70e5939u64, // floor(mod(sqrt(7), 1)*2**64)
    0x67332667ffc00b31u64, // floor(mod(sqrt(11), 1)*2**64)
    0x8eb44a8768581511u64, // floor(mod(sqrt(13), 1)*2**64)
    0xdb0c2e0d64f98fa7u64, // floor(mod(sqrt(17), 1)*2**64)
    0x47b5481dbefa4fa4u64  // floor(mod(sqrt(19), 1)*2**64)
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

pub static sha512224_initial_hash: [u64, .. 8] = [
    0x8C3D37C819544DA2u64, // floor(mod(sqrt(2), 1)*2**64)
    0x73E1996689DCD4D6u64, // floor(mod(sqrt(3), 1)*2**64)
    0x1DFAB7AE32FF9C82u64, // floor(mod(sqrt(5), 1)*2**64)
    0x679DD514582F9FCFu64, // floor(mod(sqrt(7), 1)*2**64)
    0x0F6D2B697BD44DA8u64, // floor(mod(sqrt(11), 1)*2**64)
    0x77E36F7304C48942u64, // floor(mod(sqrt(13), 1)*2**64)
    0x3F9D85A86A1D36C8u64, // floor(mod(sqrt(17), 1)*2**64)
    0x1112E6AD91D692A1u64  // floor(mod(sqrt(19), 1)*2**64)
];

pub static sha512256_initial_hash: [u64, .. 8] = [
    0x22312194FC2BF72Cu64, // floor(mod(sqrt(2), 1)*2**64)
    0x9F555FA3C84C64C2u64, // floor(mod(sqrt(3), 1)*2**64)
    0x2393B86B6F53B151u64, // floor(mod(sqrt(5), 1)*2**64)
    0x963877195940EABDu64, // floor(mod(sqrt(7), 1)*2**64)
    0x96283EE2A88EFFE3u64, // floor(mod(sqrt(11), 1)*2**64)
    0xBE5E1E2553863992u64, // floor(mod(sqrt(13), 1)*2**64)
    0x2B0199FC2C85B8AAu64, // floor(mod(sqrt(17), 1)*2**64)
    0x0EB72DDC81C52CA2u64  // floor(mod(sqrt(19), 1)*2**64)
];

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

//
// END constants
//

//
// BEGIN functions
//

pub fn sha224_begin() -> ~[u32] {
    // FIPS-180-4 SS 5.3.3 initial hash value
    // FIPS-180-4 SS 6.2.1.1 initial hash value
    return ~[
        sha224_initial_hash[0],
        sha224_initial_hash[1],
        sha224_initial_hash[2],
        sha224_initial_hash[3],
        sha224_initial_hash[4],
        sha224_initial_hash[5],
        sha224_initial_hash[6],
        sha224_initial_hash[7]
    ];
}

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

//pub fn sha256_pad(msg: &mut Vec<u8>) {
//    // FIPS-180-4 SS 6.2.1.2 message is padded
//    let len = msg.len();
//    msg.push(0x80u8);
//    for _ in range(0, (55 - len) % 64) {
//        msg.push(0u8);
//    }
//    let pad = u64::to_be((len as u64)*8);
//    for t in range(0u, 8u) {
//        msg.push(pad[t]);
//    }
//}

pub fn sha256_update(hash: &mut[u32], m: &[u8]) {
    assert!(hash.len() == 8);
    assert!(m.len() == 64);

    /* FIPS-180-4 represents this function with GREEK CAPITAL LETTER SIGMA, sub 0:
     * <math><msub><mi>&#x03A3;</mi><mn>0</mn></msub></math>
     */
    fn sha256_sigma0(x: u32) -> u32 {
        u32::rotr(x, 2) ^ u32::rotr(x, 13) ^ u32::rotr(x, 22)
    }
    
    /* FIPS-180-4 represents this function with GREEK CAPITAL LETTER SIGMA, sub 1:
     * <math><msub><mi>&#x03A3;</mi><mn>1</mn></msub></math>
     */
    fn sha256_sigma1(x: u32) -> u32 {
        u32::rotr(x, 6) ^ u32::rotr(x, 11) ^ u32::rotr(x, 25)
    }
    
    /* FIPS-180-4 represents this function with GREEK SMALL LETTER SIGMA, sub 0:
     * <math><msub><mi>&#x03C3;</mi><mn>0</mn></msub></math>
     */
    fn sha256_sig0(x: u32) -> u32 {
        u32::rotr(x, 7) ^ u32::rotr(x, 18) ^ u32::shr(x, 3)
    }
    
    /* FIPS-180-4 represents this function with GREEK SMALL LETTER SIGMA, sub 1:
     * <math><msub><mi>&#x03C3;</mi><mn>1</mn></msub></math>
     */
    fn sha256_sig1(x: u32) -> u32 {
        u32::rotr(x, 17) ^ u32::rotr(x, 19) ^ u32::shr(x, 10)
    }

    // FIPS-180-4 SS 6.1.2.1 prepare message schedule
    let mut work = [0u32, ..64];
    let w = work.as_mut_slice();
    for t in range(0u, 16u) {
        w[t] = u32::from_be(m.slice(4*t, 4*t+4));

    }
    for t in range(16u, 64u) {
        w[t] = sha256_sig1(w[t-2]) + w[t-7] + sha256_sig0(w[t-15]) + w[t-16];
    }
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

//pub fn sha224_hash(msg: &mut Vec<u8>) -> ~[u32] {
//    let mut hash = sha224_begin();
//
//    sha256_pad(msg);
//
//    for block in msg.as_slice().chunks(64) {
//        sha256_update_bytes(hash, block);
//    }
//
//    return hash;
//}
//
//pub fn sha256_hash(msg: &mut Vec<u8>) -> ~[u32] {
//    let mut hash = sha256_begin();
//
//    sha256_pad(msg);
//
//    for block in msg.as_slice().chunks(64) {
//        sha256_update_bytes(hash, block);
//    }
//
//    return hash;
//}
//
//pub fn sha224_dump(hash: &[u32]) {
//    println!("{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}",
//             hash[0],
//             hash[1],
//             hash[2],
//             hash[3],
//             hash[4],
//             hash[5],
//             hash[6]);
//}
//
//pub fn sha256_dump(hash: &[u32]) {
//    println!("{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}",
//             hash[0],
//             hash[1],
//             hash[2],
//             hash[3],
//             hash[4],
//             hash[5],
//             hash[6],
//             hash[7]);
//}

pub fn sha384_begin() -> ~[u64] {
    // FIPS-180-4 SS 5.3.3 initial hash value
    // FIPS-180-4 SS 6.2.1.1 initial hash value
    return ~[
        sha384_initial_hash[0],
        sha384_initial_hash[1],
        sha384_initial_hash[2],
        sha384_initial_hash[3],
        sha384_initial_hash[4],
        sha384_initial_hash[5],
        sha384_initial_hash[6],
        sha384_initial_hash[7]
    ];
}

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

pub fn sha512224_begin() -> ~[u64] {
    // FIPS-180-4 SS 5.3.3 initial hash value
    // FIPS-180-4 SS 6.2.1.1 initial hash value
    return ~[
        sha512224_initial_hash[0],
        sha512224_initial_hash[1],
        sha512224_initial_hash[2],
        sha512224_initial_hash[3],
        sha512224_initial_hash[4],
        sha512224_initial_hash[5],
        sha512224_initial_hash[6],
        sha512224_initial_hash[7]
    ];
}

pub fn sha512256_begin() -> ~[u64] {
    // FIPS-180-4 SS 5.3.3 initial hash value
    // FIPS-180-4 SS 6.2.1.1 initial hash value
    return ~[
        sha512256_initial_hash[0],
        sha512256_initial_hash[1],
        sha512256_initial_hash[2],
        sha512256_initial_hash[3],
        sha512256_initial_hash[4],
        sha512256_initial_hash[5],
        sha512256_initial_hash[6],
        sha512256_initial_hash[7]
    ];
}

//pub fn sha512_pad(msg: &mut Vec<u8>) {
//    // FIPS-180-4 SS 6.2.1.2 message is padded
//    let len = msg.len();
//    msg.push(0x80u8);
//    for _ in range(0, (111 - len) % 128) {
//        msg.push(0u8);
//    }
//
//    // most significant u64 of the u128 size
//    for _ in range(0u, 8u) {
//        msg.push(0u8);
//    }
//
//    // least significant u64 of the u128 size
//    let pad = u64::to_be((len as u64)*8);
//    for t in range(0u, 8u) {
//        msg.push(pad[t]);
//    }
//}

pub fn sha512_update(hash: &mut[u64], m: &[u8]) {
    assert!(hash.len() == 8);
    assert!(m.len() == 128);

    /* FIPS-180-4 represents this function with GREEK CAPITAL LETTER SIGMA, sub 0:
     * <math><msub><mi>&#x03A3;</mi><mn>0</mn></msub></math>
     */
    fn sha512_sigma0(x: u64) -> u64 {
        u64::rotr(x, 28) ^ u64::rotr(x, 34) ^ u64::rotr(x, 39)
    }
    
    /* FIPS-180-4 represents this function with GREEK CAPITAL LETTER SIGMA, sub 1:
     * <math><msub><mi>&#x03A3;</mi><mn>1</mn></msub></math>
     */
    fn sha512_sigma1(x: u64) -> u64 {
        u64::rotr(x, 14) ^ u64::rotr(x, 18) ^ u64::rotr(x, 41)
    }
    
    /* FIPS-180-4 represents this function with GREEK SMALL LETTER SIGMA, sub 0:
     * <math><msub><mi>&#x03C3;</mi><mn>0</mn></msub></math>
     */
    fn sha512_sig0(x: u64) -> u64 {
        u64::rotr(x, 1) ^ u64::rotr(x, 8) ^ u64::shr(x, 7)
    }
    
    /* FIPS-180-4 represents this function with GREEK SMALL LETTER SIGMA, sub 1:
     * <math><msub><mi>&#x03C3;</mi><mn>1</mn></msub></math>
     */
    fn sha512_sig1(x: u64) -> u64 {
        u64::rotr(x, 19) ^ u64::rotr(x, 61) ^ u64::shr(x, 6)
    }

    // FIPS-180-4 SS 6.1.2.1 prepare message schedule
    let mut work = [0u64, .. 80];
    let w = work.as_mut_slice();
    for t in range(0u, 16u) {
        w[t] = u64::from_be(m.slice(8*t, 8*t+8));

    }
    for t in range(16u, 80u) {
        w[t] = sha512_sig1(w[t-2]) + w[t-7] + sha512_sig0(w[t-15]) + w[t-16];
    }
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

//
// END functions
//

//
// BEGIN implementations
//

pub struct SHA224 {
    msg_size: uint,
    state: ~[u32]
}

impl HashAlgorithm for SHA224 {

    fn get_iv(&self) -> ~[u8] {
        u32::to_be_v(sha224_begin())
    }

    fn get_hash(&self) -> ~[u8] {
        u32::to_be_v(self.state).slice_to(self.get_hash_size()).to_owned()
    }

    #[inline]
    fn get_hash_size(&self) -> uint {
        28
    }

    #[inline]
    fn get_block_size(&self) -> uint {
        64
    }

    fn get_message_size(&self) -> uint {
        self.msg_size
    }

    fn set_message_size(&mut self, msg_size: uint) {
        self.msg_size = msg_size;
    }

    fn clear(&mut self) {
        self.state = sha224_begin();
    }

    fn hash_block(&mut self, msg_block: &[u8]) {
        sha256_update(self.state, msg_block);
    }

    fn hash_last_block(&mut self, msg_piece: &[u8]) {
        let m = u64::pad_be_64(msg_piece, 0x80u8, self.msg_size);
        for block in m.chunks(64) {
            self.hash_block(block);
        }
    }
}

pub fn sha224_new() -> ~HashAlgorithm {
    ~SHA224{ msg_size: 0, state: sha224_begin() } as ~HashAlgorithm
}

pub fn sha224_hash(msg: &[u8]) -> ~[u8] {
    sha224_new().hash(msg)
}

pub struct SHA256 {
    msg_size: uint,
    state: ~[u32]
}

impl HashAlgorithm for SHA256 {

    fn get_iv(&self) -> ~[u8] {
        u32::to_be_v(sha256_begin())
    }

    fn get_hash(&self) -> ~[u8] {
        u32::to_be_v(self.state)
    }

    #[inline]
    fn get_hash_size(&self) -> uint {
        32
    }

    #[inline]
    fn get_block_size(&self) -> uint {
        64
    }

    fn get_message_size(&self) -> uint {
        self.msg_size
    }

    fn set_message_size(&mut self, msg_size: uint) {
        self.msg_size = msg_size;
    }

    fn clear(&mut self) {
        self.state = sha256_begin();
    }

    fn hash_block(&mut self, msg_block: &[u8]) {
        sha256_update(self.state, msg_block);
    }

    fn hash_last_block(&mut self, msg_piece: &[u8]) {
        let m = u64::pad_be_64(msg_piece, 0x80u8, self.msg_size);
        for block in m.chunks(64) {
            self.hash_block(block);
        }
    }
}

pub fn sha256_new() -> ~HashAlgorithm {
    ~SHA256{ msg_size: 0, state: sha256_begin() } as ~HashAlgorithm
}

pub fn sha256_hash(msg: &[u8]) -> ~[u8] {
    sha256_new().hash(msg)
}

pub struct SHA384 {
    msg_size: uint,
    state: ~[u64]
}

impl HashAlgorithm for SHA384 {

    fn get_iv(&self) -> ~[u8] {
        u64::to_be_v(sha384_begin())
    }

    fn get_hash(&self) -> ~[u8] {
        u64::to_be_v(self.state).slice_to(self.get_hash_size()).to_owned()
    }

    #[inline]
    fn get_hash_size(&self) -> uint {
        48
    }

    #[inline]
    fn get_block_size(&self) -> uint {
        128
    }

    fn get_message_size(&self) -> uint {
        self.msg_size
    }

    fn set_message_size(&mut self, msg_size: uint) {
        self.msg_size = msg_size;
    }

    fn clear(&mut self) {
        self.state = sha384_begin();
    }

    fn hash_block(&mut self, msg_block: &[u8]) {
        sha512_update(self.state, msg_block);
    }

    fn hash_last_block(&mut self, msg_piece: &[u8]) {
        let m = u64::pad_be_128(msg_piece, 0x80u8, self.msg_size);
        for block in m.chunks(128) {
            self.hash_block(block);
        }
    }
}

pub fn sha384_new() -> ~HashAlgorithm {
    ~SHA384{ msg_size: 0, state: sha384_begin() } as ~HashAlgorithm
}

pub fn sha384_hash(msg: &[u8]) -> ~[u8] {
    sha384_new().hash(msg)
}

pub struct SHA512 {
    msg_size: uint,
    state: ~[u64]
}

impl HashAlgorithm for SHA512 {

    fn get_iv(&self) -> ~[u8] {
        u64::to_be_v(sha512_begin())
    }

    fn get_hash(&self) -> ~[u8] {
        u64::to_be_v(self.state)
    }

    #[inline]
    fn get_hash_size(&self) -> uint {
        64
    }

    #[inline]
    fn get_block_size(&self) -> uint {
        128
    }

    fn get_message_size(&self) -> uint {
        self.msg_size
    }

    fn set_message_size(&mut self, msg_size: uint) {
        self.msg_size = msg_size;
    }

    fn clear(&mut self) {
        self.state = sha512_begin();
    }

    fn hash_block(&mut self, msg_block: &[u8]) {
        sha512_update(self.state, msg_block);
    }

    fn hash_last_block(&mut self, msg_piece: &[u8]) {
        let m = u64::pad_be_128(msg_piece, 0x80u8, self.msg_size);
        for block in m.chunks(128) {
            self.hash_block(block);
        }
    }
}

pub fn sha512_new() -> ~HashAlgorithm {
    ~SHA512{ msg_size: 0, state: sha512_begin() } as ~HashAlgorithm
}

pub fn sha512_hash(msg: &[u8]) -> ~[u8] {
    sha512_new().hash(msg)
}

pub struct SHA512224 {
    msg_size: uint,
    state: ~[u64]
}

impl HashAlgorithm for SHA512224 {

    fn get_iv(&self) -> ~[u8] {
        u64::to_be_v(sha512224_begin())
    }

    fn get_hash(&self) -> ~[u8] {
        u64::to_be_v(self.state).slice_to(self.get_hash_size()).to_owned()
    }

    #[inline]
    fn get_hash_size(&self) -> uint {
        28
    }

    #[inline]
    fn get_block_size(&self) -> uint {
        128
    }

    fn get_message_size(&self) -> uint {
        self.msg_size
    }

    fn set_message_size(&mut self, msg_size: uint) {
        self.msg_size = msg_size;
    }

    fn clear(&mut self) {
        self.state = sha512224_begin();
    }

    fn hash_block(&mut self, msg_block: &[u8]) {
        sha512_update(self.state, msg_block);
    }

    fn hash_last_block(&mut self, msg_piece: &[u8]) {
        let m = u64::pad_be_128(msg_piece, 0x80u8, self.msg_size);
        for block in m.chunks(128) {
            self.hash_block(block);
        }
    }
}

pub fn sha512224_new() -> ~HashAlgorithm {
    ~SHA512224{ msg_size: 0, state: sha512224_begin() } as ~HashAlgorithm
}

pub fn sha512224_hash(msg: &[u8]) -> ~[u8] {
    sha512224_new().hash(msg)
}

pub struct SHA512256 {
    msg_size: uint,
    state: ~[u64]
}

impl HashAlgorithm for SHA512256 {

    fn get_iv(&self) -> ~[u8] {
        u64::to_be_v(sha512256_begin())
    }

    fn get_hash(&self) -> ~[u8] {
        u64::to_be_v(self.state).slice_to(self.get_hash_size()).to_owned()
    }

    #[inline]
    fn get_hash_size(&self) -> uint {
        32
    }

    #[inline]
    fn get_block_size(&self) -> uint {
        128
    }

    fn get_message_size(&self) -> uint {
        self.msg_size
    }

    fn set_message_size(&mut self, msg_size: uint) {
        self.msg_size = msg_size;
    }

    fn clear(&mut self) {
        self.state = sha512256_begin();
    }

    fn hash_block(&mut self, msg_block: &[u8]) {
        sha512_update(self.state, msg_block);
    }

    fn hash_last_block(&mut self, msg_piece: &[u8]) {
        let m = u64::pad_be_128(msg_piece, 0x80u8, self.msg_size);
        for block in m.chunks(128) {
            self.hash_block(block);
        }
    }
}

pub fn sha512256_new() -> ~HashAlgorithm {
    ~SHA512256{ msg_size: 0, state: sha512256_begin() } as ~HashAlgorithm
}

pub fn sha512256_hash(msg: &[u8]) -> ~[u8] {
    sha512256_new().hash(msg)
}

//
// END implementations
//
