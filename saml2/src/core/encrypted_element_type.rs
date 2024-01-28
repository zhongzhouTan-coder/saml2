use super::{encrypted_data::EncryptedData, encrypted_key::EncryptedKey};

pub trait EncryptedElementType {
    fn encrypted_data(&self) -> EncryptedData;

    fn set_encrypted_data(&mut self, value: EncryptedData);

    fn encrypted_keys(&self) -> Vec<EncryptedKey>;

    fn set_encrypted_keys(&mut self, value: Vec<EncryptedKey>);
}
