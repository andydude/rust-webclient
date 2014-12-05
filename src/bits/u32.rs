use std::slice;

#[inline]
pub fn rotl(x: u32, y: uint) -> u32 {
    return (x << y) | (x >> (32 - y));
}

#[inline]
pub fn rotr(x: u32, y: uint) -> u32 {
    return (x >> y) | (x << (32 - y));
}

#[inline]
pub fn shr(x: u32, y: uint) -> u32 {
    return x >> y;
}

#[inline]
pub fn shl(x: u32, y: uint) -> u32 {
    return x << y;
}


#[inline]
pub fn from_le(v: Vec<u8>) -> u32 {
    return v[3] as u32 << 24 
         | v[2] as u32 << 16 
         | v[1] as u32 << 8 
         | v[0] as u32;
}

#[inline]
pub fn from_be(v: Vec<u8>) -> u32 {
    return v[0] as u32 << 24 
         | v[1] as u32 << 16 
         | v[2] as u32 << 8 
         | v[3] as u32;
}

#[inline]
pub fn to_be(x: u32) -> Vec<u8> {
    return &[((x >> 24)&0xff) as u8,
             ((x >> 16)&0xff) as u8,
             ((x >> 8)&0xff) as u8,
             (x&0xff) as u8];
}

#[inline]
pub fn to_le(x: u32) -> Vec<u8> {
    return &[((x)&0xff) as u8,
             ((x >> 8)&0xff) as u8,
             ((x >> 16)&0xff) as u8,
             ((x >> 24)&0xff) as u8];
}

#[inline]
pub fn from_le_v(v: Vec<u8>) -> Vec<u32> {
    let mut ret = Vec::new();
    for byteslice in v.chunks(4) {
        let word = from_le(byteslice);
        ret.push(word);
    }
    ret
}

#[inline]
pub fn from_be_v(v: Vec<u8>) -> Vec<u32> {
    let mut ret = Vec::new();
    for byteslice in v.chunks(4) {
        let word = from_be(byteslice);
        ret.push(word);
    }
    ret
}

#[inline]
pub fn to_be_v(words: Vec<u32>) -> Vec<u8> {
    let mut ret = Vec::new();
    for word in words.iter() {
        let bytes = to_be(word);
        for byte in bytes.iter() {
            ret.push(byte);
        }
    }
    ret
}

#[inline]
pub fn to_le_v(words: Vec<u32>) -> Vec<u8> {
    let mut ret = Vec::new();
    for word in words.iter() {
        let bytes = to_le(word);
        for byte in bytes.iter() {
            ret.push(byte);
        }
    }
    ret
}


//pub fn bytes_as_le_u32(v: Vec<u8>) -> u32 {
//    return v[3] as u32 << 24 
//         | v[2] as u32 << 16 
//         | v[1] as u32 << 8 
//         | v[0] as u32;
//}
//
//pub fn le_u32_as_bytes(x: u32) -> Vec<u8> {
//    return &[(x&0xff) as u8,
//             ((x >> 8)&0xff) as u8,
//             ((x >> 16)&0xff) as u8,
//             ((x >> 24)&0xff) as u8];
//}


// commutative 2-ary boolean functions (the odd ones are commutative, the even ones are commutative and associative)

#[inline] pub fn bool2sym_1(a: u32, b: u32) -> u32 { bool2ary_1(a, b) } // 2, nor
#[inline] pub fn bool2sym_2(a: u32, b: u32) -> u32 { bool2ary_6(a, b) } // 2, xor, --assoc, --half, xor(a, b, c) = parity
#[inline] pub fn bool2sym_3(a: u32, b: u32) -> u32 { bool2ary_7(a, b) } // 2, nand
#[inline] pub fn bool2sym_4(a: u32, b: u32) -> u32 { bool2ary_8(a, b) } // 2, and, --assoc, and(a, b, c)
#[inline] pub fn bool2sym_5(a: u32, b: u32) -> u32 { bool2ary_9(a, b) } // 2, eqv, --assoc, --half
#[inline] pub fn bool2sym_6(a: u32, b: u32) -> u32 { bool2ary_14(a, b) }    // 2, or, --assoc, or(a, b, c)

// 2-ary boolean functions

#[inline] pub fn bool2ary_1(a: u32, b: u32) -> u32 { !(a | b) } // 2, nor, --sym                                        =!Or[a, b]
#[inline] pub fn bool2ary_2(a: u32, b: u32) -> u32 { !a & b }   // 2, nif, "NotImpliedBy"                               =!Implies[b, a]
#[inline] pub fn bool2ary_4(a: u32, b: u32) -> u32 { a & !b }   // 2, nimplies, andn, "NotImplies"                      =!Implies[a, b]
#[inline] pub fn bool2ary_6(a: u32, b: u32) -> u32 { a ^ b }    // 2, xor, "ExclusiveOr", --half, --sym                 =!Equivalent[a, b]
#[inline] pub fn bool2ary_7(a: u32, b: u32) -> u32 { !(a & b) } // 2, nand, --assoc, --sym                              =!And[a, b]
#[inline] pub fn bool2ary_8(a: u32, b: u32) -> u32 { a & b }    // 2, and, --assoc, --sym                       U+2227  =And[a, b]
#[inline] pub fn bool2ary_9(a: u32, b: u32) -> u32 { !(a ^ b) } // 2, eqv, nxor, --half, --assoc, --sym         U+2194  =Equivalent[a, b]
#[inline] pub fn bool2ary_11(a: u32, b: u32) -> u32 { !a | b }  // 2, implies, "Implies"                        U+2192  =Implies[a, b]
#[inline] pub fn bool2ary_13(a: u32, b: u32) -> u32 { a | !b }  // 2, if, "ImpliedBy"                           U+2190  =Implies[b, a]
#[inline] pub fn bool2ary_14(a: u32, b: u32) -> u32 { a | b }   // 2, or, ior, "InclusiveOr", --assoc, --sym    U+2228  =Or[a, b]

