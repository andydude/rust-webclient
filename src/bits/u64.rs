
pub fn pad_le_64(msg: Vec<u8>, bit: u8, length: uint) -> Vec<u8> {
    // FIPS-180-4 SS 6.1.1.2 message is padded
    let mut ret = Vec::new();
    for i in range(0, msg.len()) {
        ret.push(msg[i]);
    }

    // primarily for implementing MD5
    ret.push(bit);
    for _ in range(0, (55 - length) % 64) {
        ret.push(0u8);
    }

    // big-endian u64 size
    let pad = to_le((length as u64)*8);
    for i in range(0, pad.len()) {
        ret.push(pad[i]);
    }
    ret
}

//pub fn pad_le_64(msg: Vec<u8>, bit: u8, length: uint) -> Vec<u8> {
//    let mut ret: Vec<u8> = msg.clone().to_owned();
//    slice::append(ret, msg);
//    slice::append_one(ret, bit);
//    for _ in range(0, (55 - length) % 64) {
//        slice::append_one(ret, 0u8);
//    }
//    slice::append(ret, to_le((length as u64)*8));
//    ret
//}

pub fn pad_be_64(msg: Vec<u8>, bit: u8, length: uint) -> Vec<u8> {
    // FIPS-180-4 SS 6.1.1.2 message is padded
    let mut ret = Vec::new();
    for i in range(0, msg.len()) {
        ret.push(msg[i]);
    }

    // primarily for implementing SHA1, SHA224, SHA256
    ret.push(bit);
    for _ in range(0, (55 - length) % 64) {
        ret.push(0u8);
    }

    // big-endian u64 size
    let pad = to_be((length as u64)*8);
    for i in range(0, pad.len()) {
        ret.push(pad[i]);
    }
    ret
}

pub fn pad_be_128(msg: Vec<u8>, bit: u8, length: uint) -> Vec<u8> {
    // FIPS-180-4 SS 6.1.1.2 message is padded
    let mut ret = Vec::new();
    for i in range(0, msg.len()) {
        ret.push(msg[i]);
    }

    // primarily for implementing SHA384, SHA512, SHA512224, SHA512256
    ret.push(bit);
    for _ in range(0, (111 - length) % 128) {
        ret.push(0u8);
    }

    // most significant u64 of the u128 size
    for _ in range(0u, 8u) {
        ret.push(0u8);
    }

    // big-endian u64 size
    let pad = to_be((length as u64)*8);
    for i in range(0, pad.len()) {
        ret.push(pad[i]);
    }
    ret
}

//pub fn pad_be_128(msg: Vec<u8>, bit: u8, length: uint) -> Vec<u8> {
//    // FIPS-180-4 SS 6.1.1.2 message is padded
//    let ret = Vec::new();
//    ret.push_bytes(msg);
//
//    // FIPS-180-4 SS 6.2.1.2 message is padded
//    ret.push_bytes(&[bit]);
//    for _ in range(0, (111 - length) % 128) {
//        ret.push_bytes(&[0u8]);
//    }
//
//    // most significant u64 of the u128 size
//    for _ in range(0u, 8u) {
//        ret.push_bytes(&[0u8]);
//    }
//
//    // least significant u64 of the u128 size
//    let pad = to_be((length as u64)*8);
//    ret.push_bytes(pad);
//}


#[inline]
pub fn rotl(x: u64, y: uint) -> u64 {
    return (x << y) | (x >> (64 - y));
}

#[inline]
pub fn rotr(x: u64, y: uint) -> u64 {
    return (x >> y) | (x << (64 - y));
}

#[inline]
pub fn shl(x: u64, y: uint) -> u64 {
    return x << y;
}

#[inline]
pub fn shr(x: u64, y: uint) -> u64 {
    return x >> y;
}

#[inline]
pub fn from_be(v: &[u8]) -> u64 {
    return v[0] as u64 << 56 
         | v[1] as u64 << 48 
         | v[2] as u64 << 40
         | v[3] as u64 << 32
         | v[4] as u64 << 24 
         | v[5] as u64 << 16 
         | v[6] as u64 << 8 
         | v[7] as u64;
}

