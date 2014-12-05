
pub trait HashAlgorithm {
    fn get_block_size(&self) -> uint;
    fn get_hash_size(&self) -> uint;
    fn get_hash(&self) -> ~[u8];

    fn clear(&mut self);
    fn hash(&mut self, msg: &[u8]) -> ~[u8];
    fn hash_block(&mut self, block: &[u8]);
    fn hash_last_block(&mut self, piece: &[u8]);
}

pub trait SymmetricAlgorithm {
    fn get_block_size(&self);
    fn get_key_size(&self);
    fn get_iv(&self) -> ~[u8];
    fn set_iv(&mut self, iv: &[u8]);
    fn get_key(&self) -> ~[u8];
    fn set_key(&mut self, key: &[u8]);

    fn clear(&mut self);
    fn decrypt(&mut self, msg: &[u8]) -> ~[u8];
    fn decrypt_block(&mut self, block: &[u8]) -> ~[u8];
    fn decrypt_last_block(&mut self, piece: &[u8]) -> ~[u8];
    fn encrypt(&mut self, msg: &[u8]) -> ~[u8];
    fn encrypt_block(&mut self, block: &[u8]) -> ~[u8];
    fn encrypt_last_block(&mut self, piece: &[u8]) -> ~[u8];

    fn generate_iv(&self);
    fn generate_key(&self);
    fn validate_key_size(&self);
}

pub trait AsymmetricAlgorithm {
    fn get_block_size();
    fn get_key_size();
    fn get_key();
    fn set_key();

    fn clear(&mut self);
    fn decrypt(&mut self, msg: &[u8]) -> ~[u8];
    fn decrypt_block(&mut self, block: &[u8]) -> ~[u8];
    fn decrypt_last_block(&mut self, piece: &[u8]) -> ~[u8];
    fn encrypt(&mut self, msg: &[u8]) -> ~[u8];
    fn encrypt_block(&mut self, block: &[u8]) -> ~[u8];
    fn encrypt_last_block(&mut self, piece: &[u8]) -> ~[u8];
}
