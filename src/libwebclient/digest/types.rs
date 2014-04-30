
pub trait HashAlgorithm {
	fn clear(&mut self);
	fn hash(&mut self, msg: &[u8]) -> ~[u8];
	fn hash_block(&mut self, block: &[u8]);
	fn hash_last_block(&mut self, piece: &[u8]);
    fn get_hash(&self) -> ~[u8];
}

// usage: let hash_bytes = hash(msg_bytes).get_hash()

	//fn get_state();
	//fn get_reusable();
	//fn get_multiple_blocks();
	//fn get_hash_size();
	//fn get_input_block_size();
	//fn get_output_block_size();

	//fn get_hash(&self) -> ~[u8];
    //fn get_iv(&self) -> ~[u8];
    //fn set_iv(&self, iv: &[u8]);

	//fn clear(&self);
	//fn create(&self);
	//fn delete(&self);

	//fn transform_block(&self, m: &[u8]);
	//fn transform_last_block(&self, m: &[u8]);

//enum PaddingMode {
//    PadNone = 0,
//    PadANSIX923 = 1,
//    PadISO10126 = 2,
//    PadZeros = 3,
//    Pad80ZerosLengthLE64 = 4, 	// little-endian 64-bit length, for MD5
//    Pad80ZerosLengthBE64 = 5, 	// big-endian 64-bit length, for SHA-1, SHA-256
//    Pad80ZerosLengthBE128 = 6, 	// big-endian 128-bit length, for SHA-512
//    PadPKCS7 = 7
//}

//struct HashContext {
//	init_value: ~[u8],
//	hash_value: ~mut [u8],
//	block_size: uint,
//	padding_mode: PaddingMode,
//}