#[inline]
pub fn from_le(v: &[u8]) -> u64 {
    return v[7] as u64 << 56 
         | v[6] as u64 << 48 
         | v[5] as u64 << 40
         | v[4] as u64 << 32
         | v[3] as u64 << 24 
         | v[2] as u64 << 16 
         | v[1] as u64 << 8 
         | v[0] as u64;
}

#[inline]
pub fn to_be(x: u64) -> Vec<u8> {
    return vec![((x >> 56)&0xff) as u8,
                ((x >> 48)&0xff) as u8,
                ((x >> 40)&0xff) as u8,
                ((x >> 32)&0xff) as u8,
                ((x >> 24)&0xff) as u8,
                ((x >> 16)&0xff) as u8,
                ((x >> 8)&0xff) as u8,
                (x&0xff) as u8];
}

#[inline]
pub fn to_le(x: u64) -> Vec<u8> {
    return vec![((x)&0xff) as u8,
                ((x >> 8)&0xff) as u8,
                ((x >> 16)&0xff) as u8,
                ((x >> 24)&0xff) as u8,
                ((x >> 32)&0xff) as u8,
                ((x >> 40)&0xff) as u8,
                ((x >> 48)&0xff) as u8,
                ((x >> 56)&0xff) as u8];
}

#[inline]
pub fn from_be_v(v: &[u8]) -> Vec<u64> {
    let mut ret = Vec::new();
    for bytes in v.chunks(4) {
        let word = from_be(bytes);
        ret.push(word);
    }
    ret
}

#[inline]
pub fn from_le_v(v: &[u8]) -> Vec<u64> {
    let mut ret = Vec::new();
    for bytes in v.chunks(4) {
        let word = from_le(bytes);
        ret.push(word);
    }
    ret
}

#[inline]
pub fn to_be_v(x: &[u64]) -> Vec<u8> {
    let mut ret = Vec::new();
    for word in x.iter() {
        let bytes = to_be(*word);
        for byte in bytes.iter() {
            ret.push(*byte);
        }
    }
    ret
}

#[inline]
pub fn to_le_v(x: &[u64]) -> Vec<u8> {
    let mut ret = Vec::new();
    for word in x.iter() {
        let bytes = to_le(*word);
        for byte in bytes.iter() {
            ret.push(*byte);
        }
    }
    ret
}


// 2-ary boolean functions

pub fn bool2ary_1(x: u64, y: u64) -> u64 { !(x | y) }	// 2, nor
pub fn bool2ary_2(x: u64, y: u64) -> u64 { !x & y } 	// 2, nif, "NotImpliedBy"
pub fn bool2ary_4(x: u64, y: u64) -> u64 { x & !y }		// 2, nimplies, andn, "NotImplies"
pub fn bool2ary_6(x: u64, y: u64) -> u64 { x ^ y }		// 2, xor, "ExclusiveOr", --half
pub fn bool2ary_7(x: u64, y: u64) -> u64 { !(x & y) }	// 2, nand
pub fn bool2ary_8(x: u64, y: u64) -> u64 { x & y }		// 2, and
pub fn bool2ary_9(x: u64, y: u64) -> u64 { !(x ^ y) }	// 2, eqv, nxor, "Equivalent", --half
pub fn bool2ary_11(x: u64, y: u64) -> u64 { !x | y }	// 2, implies, "Implies"
pub fn bool2ary_13(x: u64, y: u64) -> u64 { x | !y }	// 2, if, "ImpliedBy"
pub fn bool2ary_14(x: u64, y: u64) -> u64 { x | y }		// 2, or, ior, "InclusiveOr"

// 3-ary boolean functions

// the boolean function number is the same as Mathematica.
// unary and binary functions have been ommitted from this list.
// for more information on a particular boolean function, see also:
// http://www.wolframalpha.com/input/?i=BooleanFunction[232,3]
// where 232 refers to bool3ary_232(x, y, z) in this file.

