use webclient::bits::u32;
use webclient::bits::u64;
use webclient::digest::types::HashAlgorithm;

//::{bool3ary_57, bool3ary_150, bool3ary_202, bool3ary_228};

/*
 * MD5, (RFC 1321)
 * 
 * 
 */

///*
// * ch(x,y,z)
// *
// * MD5 calls this function f,
// * SHA1 calls this function ch.
// *
// * boolean operator number 202
// */
//fn ch(x: u32, y: u32, z: u32) -> u32 {
//    //return ((x)&(y)) | ((!x)&(z));
//    return z ^ (x & (y ^ z));
//}
//
///*
// * boolean operator number 57
// */
//fn fi(x: u32, y: u32, z: u32) -> u32 {
//    //return !((x & z) ^ y ^ z)
//    return y ^ (x | (!z));
//}
//
//fn parity(x: u32, y: u32, z: u32) -> u32 {
//    return x ^ y ^ z;
//}
//
//fn rotl(x: u32, y: uint) -> u32 {
//    return (x << y) | (x >> (32 - y));
//}
//
//fn bytes_as_le_u32(v: &[u8]) -> u32 {
//    return v[3] as u32 << 24 
//         | v[2] as u32 << 16 
//         | v[1] as u32 << 8 
//         | v[0] as u32;
//}
//
////fn bytes_as_be_u32(v: &[u8]) -> u32 {
////    return v[0] as u32 << 24 
////         | v[1] as u32 << 16 
////         | v[2] as u32 << 8 
////         | v[3] as u32;
////}
//
////fn be_u32_as_bytes(x: u32) -> ~[u8] {
////    return ~[((x >> 24)&0xff) as u8,
////             ((x >> 16)&0xff) as u8,
////             ((x >> 8)&0xff) as u8,
////             (x&0xff) as u8];
////}
//
////fn be_u64_as_bytes(x: u64) -> ~[u8] {
////    return ~[((x >> 56)&0xff) as u8,
////             ((x >> 48)&0xff) as u8,
////             ((x >> 40)&0xff) as u8,
////             ((x >> 32)&0xff) as u8,
////             ((x >> 24)&0xff) as u8,
////             ((x >> 16)&0xff) as u8,
////             ((x >> 8)&0xff) as u8,
////             (x&0xff) as u8];
////}
//
//fn le_u32_as_bytes(x: u32) -> ~[u8] {
//    return ~[((x)&0xff) as u8,
//             ((x >> 8)&0xff) as u8,
//             ((x >> 16)&0xff) as u8,
//             ((x >> 24)&0xff) as u8];
//}
//
//fn le_u64_as_bytes(x: u64) -> ~[u8] {
//    return ~[((x)&0xff) as u8,
//             ((x >> 8)&0xff) as u8,
//             ((x >> 16)&0xff) as u8,
//             ((x >> 24)&0xff) as u8,
//             ((x >> 32)&0xff) as u8,
//             ((x >> 40)&0xff) as u8,
//             ((x >> 48)&0xff) as u8,
//             ((x >> 56)&0xff) as u8];
//}
//
////fn bytes_as_le_u32(v: &[u8]) -> u32 {
////    return v[3] as u32 << 24 
////         | v[2] as u32 << 16 
////         | v[1] as u32 << 8 
////         | v[0] as u32;
////}
////
////fn le_u32_as_bytes(x: u32) -> ~[u8] {
////    return ~[(x&0xff) as u8,
////             ((x >> 8)&0xff) as u8,
////             ((x >> 16)&0xff) as u8,
////             ((x >> 24)&0xff) as u8];
////}
//

//
// BEGIN public API
//

pub static md5_block_size: uint = 64u; // in bytes