// commutative 3-ary boolean functions (the odd ones are commutative, the even ones are commutative and associative)

#[inline] pub fn bool3sym_1(a: u32, b: u32, c: u32) -> u32 { bool3ary_1(a, b, c) }      // 3, nor
#[inline] pub fn bool3sym_2(a: u32, b: u32, c: u32) -> u32 { bool3ary_22(a, b, c) }     // 3, uni, one, xand
#[inline] pub fn bool3sym_3(a: u32, b: u32, c: u32) -> u32 { bool3ary_23(a, b, c) }     // 3, nmaj, minority, --half
#[inline] pub fn bool3sym_4(a: u32, b: u32, c: u32) -> u32 { bool3ary_104(a, b, c) }    // 3, duo, two
#[inline] pub fn bool3sym_5(a: u32, b: u32, c: u32) -> u32 { bool3ary_105(a, b, c) }    // 3, nxor, nparity, --half
#[inline] pub fn bool3sym_6(a: u32, b: u32, c: u32) -> u32 { bool3ary_126(a, b, c) }    // 3, neqv
#[inline] pub fn bool3sym_7(a: u32, b: u32, c: u32) -> u32 { bool3ary_127(a, b, c) }    // 3, nand
#[inline] pub fn bool3sym_8(a: u32, b: u32, c: u32) -> u32 { bool3ary_128(a, b, c) }    // 3, and
#[inline] pub fn bool3sym_9(a: u32, b: u32, c: u32) -> u32 { bool3ary_129(a, b, c) }    // 3, eqv
#[inline] pub fn bool3sym_10(a: u32, b: u32, c: u32) -> u32 { bool3ary_150(a, b, c) }   // 3, xor, parity, --half =Eqv[Eqv[a, b], c]
#[inline] pub fn bool3sym_11(a: u32, b: u32, c: u32) -> u32 { bool3ary_151(a, b, c) }   // 3, nduo
#[inline] pub fn bool3sym_12(a: u32, b: u32, c: u32) -> u32 { bool3ary_232(a, b, c) }   // 3, maj, majority, --half
#[inline] pub fn bool3sym_13(a: u32, b: u32, c: u32) -> u32 { bool3ary_233(a, b, c) }   // 3, nuni, nxand
#[inline] pub fn bool3sym_14(a: u32, b: u32, c: u32) -> u32 { bool3ary_254(a, b, c) }   // 3, or


// 3-ary boolean functions

// the boolean function number is the same as Mathematica.
// unary and binary functions have been ommitted from this list.
// for more information on a particular boolean function, see also:
// http://www.wolframalpha.com/input/?i=BooleanFunction[232,3]
// where 232 refers to bool3ary_232(a, b, c) in this file.

// there are only 70 3-ary boolean functions which have the property (--half)
// that half of their inputs yield false and the other half yield true.
// these functions numbers are given by [A014312] https://oeis.org/A014312
// 15, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58, 60, 71, 75, 77, 78, 83, 85, 
// 86, 89, 90, 92, 99, 101, 102, 105, 106, 108, 113, 114, 116, 120, 135, 139, 141, 142, 
// 147, 149, 150, 153, 154, 156, 163, 165, 166, 169, 170, 172, 177, 178, 180, 184, 195, 
// 197, 198, 201, 202, 204, 209, 210, 212, 216, 225, 226, 228, 232, 240