// there are only 70 3-ary boolean functions which have the property
// that half of their inputs yield false and the other half yield true.
// these functions numbers are given by [A014312] https://oeis.org/A014312
// 15, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58, 60, 71, 75, 77, 78, 83, 85, 
// 86, 89, 90, 92, 99, 101, 102, 105, 106, 108, 113, 114, 116, 120, 135, 139, 141, 142, 
// 147, 149, 150, 153, 154, 156, 163, 165, 166, 169, 170, 172, 177, 178, 180, 184, 195, 
// 197, 198, 201, 202, 204, 209, 210, 212, 216, 225, 226, 228, 232, 240

pub fn bool3ary_1(x: u64, y: u64, z: u64) -> u64 { !(x | y | z) }					// 3, nor
pub fn bool3ary_2(x: u64, y: u64, z: u64) -> u64 { !(x | y | !z) }					// 3, --mostly-false
pub fn bool3ary_4(x: u64, y: u64, z: u64) -> u64 { !(x | !y | z) }					// 3, --mostly-false
pub fn bool3ary_6(x: u64, y: u64, z: u64) -> u64 { !x & (y ^ z) }					// 3, --mostly-false
pub fn bool3ary_7(x: u64, y: u64, z: u64) -> u64 { !(x | (y & z)) }					// 3, --mostly-false
pub fn bool3ary_8(x: u64, y: u64, z: u64) -> u64 { !x & y & z }						// 3, --mostly-false
pub fn bool3ary_9(x: u64, y: u64, z: u64) -> u64 { !(x | (y ^ z)) }					// 3, --mostly-false
pub fn bool3ary_11(x: u64, y: u64, z: u64) -> u64 { !x & (!y | z) }					// 3, --mostly-false
pub fn bool3ary_13(x: u64, y: u64, z: u64) -> u64 { !x & (y | !z) }					// 3, --mostly-false
pub fn bool3ary_14(x: u64, y: u64, z: u64) -> u64 { !x & (y | z) }					// 3, --mostly-false
pub fn bool3ary_16(x: u64, y: u64, z: u64) -> u64 { x & !(y | z) }					// 3, --mostly-false
pub fn bool3ary_18(x: u64, y: u64, z: u64) -> u64 { !y & (x ^ z) }					// 3, --mostly-false
pub fn bool3ary_19(x: u64, y: u64, z: u64) -> u64 { !(y | (x & z)) }				// 3, --mostly-false
pub fn bool3ary_20(x: u64, y: u64, z: u64) -> u64 { !z & (x ^ y) }					// 3, --mostly-false
pub fn bool3ary_21(x: u64, y: u64, z: u64) -> u64 { !(z | (x & y)) }				// 3, --mostly-false
pub fn bool3ary_22(x: u64, y: u64, z: u64) -> u64 { x^y^z^(x&y&z) }					// 3, --mostly-false
pub fn bool3ary_23(x: u64, y: u64, z: u64) -> u64 { !bool3ary_232(x, y, z) }		// 3, nmajority, --half
//pub fn bool3ary_24(x: u64, y: u64, z: u64) -> u64 { (x ^ y) & (x ^ z) }			// 3, --mostly-false
//pub fn bool3ary_25(x: u64, y: u64, z: u64) -> u64 { !((x & y) | (y ^ z)) }		// 3, --mostly-false
//pub fn bool3ary_26(x: u64, y: u64, z: u64) -> u64 { x^z^(x & y)^(x&y&z) }			// 3, --mostly-false
//pub fn bool3ary_27(x: u64, y: u64, z: u64) -> u64 { (!x & z) | !(y | z) }			// 3, --half
//pub fn bool3ary_28(x: u64, y: u64, z: u64) -> u64 { x^y^(x & z)^(x&y&z) }			// 3, --mostly-false
//pub fn bool3ary_29(x: u64, y: u64, z: u64) -> u64 { (!x & y) ^ !(y | z) }			// 3, --half
//pub fn bool3ary_30(x: u64, y: u64, z: u64) -> u64 { x ^ (y | z) }					// 3, --half
//pub fn bool3ary_31(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-true
//pub fn bool3ary_32(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-false
//pub fn bool3ary_33(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-false
//pub fn bool3ary_34(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-false
//pub fn bool3ary_35(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-false
//pub fn bool3ary_36(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-false
//pub fn bool3ary_37(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-false
//pub fn bool3ary_38(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-false
//pub fn bool3ary_39(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --half
//pub fn bool3ary_40(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-false
//pub fn bool3ary_41(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-false
//pub fn bool3ary_42(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-false
//pub fn bool3ary_43(x: u64, y: u64, z: u64) -> u64 { bool3ary_232(!x, !y, z) }			// 3, --half
//pub fn bool3ary_44(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-false
//pub fn bool3ary_45(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --half
//pub fn bool3ary_46(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --half
//pub fn bool3ary_47(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-true
//pub fn bool3ary_48(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-false
//pub fn bool3ary_49(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-false
//pub fn bool3ary_50(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-false
//pub fn bool3ary_52(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_53(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_54(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_55(x: u64, y: u64, z: u64) -> u64 {0}			// 3, --mostly-true
//pub fn bool3ary_56(x: u64, y: u64, z: u64) -> u64 {0}
pub fn bool3ary_57(x: u64, y: u64, z: u64) -> u64 { y ^ (x | !z) }			// 3, MD5_I, --half
//pub fn bool3ary_58(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_59(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_60(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_61(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_62(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_63(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_64(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_65(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_66(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_67(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_68(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_69(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_70(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_71(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_72(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_73(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_74(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_75(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_76(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_77(x: u64, y: u64, z: u64) -> u64 { bool3ary_232(!x, y, !z) }			// 3, --half
//pub fn bool3ary_78(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_79(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_80(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_81(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_82(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_83(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_84(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_86(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_87(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_88(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_89(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_90(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_91(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_92(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_93(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_94(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_95(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_96(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_97(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_98(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_99(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_100(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_101(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_102(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_103(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_104(x: u64, y: u64, z: u64) -> u64 {0}
pub fn bool3ary_105(x: u64, y: u64, z: u64) -> u64 { !(x ^ y ^ z) }			// 3, nxor, nparity, --half
//pub fn bool3ary_106(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_107(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_108(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_109(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_110(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_111(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_112(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_113(x: u64, y: u64, z: u64) -> u64 { bool3ary_232(x, !y, !z) }			// 3, --half
//pub fn bool3ary_114(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_115(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_116(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_117(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_118(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_119(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_120(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_121(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_122(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_123(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_124(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_125(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_126(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_127(x: u64, y: u64, z: u64) -> u64 { !(x & y & z) }			// 3, nand
//pub fn bool3ary_128(x: u64, y: u64, z: u64) -> u64 { x & y & z }			// 3, and
//pub fn bool3ary_129(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_130(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_131(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_132(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_133(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_134(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_135(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_136(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_137(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_138(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_139(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_140(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_141(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_142(x: u64, y: u64, z: u64) -> u64 { bool3ary_232(!x, y, z) }			// 3, --half
//pub fn bool3ary_143(x: u64, y: u64, z: u64) -> u64 { !x | (y & z) }			// 3, ranand, --mostly-true
//pub fn bool3ary_144(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_145(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_146(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_147(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_148(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_149(x: u64, y: u64, z: u64) -> u64 {0}			// --half
pub fn bool3ary_150(x: u64, y: u64, z: u64) -> u64 { x ^ y ^ z }		// 3, xor, parity, MD5_H, SHA1_F1, SHA1_F3, --half
//pub fn bool3ary_151(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_152(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_153(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_154(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_155(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_156(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_157(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_158(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_159(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_160(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_161(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_162(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_163(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_164(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_165(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_166(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_167(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_168(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_169(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_171(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_172(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_173(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_174(x: u64, y: u64, z: u64) -> u64 { z | (y & !x) } // 3, laifimplies, --mostly-true
//pub fn bool3ary_175(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_176(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_177(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_178(x: u64, y: u64, z: u64) -> u64 { bool3ary_232(x, !y, z) }			// 3, --half
//pub fn bool3ary_179(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_180(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_181(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_182(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_183(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_184(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_185(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_186(x: u64, y: u64, z: u64) -> u64 { !(!x | y) | z }			// 3, laimplies, --mostly-true
//pub fn bool3ary_187(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_188(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_189(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_190(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_191(x: u64, y: u64, z: u64) -> u64 { !x | !y | z }				// 3, raimplies, --mostly-true
//pub fn bool3ary_192(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_193(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_194(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_195(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_196(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_197(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_198(x: u64, y: u64, z: u64) -> u64 {0}			// --half
//pub fn bool3ary_199(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_200(x: u64, y: u64, z: u64) -> u64 { (x | z) & y }					// 3
//pub fn bool3ary_201(x: u64, y: u64, z: u64) -> u64 { !((x & z)^x^y^z) }				// 3, --half
pub fn bool3ary_202(x: u64, y: u64, z: u64) -> u64 { z ^ (x & (y ^ z)) }			// 3, MD5_F, SHA1_F0, --half
//pub fn bool3ary_203(x: u64, y: u64, z: u64) -> u64 {0} // 3
//pub fn bool3ary_205(x: u64, y: u64, z: u64) -> u64 { !(x | z) | y } // 3
//pub fn bool3ary_206(x: u64, y: u64, z: u64) -> u64 {0} // 3
//pub fn bool3ary_208(x: u64, y: u64, z: u64) -> u64 { x & (y | !z) }					// 3,
//pub fn bool3ary_209(x: u64, y: u64, z: u64) -> u64 { !((x & y)^(y & z)^y^z) }		// 3, --half
//pub fn bool3ary_210(x: u64, y: u64, z: u64) -> u64 { x ^ z ^ (y & z) }				// 3, --half
//pub fn bool3ary_211(x: u64, y: u64, z: u64) -> u64 { (!x | y | z) & (x | !y) }		// 3, --mostly-true
//pub fn bool3ary_212(x: u64, y: u64, z: u64) -> u64 { bool3ary_232(x, y, !z) }		// 3, --half
//pub fn bool3ary_213(x: u64, y: u64, z: u64) -> u64 { (x & y) | !z }					// 3, lanand, --mostly-true
//pub fn bool3ary_214(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_215(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_216(x: u64, y: u64, z: u64) -> u64 { (x & !z) | (y & z) }			// 3, --half
//pub fn bool3ary_217(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_218(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_219(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_220(x: u64, y: u64, z: u64) -> u64 {0}
//pub fn bool3ary_222(x: u64, y: u64, z: u64) -> u64 { y | (x ^ z) }		// 3, =imp(eqv(x,z),y), --mostly-true
//pub fn bool3ary_223(x: u64, y: u64, z: u64) -> u64 { y | !(x & z) }		// 3, raimpliesif, --mostly-true
//pub fn bool3ary_224(x: u64, y: u64, z: u64) -> u64 { x & (y | z) }		// 3, --mostly-false
//pub fn bool3ary_225(x: u64, y: u64, z: u64) -> u64 { !(x ^ (y | z)) }				// 3, --half
//pub fn bool3ary_226(x: u64, y: u64, z: u64) -> u64 { (x & y) ^ (y & z) ^ z }		// 3, --half
//pub fn bool3ary_227(x: u64, y: u64, z: u64) -> u64 { !(x^y^(x & z)^(x&y&z)) }		// 3, --mostly-true
pub fn bool3ary_228(x: u64, y: u64, z: u64) -> u64 { y ^ (z & (x ^ y)) }			// 3, MD5_G, --half
pub fn bool3ary_229(x: u64, y: u64, z: u64) -> u64 { !(x^z^(x & y)^(x&y&z)) }		// 3, --mostly-true
pub fn bool3ary_230(x: u64, y: u64, z: u64) -> u64 { (x&y&z) ^ y ^ z }				// 3, --mostly-true
pub fn bool3ary_231(x: u64, y: u64, z: u64) -> u64 { !((x ^ y) & (x ^ z)) }			// 3, --mostly-true
pub fn bool3ary_232(x: u64, y: u64, z: u64) -> u64 { (x & y) ^ (x & z) ^ (y & z) }	// 3, majority, SHA1_F2, --half
pub fn bool3ary_233(x: u64, y: u64, z: u64) -> u64 { !((x&y&z)^x^y^z) }				// 3, --mostly-true
pub fn bool3ary_234(x: u64, y: u64, z: u64) -> u64 { z | (x & y) }					// 3, --mostly-true
pub fn bool3ary_235(x: u64, y: u64, z: u64) -> u64 { z | !(x ^ y) }					// 3, --mostly-true
pub fn bool3ary_236(x: u64, y: u64, z: u64) -> u64 { (x & z) | y }					// 3, --mostly-true
pub fn bool3ary_237(x: u64, y: u64, z: u64) -> u64 { y | !(x ^ z) }					// 3, --mostly-true
pub fn bool3ary_239(x: u64, y: u64, z: u64) -> u64 { !x | y | z }					// 3, --mostly-true
pub fn bool3ary_241(x: u64, y: u64, z: u64) -> u64 { x | !(y | z) }					// 3, --mostly-true
pub fn bool3ary_242(x: u64, y: u64, z: u64) -> u64 { x | (!y & z) }					// 3, raif, --mostly-true
pub fn bool3ary_244(x: u64, y: u64, z: u64) -> u64 { x | (y & !z) }					// 3, --mostly-true
pub fn bool3ary_246(x: u64, y: u64, z: u64) -> u64 { x | (y ^ z) }					// 3, --mostly-true
pub fn bool3ary_247(x: u64, y: u64, z: u64) -> u64 { x | !y | !z }					// 3, laif, --mostly-true
pub fn bool3ary_248(x: u64, y: u64, z: u64) -> u64 { x | (y & z) }					// 3, --mostly-true
pub fn bool3ary_249(x: u64, y: u64, z: u64) -> u64 { x | !(y ^ z) }					// 3, --mostly-true
pub fn bool3ary_251(x: u64, y: u64, z: u64) -> u64 { x | !y | z }					// 3, --mostly-true
pub fn bool3ary_253(x: u64, y: u64, z: u64) -> u64 { x | y | !z }					// 3, --mostly-true
pub fn bool3ary_254(x: u64, y: u64, z: u64) -> u64 { x | y | z }					// 3, or, --mostly-true

