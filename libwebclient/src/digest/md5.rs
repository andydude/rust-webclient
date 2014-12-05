use webclient::bits::u32;
use webclient::bits::u64;
use webclient::digest::types::HashAlgorithm;

pub static MD5_CONSTANT_POOL: [u32, .. 64] = [
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

pub static MD5_INITIAL_HASH: [u32, .. 4] = [
    0x67452301u32, // digits are (34*n + 1) where n = 3, 2, 1, 0
    0xefcdab89u32, // digits 
    0x98badcfeu32, // digits 
    0x10325476u32  // digits 
];

pub static MD5_SHIFT_AMOUNTS: [uint, .. 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
    5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
    4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
    6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21
];

pub fn md5_update(hash: &mut Vec<u32>, w: &mut Vec<u32>, m: Vec<u8>) {
    assert!(hash.len() == 4);
    assert!(w.len() == 16);
    assert!(m.len() == 64);

    // FIPS-180-4 SS 6.1.2.1 prepare message schedule
    for t in range(0u, 16u) {
        w[t] = u32::from_le(m.slice(4*t, 4*t+4)); 
    }

    // FIPS-180-4 SS 6.1.2.2 initialize working variables
    //let mut round: uint;
    let mut atemp: u32;
    let mut btemp: u32;
    let mut r: uint;
    let mut a: u32 = hash[0];
    let mut b: u32 = hash[1];
    let mut c: u32 = hash[2];
    let mut d: u32 = hash[3];

    // FIPS-180-4 SS 4.1.1 functions
    // FIPS-180-4 SS 4.2.1 constants
    // FIPS-180-4 SS 6.1.2.3
    for t in range(0u, 16u) {
        r = t;
        btemp = u32::rotl(a + w[r] + MD5_CONSTANT_POOL[t] + u32::bool3ary_202(b, c, d), MD5_SHIFT_AMOUNTS[t]);
        atemp = d; d = c; c = b;
        b += btemp;
        a = atemp;
    }
    for t in range(16u, 32u) {
        r = (5*t + 1)%16;
        btemp = u32::rotl(a + w[r] + MD5_CONSTANT_POOL[t] + u32::bool3ary_228(b, c, d), MD5_SHIFT_AMOUNTS[t]);
        atemp = d; d = c; c = b;
        b += btemp;
        a = atemp;
    }
    for t in range(32u, 48u) {
        r = (3*t + 5)%16;
        btemp = u32::rotl(a + w[r] + MD5_CONSTANT_POOL[t] + u32::bool3ary_150(b, c, d), MD5_SHIFT_AMOUNTS[t]);
        atemp = d; d = c; c = b;
        b += btemp;
        a = atemp;
    }
    for t in range(48u, 64u) {
        r = (7*t)%16;
        btemp = u32::rotl(a + w[r] + MD5_CONSTANT_POOL[t] + u32::bool3ary_57(b, c, d), MD5_SHIFT_AMOUNTS[t]);
        atemp = d; d = c; c = b;
        b += btemp;
        a = atemp;
    }

    // FIPS-180-4 SS 6.1.2.4
    hash[0] += a;
    hash[1] += b;
    hash[2] += c;
    hash[3] += d;
}

pub struct MD5 {
    msg_size: uint,
	hash_state: Vec<u32>,
	work_state: Vec<u32>
}

impl HashAlgorithm for MD5 {

    #[inline]
    fn get_hash(&self) -> Vec<u8> {
        u32::to_le_v(self.hash_state.as_slice())
    }

    #[inline]
    fn get_hash_size(&self) -> uint {
        16
    }

    #[inline]
    fn get_block_size(&self) -> uint {
        64
    }

    #[inline]
    fn get_message_size(&self) -> uint {
        self.msg_size
    }

    #[inline]
    fn set_message_size(&mut self, msg_size: uint) {
        self.msg_size = msg_size;
    }

    #[inline]
    fn clear(&mut self) {
        self.hash_state = MD5_INITIAL_HASH.to_vec();
        self.work_state = Vec::from_elem(16, 0u32);
    }

    #[inline]
	fn hash_block(&mut self, msg_block: Vec<u8>) {
        md5_update(&mut self.hash_state, &mut self.work_state, msg_block);
    }

    #[inline]
	fn hash_last_block(&mut self, msg_piece: Vec<u8>) {
        let m = u64::pad_le_64(msg_piece, 0x80u8, self.msg_size);
        for chunk in m.chunks(64) {
            let block = chunk.to_vec();
            self.hash_block(block);
        }
    }
}

pub fn md5_new() -> MD5 {
    MD5{
        msg_size: 0,
        hash_state: MD5_INITIAL_HASH.to_vec(),
        work_state: Vec::from_elem(16, 0u32)
    }
}

pub fn md5_hash(msg: Vec<u8>) -> Vec<u8> {
    md5_new().hash(msg)
}