#[inline] pub fn bool3ary_1(a: u32, b: u32, c: u32) -> u32 { !(a | b | c) }                 // 3, nor, --c
#[inline] pub fn bool3ary_2(a: u32, b: u32, c: u32) -> u32 { !(a | b | !c) }                // 3, --mostly-false
#[inline] pub fn bool3ary_4(a: u32, b: u32, c: u32) -> u32 { !(a | !b | c) }                // 3, --mostly-false
#[inline] pub fn bool3ary_6(a: u32, b: u32, c: u32) -> u32 { !a & (b ^ c) }                 // 3, --mostly-false
#[inline] pub fn bool3ary_7(a: u32, b: u32, c: u32) -> u32 { !(a | (b & c)) }               // 3, --mostly-false
#[inline] pub fn bool3ary_8(a: u32, b: u32, c: u32) -> u32 { !a & b & c }                   // 3, --mostly-false
#[inline] pub fn bool3ary_9(a: u32, b: u32, c: u32) -> u32 { !(a | (b ^ c)) }               // 3, --mostly-false
#[inline] pub fn bool3ary_11(a: u32, b: u32, c: u32) -> u32 { !a & (!b | c) }               // 3, --mostly-false
#[inline] pub fn bool3ary_13(a: u32, b: u32, c: u32) -> u32 { !a & (b | !c) }               // 3, --mostly-false
#[inline] pub fn bool3ary_14(a: u32, b: u32, c: u32) -> u32 { !a & (b | c) }                // 3, ranor, --mostly-false
#[inline] pub fn bool3ary_16(a: u32, b: u32, c: u32) -> u32 { a & !(b | c) }                // 3, --mostly-false
#[inline] pub fn bool3ary_18(a: u32, b: u32, c: u32) -> u32 { !b & (a ^ c) }                // 3, --mostly-false
#[inline] pub fn bool3ary_19(a: u32, b: u32, c: u32) -> u32 { !(b | (a & c)) }              // 3, --mostly-false
#[inline] pub fn bool3ary_20(a: u32, b: u32, c: u32) -> u32 { !c & (a ^ b) }                // 3, --mostly-false
#[inline] pub fn bool3ary_21(a: u32, b: u32, c: u32) -> u32 { !(c | (a & b)) }              // 3, --mostly-false
#[inline] pub fn bool3ary_22(a: u32, b: u32, c: u32) -> u32 { a^b^c^(a&b&c) }               // 3, uni, one, xand, --mostly-false
#[inline] pub fn bool3ary_23(a: u32, b: u32, c: u32) -> u32 { !bool3ary_232(a, b, c) }      // 3, minority, nmajority, --half
#[inline] pub fn bool3ary_24(a: u32, b: u32, c: u32) -> u32 { bool3ary_129(!a, b, c) }      // 3, --mostly-false, (a ^ b) & (a ^ c)
#[inline] pub fn bool3ary_25(a: u32, b: u32, c: u32) -> u32 { !((a & b) | (b ^ c)) }        // 3, --mostly-false
#[inline] pub fn bool3ary_26(a: u32, b: u32, c: u32) -> u32 { a^c^(a & b)^(a&b&c) }         // 3, --mostly-false
#[inline] pub fn bool3ary_27(a: u32, b: u32, c: u32) -> u32 { (!a & c) | !(b | c) }         // 3, --half
#[inline] pub fn bool3ary_28(a: u32, b: u32, c: u32) -> u32 { a^b^(a & c)^(a&b&c) }         // 3, --mostly-false
#[inline] pub fn bool3ary_29(a: u32, b: u32, c: u32) -> u32 { (!a & b) ^ !(b | c) }         // 3, --half
#[inline] pub fn bool3ary_30(a: u32, b: u32, c: u32) -> u32 { a ^ (b | c) }                 // 3, --half
#[inline] pub fn bool3ary_31(a: u32, b: u32, c: u32) -> u32 { !(a & (b | c)) }              // 3, --mostly-true
#[inline] pub fn bool3ary_32(a: u32, b: u32, c: u32) -> u32 { a | !b | c }                  // 3, --mostly-false
#[inline] pub fn bool3ary_33(a: u32, b: u32, c: u32) -> u32 { !(b | (a ^ c)) }              // 3, --mostly-false
#[inline] pub fn bool3ary_35(a: u32, b: u32, c: u32) -> u32 { !b & (c | !a) }               // 3, --mostly-false
#[inline] pub fn bool3ary_36(a: u32, b: u32, c: u32) -> u32 { bool3ary_129(a, !b, c) }      // 3, --mostly-false, (a ^ b) & (b ^ c)
#[inline] pub fn bool3ary_37(a: u32, b: u32, c: u32) -> u32 { !((a & b) | (a ^ c)) }        // 3, --mostly-false
#[inline] pub fn bool3ary_38(a: u32, b: u32, c: u32) -> u32 { (a&b)^(a&b&c)^b^c }           // 3, --mostly-false
#[inline] pub fn bool3ary_39(a: u32, b: u32, c: u32) -> u32 { !((a&c) ^ (b&c) ^ a) }        // 3, --half
#[inline] pub fn bool3ary_40(a: u32, b: u32, c: u32) -> u32 { c & (a ^ b) }                 // 3, --mostly-false
#[inline] pub fn bool3ary_41(a: u32, b: u32, c: u32) -> u32 { !((a & b) | (a ^ b ^ c)) }    // 3, --mostly-false
#[inline] pub fn bool3ary_42(a: u32, b: u32, c: u32) -> u32 { c & !(a & b) }                // 3, --mostly-false
#[inline] pub fn bool3ary_43(a: u32, b: u32, c: u32) -> u32 { bool3ary_232(!a, !b, c) }     // 3, --half
#[inline] pub fn bool3ary_44(a: u32, b: u32, c: u32) -> u32 { (b | c) & (a ^ b) }           // 3, --mostly-false
#[inline] pub fn bool3ary_45(a: u32, b: u32, c: u32) -> u32 { a ^ (b | !c) }                // 3, --half
#[inline] pub fn bool3ary_46(a: u32, b: u32, c: u32) -> u32 { (!a & b) | (!b & c) }         // 3, --half
#[inline] pub fn bool3ary_47(a: u32, b: u32, c: u32) -> u32 { !a | (!b & c) }               // 3, --mostly-true
#[inline] pub fn bool3ary_49(a: u32, b: u32, c: u32) -> u32 { !b & (a | !c) }               // 3, --mostly-false
#[inline] pub fn bool3ary_50(a: u32, b: u32, c: u32) -> u32 { !b & (a | c) }                // 3, --mostly-false
#[inline] pub fn bool3ary_52(a: u32, b: u32, c: u32) -> u32 { (b&c)^(a&b&c)^a^b }           // 3, --mostly-false
#[inline] pub fn bool3ary_53(a: u32, b: u32, c: u32) -> u32 { !((a & b) ^ (a & c) ^ c) }    // 3, --half
#[inline] pub fn bool3ary_54(a: u32, b: u32, c: u32) -> u32 { b ^ (a | c) }                 // 3, --half
#[inline] pub fn bool3ary_55(a: u32, b: u32, c: u32) -> u32 { !(b & (a | c)) }              // 3, --mostly-true
#[inline] pub fn bool3ary_56(a: u32, b: u32, c: u32) -> u32 { (a | c) & (a ^ b) }           // 3, --mostly-false
#[inline] pub fn bool3ary_57(a: u32, b: u32, c: u32) -> u32 { b ^ (a | !c) }                // 3, MD5_I, --half
//pub fn bool3ary_58(a: u32, b: u32, c: u32) -> u32 {0}     // 3, --half
//pub fn bool3ary_59(a: u32, b: u32, c: u32) -> u32 {0}     // 3, --mostly-
//pub fn bool3ary_60(a: u32, b: u32, c: u32) -> u32 {0}     // 3, --half
//pub fn bool3ary_61(a: u32, b: u32, c: u32) -> u32 {0}     // 3, --mostly-
//pub fn bool3ary_62(a: u32, b: u32, c: u32) -> u32 {0}     // 3, --mostly-
//pub fn bool3ary_63(a: u32, b: u32, c: u32) -> u32 {0}     // 3, --mostly-
//pub fn bool3ary_64(a: u32, b: u32, c: u32) -> u32 {0}     // 3, --mostly-
//pub fn bool3ary_65(a: u32, b: u32, c: u32) -> u32 {0}     // 3, --mostly-
//pub fn bool3ary_66(a: u32, b: u32, c: u32) -> u32 { bool3ary_129(a, b, !c) }      // 3, --mostly-false
//pub fn bool3ary_67(a: u32, b: u32, c: u32) -> u32 {0}                 // 3, --mostly-
//pub fn bool3ary_68(a: u32, b: u32, c: u32) -> u32 {0}                 // 3, --mostly-
//pub fn bool3ary_69(a: u32, b: u32, c: u32) -> u32 {0}                 // 3, --mostly-
//pub fn bool3ary_70(a: u32, b: u32, c: u32) -> u32 {0}                 // 3, --mostly-
//pub fn bool3ary_71(a: u32, b: u32, c: u32) -> u32 {0}                 // 3, --half
//pub fn bool3ary_72(a: u32, b: u32, c: u32) -> u32 {0}                 // 3, --mostly-
//pub fn bool3ary_73(a: u32, b: u32, c: u32) -> u32 {0}                 // 3, --mostly-
//pub fn bool3ary_74(a: u32, b: u32, c: u32) -> u32 {0}                 // 3, --mostly-
//pub fn bool3ary_75(a: u32, b: u32, c: u32) -> u32 {0}                 // 3, --half
//pub fn bool3ary_76(a: u32, b: u32, c: u32) -> u32 {0}                 // 3, --mostly-
//pub fn bool3ary_77(a: u32, b: u32, c: u32) -> u32 { bool3ary_232(!a, b, !c) }         // 3, --half
//pub fn bool3ary_78(a: u32, b: u32, c: u32) -> u32 {0}         // --half
//pub fn bool3ary_79(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_80(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_81(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_82(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_83(a: u32, b: u32, c: u32) -> u32 {0}         // --half
//pub fn bool3ary_84(a: u32, b: u32, c: u32) -> u32 { ((a | b) & !c }   // 3, lanor
//pub fn bool3ary_86(a: u32, b: u32, c: u32) -> u32 {0}         // --half
//pub fn bool3ary_87(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_88(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_89(a: u32, b: u32, c: u32) -> u32 {0}         // --half
//pub fn bool3ary_90(a: u32, b: u32, c: u32) -> u32 {0}         // --half
//pub fn bool3ary_91(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_92(a: u32, b: u32, c: u32) -> u32 {0}         // --half
//pub fn bool3ary_93(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_94(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_95(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_96(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_97(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_98(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_99(a: u32, b: u32, c: u32) -> u32 {0}         // --half
//pub fn bool3ary_100(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_101(a: u32, b: u32, c: u32) -> u32 {0}            // --half
//pub fn bool3ary_102(a: u32, b: u32, c: u32) -> u32 {0}            // --half
//pub fn bool3ary_103(a: u32, b: u32, c: u32) -> u32 {0}
pub fn bool3ary_104(a: u32, b: u32, c: u32) -> u32 { (b | c) & (a ^ (b & c)) }  // 3, duo, two
pub fn bool3ary_105(a: u32, b: u32, c: u32) -> u32 { !(a ^ b ^ c) }             // 3, nxor, nparity, --half
//pub fn bool3ary_106(a: u32, b: u32, c: u32) -> u32 {0}            // --half
//pub fn bool3ary_107(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_108(a: u32, b: u32, c: u32) -> u32 {0}            // --half
//pub fn bool3ary_109(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_110(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_111(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_112(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_113(a: u32, b: u32, c: u32) -> u32 { bool3ary_232(a, !b, !c) }            // 3, --half
//pub fn bool3ary_114(a: u32, b: u32, c: u32) -> u32 {0}            // --half
//pub fn bool3ary_115(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_116(a: u32, b: u32, c: u32) -> u32 {0}            // --half
//pub fn bool3ary_117(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_118(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_119(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_120(a: u32, b: u32, c: u32) -> u32 {0}            // --half
//pub fn bool3ary_121(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_122(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_123(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_124(a: u32, b: u32, c: u32) -> u32 {0}
//pub fn bool3ary_125(a: u32, b: u32, c: u32) -> u32 {0}
pub fn bool3ary_126(a: u32, b: u32, c: u32) -> u32 { !bool3ary_129(a, b, c) }       // 3, neqv
pub fn bool3ary_127(a: u32, b: u32, c: u32) -> u32 { !(a & b & c) }                 // 3, nand