// N-ary boolean functions that could be defined as
// 3-ary boolean functions where an argument is ignored

//pub fn bool3ary_3(x: u64, y: u64, z: u64) -> u64 { !(x | y) }		// 2
//pub fn bool3ary_5(x: u64, y: u64, z: u64) -> u64 { !(x | z) }		// 2
//pub fn bool3ary_10(x: u64, y: u64, z: u64) -> u64 { !x & z }		// 2
//pub fn bool3ary_12(x: u64, y: u64, z: u64) -> u64 { !x & y }		// 2
//pub fn bool3ary_15(x: u64, y: u64, z: u64) -> u64 { !x }			// 1, --half
//pub fn bool3ary_17(x: u64, y: u64, z: u64) -> u64 { !y & !z }		// 2
//pub fn bool3ary_51(x: u64, y: u64, z: u64) -> u64 { !y }			// 1, --half
//pub fn bool3ary_85(x: u64, y: u64, z: u64) -> u64 { !z }			// 1, --half
//pub fn bool3ary_170(x: u64, y: u64, z: u64) -> u64 { z }			// 1, --half
//pub fn bool3ary_204(x: u64, y: u64, z: u64) -> u64 { y }			// 1, --half
//pub fn bool3ary_207(x: u64, y: u64, z: u64) -> u64 { !x | y }		// 2
//pub fn bool3ary_221(x: u64, y: u64, z: u64) -> u64 { y | !z } 	// 2
//pub fn bool3ary_238(x: u64, y: u64, z: u64) -> u64 { y | z }		// 2
//pub fn bool3ary_240(x: u64, y: u64, z: u64) -> u64 { x }			// 1, --half
//pub fn bool3ary_243(x: u64, y: u64, z: u64) -> u64 { x | !y }		// 3
//pub fn bool3ary_245(x: u64, y: u64, z: u64) -> u64 { x | !z }		// 2
//pub fn bool3ary_250(x: u64, y: u64, z: u64) -> u64 { x | z }		// 2
//pub fn bool3ary_252(x: u64, y: u64, z: u64) -> u64 { x | y }		// 2