pub static md5_constant_pool: [u32, .. 64] = [
    0xd76aa478u32, // digits of floor(abs(sin(1))*2^32)
    0xe8c7b756u32, // digits of floor(abs(sin(2))*2^32)
    0x242070dbu32, // digits of floor(abs(sin(3))*2^32)
    0xc1bdceeeu32, // digits of floor(abs(sin(4))*2^32)
    0xf57c0fafu32, // digits of floor(abs(sin(5))*2^32)
    0x4787c62au32, // digits of floor(abs(sin(6))*2^32)
    0xa8304613u32, // digits of floor(abs(sin(7))*2^32)
    0xfd469501u32, // digits of floor(abs(sin(8))*2^32)
    0x698098d8u32, // digits of floor(abs(sin(9))*2^32)
    0x8b44f7afu32, // digits of floor(abs(sin(10))*2^32)
    0xffff5bb1u32, // digits of floor(abs(sin(11))*2^32)
    0x895cd7beu32, // digits of floor(abs(sin(12))*2^32)
    0x6b901122u32, // digits of floor(abs(sin(13))*2^32)
    0xfd987193u32, // digits of floor(abs(sin(14))*2^32)
    0xa679438eu32, // digits of floor(abs(sin(15))*2^32)
    0x49b40821u32, // digits of floor(abs(sin(16))*2^32)
    0xf61e2562u32, // digits of floor(abs(sin(17))*2^32)
    0xc040b340u32, // digits of floor(abs(sin(18))*2^32)
    0x265e5a51u32, // digits of floor(abs(sin(19))*2^32)
    0xe9b6c7aau32, // digits of floor(abs(sin(20))*2^32)
    0xd62f105du32, // digits of floor(abs(sin(21))*2^32)
    0x02441453u32, // digits of floor(abs(sin(22))*2^32)
    0xd8a1e681u32, // digits of floor(abs(sin(23))*2^32)
    0xe7d3fbc8u32, // digits of floor(abs(sin(24))*2^32)
    0x21e1cde6u32, // digits of floor(abs(sin(25))*2^32)
    0xc33707d6u32, // digits of floor(abs(sin(26))*2^32)
    0xf4d50d87u32, // digits of floor(abs(sin(27))*2^32)
    0x455a14edu32, // digits of floor(abs(sin(28))*2^32)
    0xa9e3e905u32, // digits of floor(abs(sin(29))*2^32)
    0xfcefa3f8u32, // digits of floor(abs(sin(30))*2^32)
    0x676f02d9u32, // digits of floor(abs(sin(31))*2^32)
    0x8d2a4c8au32, // digits of floor(abs(sin(32))*2^32)
    0xfffa3942u32, // digits of floor(abs(sin(33))*2^32)
    0x8771f681u32, // digits of floor(abs(sin(34))*2^32)
    0x6d9d6122u32, // digits of floor(abs(sin(35))*2^32)
    0xfde5380cu32, // digits of floor(abs(sin(36))*2^32)
    0xa4beea44u32, // digits of floor(abs(sin(37))*2^32)
    0x4bdecfa9u32, // digits of floor(abs(sin(38))*2^32)
    0xf6bb4b60u32, // digits of floor(abs(sin(39))*2^32)
    0xbebfbc70u32, // digits of floor(abs(sin(40))*2^32)
    0x289b7ec6u32, // digits of floor(abs(sin(41))*2^32)
    0xeaa127fau32, // digits of floor(abs(sin(42))*2^32)
    0xd4ef3085u32, // digits of floor(abs(sin(43))*2^32)
    0x04881d05u32, // digits of floor(abs(sin(44))*2^32)
    0xd9d4d039u32, // digits of floor(abs(sin(45))*2^32)
    0xe6db99e5u32, // digits of floor(abs(sin(46))*2^32)
    0x1fa27cf8u32, // digits of floor(abs(sin(47))*2^32)
    0xc4ac5665u32, // digits of floor(abs(sin(48))*2^32)
    0xf4292244u32, // digits of floor(abs(sin(49))*2^32)
    0x432aff97u32, // digits of floor(abs(sin(50))*2^32)
    0xab9423a7u32, // digits of floor(abs(sin(51))*2^32)
    0xfc93a039u32, // digits of floor(abs(sin(52))*2^32)
    0x655b59c3u32, // digits of floor(abs(sin(53))*2^32)
    0x8f0ccc92u32, // digits of floor(abs(sin(54))*2^32)
    0xffeff47du32, // digits of floor(abs(sin(55))*2^32)
    0x85845dd1u32, // digits of floor(abs(sin(56))*2^32)
    0x6fa87e4fu32, // digits of floor(abs(sin(57))*2^32)
    0xfe2ce6e0u32, // digits of floor(abs(sin(58))*2^32)
    0xa3014314u32, // digits of floor(abs(sin(59))*2^32)
    0x4e0811a1u32, // digits of floor(abs(sin(60))*2^32)
    0xf7537e82u32, // digits of floor(abs(sin(61))*2^32)
    0xbd3af235u32, // digits of floor(abs(sin(62))*2^32)
    0x2ad7d2bbu32, // digits of floor(abs(sin(63))*2^32)
    0xeb86d391u32  // digits of floor(abs(sin(64))*2^32)
];

pub static md5_initial_hash: [u32, .. 4] = [
    0x67452301u32, // digits are (34*n + 1) where n = 3, 2, 1, 0
    0xefcdab89u32, // digits 
    0x98badcfeu32, // digits 
    0x10325476u32  // digits 
];

pub static md5_shift_amounts: [uint, .. 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
    5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
    4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
    6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21
];

pub fn md5_begin() -> ~[u32] {
    // FIPS-180-4 SS 6.1.1.1 initial hash value
    return ~[
        md5_initial_hash[0],
        md5_initial_hash[1],
        md5_initial_hash[2],
        md5_initial_hash[3]
    ];
}