// 50%

#[inline] pub fn bool3ary_128(a: u32, b: u32, c: u32) -> u32 { a & b & c }                  // 3, and                       =And[a, b, c]
#[inline] pub fn bool3ary_129(a: u32, b: u32, c: u32) -> u32 { !((a ^ b) | (a ^ c)) }       // 3, eqv, equivalent,          =Equivalent[a, b, c]
#[inline] pub fn bool3ary_130(a: u32, b: u32, c: u32) -> u32 { !(a ^ b) & c }               // 3, econd, laeqvand           =And[Equivalent[a, b], c]
#[inline] pub fn bool3ary_131(a: u32, b: u32, c: u32) -> u32 { bool3ary_129(a, b, b & c) }  // 3
#[inline] pub fn bool3ary_132(a: u32, b: u32, c: u32) -> u32 { bool3ary_130(a, c, b) }      // 3
#[inline] pub fn bool3ary_133(a: u32, b: u32, c: u32) -> u32 { bool3ary_129(a, c, b & c) }  // 3
#[inline] pub fn bool3ary_134(a: u32, b: u32, c: u32) -> u32 { (b | c) & (a ^ b ^ c) }      // 3, ?
#[inline] pub fn bool3ary_135(a: u32, b: u32, c: u32) -> u32 { !(a ^ (b & c))}              // 3, eboth, raeqvand, --half   =Equivalent[a, And[b, c]]
#[inline] pub fn bool3ary_137(a: u32, b: u32, c: u32) -> u32 { bool3ary_129(a | c, b, c) }  // 3
#[inline] pub fn bool3ary_138(a: u32, b: u32, c: u32) -> u32 { (!a | b) & c }               // 3, icond, laimpand           =And[Implies[a, b], c]
#[inline] pub fn bool3ary_139(a: u32, b: u32, c: u32) -> u32 { !(a | b) ^ (b & c) }         // 3, ?, --half
#[inline] pub fn bool3ary_140(a: u32, b: u32, c: u32) -> u32 { bool3ary_138(a, c, b) }      // 3
#[inline] pub fn bool3ary_141(a: u32, b: u32, c: u32) -> u32 { !((a&c) ^ (b&c) ^ a ^ c) }   // 3, ?, --half
#[inline] pub fn bool3ary_142(a: u32, b: u32, c: u32) -> u32 { bool3ary_232(!a, b, c) }     // 3, --half
#[inline] pub fn bool3ary_143(a: u32, b: u32, c: u32) -> u32 { !a | (b & c) }               // 3, iboth, ranand, raimpand,  =Implies[a, And[b, c]]
#[inline] pub fn bool3ary_144(a: u32, b: u32, c: u32) -> u32 { bool3ary_130(b, c, a) }      // 3, raandeqv, --mostly-false
#[inline] pub fn bool3ary_145(a: u32, b: u32, c: u32) -> u32 { bool3ary_129(a & c, b, c) }  // 3
#[inline] pub fn bool3ary_146(a: u32, b: u32, c: u32) -> u32 { (a | c) & (a ^ b ^ c) }      // 3, ?,        =And[Xor[a, b, c], Implies[b, a]]
#[inline] pub fn bool3ary_147(a: u32, b: u32, c: u32) -> u32 { bool3ary_135(b, c, a) }      // --half
#[inline] pub fn bool3ary_148(a: u32, b: u32, c: u32) -> u32 { (a | b) & (a ^ b ^ c) }      // 3, ?,        =And[Xor[a, b, c], Implies[c, a]]
#[inline] pub fn bool3ary_149(a: u32, b: u32, c: u32) -> u32 { bool3ary_135(c, a, b) }      // 3, laandeqv, --half
#[inline] pub fn bool3ary_150(a: u32, b: u32, c: u32) -> u32 { a ^ b ^ c }                  // 3, xor, parity, MD5_H, SHA1_F1, SHA1_F3, --half, --sym
#[inline] pub fn bool3ary_151(a: u32, b: u32, c: u32) -> u32 { !(a | b) | (a ^ b ^ c) }     // 3, nduo, ntwo
#[inline] pub fn bool3ary_152(a: u32, b: u32, c: u32) -> u32 { (a | c) & !(b ^ c) }         // 3
#[inline] pub fn bool3ary_154(a: u32, b: u32, c: u32) -> u32 { c ^ (a & !b) }               // 3, laimpeqv, --half, (a IMP b) EQV c 
#[inline] pub fn bool3ary_155(a: u32, b: u32, c: u32) -> u32 { (!a&c) | (b&c) | !(b|c) }    // 3
#[inline] pub fn bool3ary_156(a: u32, b: u32, c: u32) -> u32 { b ^ (a & !c) }               // 3, --half
#[inline] pub fn bool3ary_157(a: u32, b: u32, c: u32) -> u32 { (!a&b) | (b&c) | !(b|c) }    // 3
#[inline] pub fn bool3ary_158(a: u32, b: u32, c: u32) -> u32 { (b & c) | (a ^ b ^ c) }      // 3
#[inline] pub fn bool3ary_159(a: u32, b: u32, c: u32) -> u32 { !(a & (b ^ c)) }             // 3, raimpeqv, a IMP (b EQV c)
#[inline] pub fn bool3ary_161(a: u32, b: u32, c: u32) -> u32 { bool3ary_129(a, b | c, c) }  // 3
#[inline] pub fn bool3ary_162(a: u32, b: u32, c: u32) -> u32 { bool3ary_138(b, a, c) }      // 3, laifand((a IF b) AND c), (a | !b) & c
#[inline] pub fn bool3ary_163(a: u32, b: u32, c: u32) -> u32 { (a & c) | !(a | b) }         // 3, --half
#[inline] pub fn bool3ary_164(a: u32, b: u32, c: u32) -> u32 { (b | c) & !(a ^ c) }         // 3, =Equivalent[a, Implies[b, c], c]
#[inline] pub fn bool3ary_166(a: u32, b: u32, c: u32) -> u32 { c ^ (b | !a) }               // 3, laifeqv, --half, =Equivalent[Implies[b, a], c]
#[inline] pub fn bool3ary_167(a: u32, b: u32, c: u32) -> u32 { !((a&c)^(b&c)^(a&b&c)^a) }   // 3
#[inline] pub fn bool3ary_168(a: u32, b: u32, c: u32) -> u32 { (a | b) & c }                // 3, laorand
#[inline] pub fn bool3ary_169(a: u32, b: u32, c: u32) -> u32 { !(c ^ (a | b)) }             // 3, laoreqv, --half, =Equivalent[Or[a, b], c]
#[inline] pub fn bool3ary_171(a: u32, b: u32, c: u32) -> u32 { !(a | b) | c }               // 3, laorimp
#[inline] pub fn bool3ary_172(a: u32, b: u32, c: u32) -> u32 { (a&b) ^ (a&c) ^ b }          // 3, --half
#[inline] pub fn bool3ary_173(a: u32, b: u32, c: u32) -> u32 { !((b&c) ^ (a&b&c) ^ a ^ c) } // 3
#[inline] pub fn bool3ary_174(a: u32, b: u32, c: u32) -> u32 { c | (b & !a) }               // 3, laifimplies, --mostly-true
#[inline] pub fn bool3ary_176(a: u32, b: u32, c: u32) -> u32 { a & (!b | c) }               // 3, raandimp
#[inline] pub fn bool3ary_177(a: u32, b: u32, c: u32) -> u32 { !((a&c) ^ (b&c) ^ b ^ c) }   // 3, --half
#[inline] pub fn bool3ary_178(a: u32, b: u32, c: u32) -> u32 { bool3ary_232(a, !b, c) }     // 3, --half
#[inline] pub fn bool3ary_179(a: u32, b: u32, c: u32) -> u32 { (a & c) | !b }               // 3, =Implies[b, And[a, c]]
#[inline] pub fn bool3ary_180(a: u32, b: u32, c: u32) -> u32 { a ^ (b | !c) }               // 3, raeqvimp, --half, =Equivalent[a, Implies[b, c]]
#[inline] pub fn bool3ary_181(a: u32, b: u32, c: u32) -> u32 { !((a&b)^(a&c)^(a&b&c)^c) }   // 3
#[inline] pub fn bool3ary_182(a: u32, b: u32, c: u32) -> u32 { (a ^ b ^ c) | (a & c)}       // 3, =Or[Xor[a, b, c], And[a, c]]
#[inline] pub fn bool3ary_183(a: u32, b: u32, c: u32) -> u32 { !(b & (a ^ c)) }             // 3, =Implies[b, Equivalent[a, c]]
#[inline] pub fn bool3ary_184(a: u32, b: u32, c: u32) -> u32 { (a&b)^(b&c)^a }              // 3, --half
#[inline] pub fn bool3ary_185(a: u32, b: u32, c: u32) -> u32 { (a|b|!c)&(!b|c) }            // 3
#[inline] pub fn bool3ary_186(a: u32, b: u32, c: u32) -> u32 { !(!a | b) | c }              // 3, laimplies, --mostly-true
#[inline] pub fn bool3ary_188(a: u32, b: u32, c: u32) -> u32 { (a & b & c) ^ a ^ b }        // 3
#[inline] pub fn bool3ary_189(a: u32, b: u32, c: u32) -> u32 { !bool3ary_129(a, b, !c) }    // 3
#[inline] pub fn bool3ary_190(a: u32, b: u32, c: u32) -> u32 { c | (a ^ b) }                // 3
#[inline] pub fn bool3ary_191(a: u32, b: u32, c: u32) -> u32 { !a | !b | c }                // 3, raimplies(a IMP (b IMP c)), laandimp((a AND b) IMP c)
#[inline] pub fn bool3ary_193(a: u32, b: u32, c: u32) -> u32 { (b | !c) & !(a ^ b) }        // 3
#[inline] pub fn bool3ary_194(a: u32, b: u32, c: u32) -> u32 { (b | c) & !(a ^ b) }         // 3
#[inline] pub fn bool3ary_196(a: u32, b: u32, c: u32) -> u32 { b & (a | !c) }               // 3
#[inline] pub fn bool3ary_197(a: u32, b: u32, c: u32) -> u32 { !((a&b)^(a&c)^a^c) }         // 3, --half
#[inline] pub fn bool3ary_198(a: u32, b: u32, c: u32) -> u32 { b ^ (c & !a) }               // 3, --half
#[inline] pub fn bool3ary_199(a: u32, b: u32, c: u32) -> u32 { !((a&b)^(b&c)^(a&b&c)^a) }   // 3
#[inline] pub fn bool3ary_200(a: u32, b: u32, c: u32) -> u32 { (a | c) & b }                // 3
#[inline] pub fn bool3ary_201(a: u32, b: u32, c: u32) -> u32 { !((a & c)^a^b^c) }           // 3, --half
#[inline] pub fn bool3ary_202(a: u32, b: u32, c: u32) -> u32 { c ^ (a & (b ^ c)) }          // 3, MD5_F, SHA1_F0, --half
#[inline] pub fn bool3ary_203(a: u32, b: u32, c: u32) -> u32 { !((b&c)^(a&b&c)^a^b) }       // 3
#[inline] pub fn bool3ary_205(a: u32, b: u32, c: u32) -> u32 { !(a | c) | b }               // 3
#[inline] pub fn bool3ary_206(a: u32, b: u32, c: u32) -> u32 { b | (c & !a) }               // 3
#[inline] pub fn bool3ary_208(a: u32, b: u32, c: u32) -> u32 { a & (b | !c) }               // 3, raandif
#[inline] pub fn bool3ary_209(a: u32, b: u32, c: u32) -> u32 { !((a & b)^(b & c)^b^c) }     // 3, --half
#[inline] pub fn bool3ary_210(a: u32, b: u32, c: u32) -> u32 { a ^ c ^ (b & c) }            // 3, --half
#[inline] pub fn bool3ary_211(a: u32, b: u32, c: u32) -> u32 { (!a | b | c) & (a | !b) }    // 3, --mostly-true
#[inline] pub fn bool3ary_212(a: u32, b: u32, c: u32) -> u32 { bool3ary_232(a, b, !c) }     // 3, --half
#[inline] pub fn bool3ary_213(a: u32, b: u32, c: u32) -> u32 { (a & b) | !c }               // 3, lanand, laandif, --mostly-true
#[inline] pub fn bool3ary_214(a: u32, b: u32, c: u32) -> u32 { (a&b) | (a^b^c) }            // 3
#[inline] pub fn bool3ary_215(a: u32, b: u32, c: u32) -> u32 { !(c & (a ^ b)) }             // 3
#[inline] pub fn bool3ary_216(a: u32, b: u32, c: u32) -> u32 { (a & !c) | (b & c) }         // 3, --half
#[inline] pub fn bool3ary_217(a: u32, b: u32, c: u32) -> u32 { !((a&b)^(a&b&c)^b^c) }       // 3
#[inline] pub fn bool3ary_218(a: u32, b: u32, c: u32) -> u32 { (a&b&c) ^ a ^ c }            // 3
#[inline] pub fn bool3ary_219(a: u32, b: u32, c: u32) -> u32 { !bool3ary_129(a, !b, c) }    // 3
#[inline] pub fn bool3ary_220(a: u32, b: u32, c: u32) -> u32 { b | (a & !c) }               // 3
#[inline] pub fn bool3ary_222(a: u32, b: u32, c: u32) -> u32 { b | (a ^ c) }                // 3, =imp(eqv(a,c),b), --mostly-true
#[inline] pub fn bool3ary_223(a: u32, b: u32, c: u32) -> u32 { b | !(a & c) }               // 3, raimpliesif, --mostly-true
#[inline] pub fn bool3ary_224(a: u32, b: u32, c: u32) -> u32 { a & (b | c) }                // 3, --mostly-false
#[inline] pub fn bool3ary_225(a: u32, b: u32, c: u32) -> u32 { !(a ^ (b | c)) }             // 3, --half
#[inline] pub fn bool3ary_226(a: u32, b: u32, c: u32) -> u32 { (a & b) ^ (b & c) ^ c }      // 3, --half
#[inline] pub fn bool3ary_227(a: u32, b: u32, c: u32) -> u32 { !(a^b^(a & c)^(a&b&c)) }     // 3, --mostly-true
#[inline] pub fn bool3ary_228(a: u32, b: u32, c: u32) -> u32 { b ^ (c & (a ^ b)) }          // 3, MD5_G, --half
#[inline] pub fn bool3ary_229(a: u32, b: u32, c: u32) -> u32 { !(a^c^(a & b)^(a&b&c)) }     // 3, --mostly-true
#[inline] pub fn bool3ary_230(a: u32, b: u32, c: u32) -> u32 { (a&b&c) ^ b ^ c }            // 3, --mostly-true
#[inline] pub fn bool3ary_231(a: u32, b: u32, c: u32) -> u32 { !bool3ary_129(!a, b, c) }    // 3, --mostly-true, !((a ^ b) & (a ^ c))
#[inline] pub fn bool3ary_232(a: u32, b: u32, c: u32) -> u32 { (a & b) ^ (a & c) ^ (b & c) }// 3, majority, SHA1_F2, --half
#[inline] pub fn bool3ary_233(a: u32, b: u32, c: u32) -> u32 { !((a&b&c)^a^b^c) }           // 3, nuni, none, nxand, Not[OneBit[a, b, c]], --mostly-true
#[inline] pub fn bool3ary_234(a: u32, b: u32, c: u32) -> u32 { c | (a & b) }                // 3, --mostly-true
#[inline] pub fn bool3ary_235(a: u32, b: u32, c: u32) -> u32 { c | !(a ^ b) }               // 3, --mostly-true
#[inline] pub fn bool3ary_236(a: u32, b: u32, c: u32) -> u32 { (a & c) | b }                // 3, --mostly-true
#[inline] pub fn bool3ary_237(a: u32, b: u32, c: u32) -> u32 { b | !(a ^ c) }               // 3, --mostly-true
#[inline] pub fn bool3ary_239(a: u32, b: u32, c: u32) -> u32 { !a | b | c }                 // 3, --mostly-true
#[inline] pub fn bool3ary_241(a: u32, b: u32, c: u32) -> u32 { a | !(b | c) }               // 3, --mostly-true
#[inline] pub fn bool3ary_242(a: u32, b: u32, c: u32) -> u32 { a | (!b & c) }               // 3, raif, --mostly-true
#[inline] pub fn bool3ary_244(a: u32, b: u32, c: u32) -> u32 { a | (b & !c) }               // 3, --mostly-true
#[inline] pub fn bool3ary_246(a: u32, b: u32, c: u32) -> u32 { a | (b ^ c) }                // 3, --mostly-true
#[inline] pub fn bool3ary_247(a: u32, b: u32, c: u32) -> u32 { a | !b | !c }                // 3, laif((a IF b) IF c), raifand(a IF (b AND c)), --mostly-true
#[inline] pub fn bool3ary_248(a: u32, b: u32, c: u32) -> u32 { a | (b & c) }                // 3, raorand, --mostly-true
#[inline] pub fn bool3ary_249(a: u32, b: u32, c: u32) -> u32 { a | !(b ^ c) }               // 3, raifxor, raoreqv, --mostly-true
#[inline] pub fn bool3ary_251(a: u32, b: u32, c: u32) -> u32 { a | !b | c }                 // 3, --mostly-true
#[inline] pub fn bool3ary_253(a: u32, b: u32, c: u32) -> u32 { a | b | !c }                 // 3, --mostly-true
#[inline] pub fn bool3ary_254(a: u32, b: u32, c: u32) -> u32 { a | b | c }                  // 3, or, --mostly-true

