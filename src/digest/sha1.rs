use bits::u32;
use bits::u64;
use digest::types::HashAlgorithm;

//
// BEGIN public API
//

pub static SHA1_INITIAL_HASH: [u32, .. 5] = [
	0x67452301u32, // digits are (34*n + 1) where n = 3, 2, 1, 0
	0xefcdab89u32, // digits are (34*n + 1) where n = 7, 6, 5, 4
	0x98badcfeu32, // digits are (34*n + 16) where n = 4, 5, 6, 7
	0x10325476u32, // digits are (34*n + 16) where n = 0, 1, 2, 3
	0xc3d2e1f0u32  // digits are (15*n) where n = 13, 14, 15, 16
];

pub static SHA1_CONSTANT_POOL: [u32, .. 4] = [
	0x5a827999u32, // digits of floor(sqrt(2)*2^30)
	0x6ed9eba1u32, // digits of floor(sqrt(3)*2^30)
	0x8f1bbcdcu32, // digits of floor(sqrt(5)*2^30)
	0xca62c1d6u32  // digits of floor(sqrt(10)*2^30)
];

pub fn sha1_update(hash: &mut Vec<u32>, w: &mut Vec<u32>, m: Vec<u8>) {
    assert!(hash.len() == 5);
    assert!(w.len() == 80);
    assert!(m.len() == 64);

    // FIPS-180-4 SS 6.1.2.1 prepare message schedule
    for t in range(0u, 16u) {
        w[t] = u32::from_be(m.slice(4*t, 4*t+4)); 
    }
    for t in range(16u, 80u) {
        w[t] = u32::rotl(w[t-3]^w[t-8]^w[t-14]^w[t-16], 1);
    }

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
        temp = u32::rotl(a, 5) + e + w[t] + SHA1_CONSTANT_POOL[0] + u32::bool3ary_202(b, c, d);
        e = d; d = c;
        c = u32::rotl(b, 30); 
        b = a; a = temp;
    }
    for t in range(20u, 40u) {
        temp = u32::rotl(a, 5) + e + w[t] + SHA1_CONSTANT_POOL[1] + u32::bool3ary_150(b, c, d);
        e = d; d = c;
        c = u32::rotl(b, 30); 
        b = a; a = temp;
    }
    for t in range(40u, 60u) {
        temp = u32::rotl(a, 5) + e + w[t] + SHA1_CONSTANT_POOL[2] + u32::bool3ary_232(b, c, d);
        e = d; d = c;
        c = u32::rotl(b, 30); 
        b = a; a = temp;
    }
    for t in range(60u, 80u) {
        temp = u32::rotl(a, 5) + e + w[t] + SHA1_CONSTANT_POOL[3] + u32::bool3ary_150(b, c, d);
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

pub struct SHA1 {
    msg_size: uint,
	hash_state: Vec<u32>,
	work_state: Vec<u32>
}

impl HashAlgorithm for SHA1 {

    fn get_hash(&self) -> Vec<u8> {
        u32::to_be_v(self.hash_state.as_slice())
    }

    #[inline]
    fn get_hash_size(&self) -> uint {
        20
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
        self.hash_state = SHA1_INITIAL_HASH.to_vec();
        self.work_state = Vec::from_elem(80, 0u32);
    }

	fn hash_block(&mut self, msg_block: Vec<u8>) {
        sha1_update(&mut self.hash_state, &mut self.work_state, msg_block);
    }

	fn hash_last_block(&mut self, msg_piece: Vec<u8>) {
        let m = u64::pad_be_64(msg_piece, 0x80u8, self.msg_size);
        for block in m.chunks(64) {
            self.hash_block(block.to_vec());
        }
    }
}

pub fn sha1_new() -> SHA1 {
    SHA1{msg_size: 0,
         hash_state: SHA1_INITIAL_HASH.to_vec(),
         work_state: Vec::from_elem(80, 0u32) }
}

pub fn sha1_hash(msg: Vec<u8>) -> Vec<u8> {
    sha1_new().hash(msg)
}