pub fn md5_update(hash: &mut[u32], m: &[u8]) {
    assert!(hash.len() == 4);
    assert!(m.len() == 64);

    // FIPS-180-4 SS 6.1.2.1 prepare message schedule
    let mut work = [0u32, ..16];
    for t in range(0u, 16u) {
        work[t] = u32::from_le(m.slice(4*t, 4*t+4)); 
    }

    let w = work.as_slice();
    assert!(w.len() == 16);

    // FIPS-180-4 SS 6.1.2.2 initialize working variables
    //let mut round: uint;
    let mut btemp: u32;
    let mut dtemp: u32;
    //let mut s: uint;
    let mut r: uint;
    let mut a: u32 = hash[0];
    let mut b: u32 = hash[1];
    let mut c: u32 = hash[2];
    let mut d: u32 = hash[3];

    // FIPS-180-4 SS 4.1.1 functions
    // FIPS-180-4 SS 4.2.1 constants
    // FIPS-180-4 SS 6.1.2.3
    for t in range(0u, 16u) {
        //round = t >> 4;
        // [abcd r s t]
        r = t;
        btemp = u32::rotl(a + w[r] + md5_constant_pool[t] + u32::bool3ary_202(b, c, d), md5_shift_amounts[t]);
        dtemp = d; d = c; c = b;
        b += btemp;
        a = dtemp;
    }
    for t in range(16u, 32u) {
        // [abcd r s t]
        r = (5*t + 1)%16;
        btemp = u32::rotl(a + w[r] + md5_constant_pool[t] + u32::bool3ary_228(b, c, d), md5_shift_amounts[t]);
        dtemp = d; d = c; c = b;
        b += btemp;
        a = dtemp;
    }
    for t in range(32u, 48u) {
        // [abcd r s t]
        r = (3*t + 5)%16;
        btemp = u32::rotl(a + w[r] + md5_constant_pool[t] + u32::bool3ary_150(b, c, d), md5_shift_amounts[t]);
        dtemp = d; d = c; c = b;
        b += btemp;
        a = dtemp;
    }
    for t in range(48u, 64u) {
        // [abcd r s t]
        r = (7*t)%16;
        btemp = u32::rotl(a + w[r] + md5_constant_pool[t] + u32::bool3ary_57(b, c, d), md5_shift_amounts[t]);
        dtemp = d; d = c; c = b;
        b += btemp;
        a = dtemp;
    }

    // FIPS-180-4 SS 6.1.2.4
    hash[0] += a;
    hash[1] += b;
    hash[2] += c;
    hash[3] += d;
}

//pub fn md5_dump(hash: &[u32]) {
//    for word_i in range(0u, hash.len()) {
//        let word = hash[word_i];
//        let byteslice = u32::to_le(word);
//        for byte_i in range(0u, byteslice.len()) {
//            let byte = byteslice[byte_i];
//            print!("{:02x}", byte);
//        }
//    }
//    println!("");
//}

//
// END public API
//

//pub fn main() {
//    let mut reader = stdin();
//    let mut msg = reader.read_to_end().unwrap();
//    let hash = md5_hash(&mut msg);
//    md5_dump(hash);
//}


pub struct MD5 {
	block_size: uint,
    msg_length: uint,
	state: ~[u32]
}

impl HashAlgorithm for MD5 {

    fn clear(&mut self) {
        self.state = md5_begin();
    }

    fn hash(&mut self, msg: &[u8]) -> ~[u8] {
        self.clear();

        for block in msg.chunks(self.block_size) {
            self.msg_length += block.len();
            if self.msg_length == self.block_size {
                self.hash_block(block);
            } else {
                self.hash_last_block(block);
            }
        }

        self.get_hash()
    }

    fn hash_block(&mut self, block: &[u8]) {
        md5_update(self.state, block);
    }

    fn hash_last_block(&mut self, piece: &[u8]) {
        let m = u64::pad_le_64(piece, 0x80u8, self.msg_length);
        for block in m.chunks(self.block_size) {
            self.hash_block(block);
        }
    }

    fn get_hash(&self) -> ~[u8] {
        use std::slice;

        let mut ret = slice::with_capacity(self.block_size);
        for word_i in range(0u, self.block_size/4) {
            let word = self.state[word_i];
            let byteslice = u32::to_le(word);
            for byte_i in range(0u, byteslice.len()) {
                let byte = byteslice[byte_i];
                ret[4*word_i + byte_i] = byte;
            }
        }
        ret
    }
}

pub fn md5_new() -> ~HashAlgorithm {
    ~MD5{
        block_size: 64, // in bytes
        msg_length: 0, // in bytes
        state: md5_begin()
    } as ~HashAlgorithm
}

pub fn md5_hash(msg: &[u8]) -> ~[u8] {
    md5_new().hash(msg)
}