// N-ary boolean functions that could be defined as
// 3-ary boolean functions where an argument is ignored

// 36 reducible
// (256 - 36) irriducible

//pub fn bool3ary_3(a: u32, b: u32, c: u32) -> u32 { !(a | b) }     // 2
//pub fn bool3ary_5(a: u32, b: u32, c: u32) -> u32 { !(a | c) }     // 2
//pub fn bool3ary_10(a: u32, b: u32, c: u32) -> u32 { !a & c }      // 2
//pub fn bool3ary_12(a: u32, b: u32, c: u32) -> u32 { !a & b }      // 2
//pub fn bool3ary_15(a: u32, b: u32, c: u32) -> u32 { !a }          // 1, --half
//pub fn bool3ary_17(a: u32, b: u32, c: u32) -> u32 { !b & !c }     // 2
//pub fn bool3ary_34(a: u32, b: u32, c: u32) -> u32 { !b | c }      // 2
//pub fn bool3ary_48(a: u32, b: u32, c: u32) -> u32 { a & !b }      // 2
//pub fn bool3ary_51(a: u32, b: u32, c: u32) -> u32 { !b }          // 1, --half
//pub fn bool3ary_85(a: u32, b: u32, c: u32) -> u32 { !c }          // 1, --half

//pub fn bool3ary_136(a: u32, b: u32, c: u32) -> u32 { b & c }      // 2
//pub fn bool3ary_153(a: u32, b: u32, c: u32) -> u32 { !(b ^ c) }   // 2, --half
//pub fn bool3ary_160(a: u32, b: u32, c: u32) -> u32 { a & c }      // 2
//pub fn bool3ary_165(a: u32, b: u32, c: u32) -> u32 { !(a ^ c) }   // 2, --half
//pub fn bool3ary_170(a: u32, b: u32, c: u32) -> u32 { c }          // 1, --half
//pub fn bool3ary_175(a: u32, b: u32, c: u32) -> u32 { !a | c }     // 2
//pub fn bool3ary_187(a: u32, b: u32, c: u32) -> u32 { !b | c }     // 2
//pub fn bool3ary_192(a: u32, b: u32, c: u32) -> u32 { a ^ b }      // 2
//pub fn bool3ary_195(a: u32, b: u32, c: u32) -> u32 { !(a ^ b) }   // 2, --half
//pub fn bool3ary_204(a: u32, b: u32, c: u32) -> u32 { b }          // 1, --half
//pub fn bool3ary_207(a: u32, b: u32, c: u32) -> u32 { !a | b }     // 2
//pub fn bool3ary_221(a: u32, b: u32, c: u32) -> u32 { b | !c }     // 2
//pub fn bool3ary_238(a: u32, b: u32, c: u32) -> u32 { b | c }      // 2
//pub fn bool3ary_240(a: u32, b: u32, c: u32) -> u32 { a }          // 1, --half
//pub fn bool3ary_243(a: u32, b: u32, c: u32) -> u32 { a | !b }     // 3
//pub fn bool3ary_245(a: u32, b: u32, c: u32) -> u32 { a | !c }     // 2
//pub fn bool3ary_250(a: u32, b: u32, c: u32) -> u32 { a | c }      // 2
//pub fn bool3ary_252(a: u32, b: u32, c: u32) -> u32 { a | b }      // 2
