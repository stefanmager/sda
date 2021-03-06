//! All crypto-related code.

mod signing;
mod masking;
mod sharing;
mod encryption;

use sda_protocol::*;
use errors::SdaClientResult;

use std::sync::Arc;

pub use self::signing::{
    SignatureKeypair, 
    SignExport, 
    SignatureVerification};
    
pub use self::masking::{
    SecretMaskerConstruction,
    MaskCombinerConstruction,
    SecretUnmaskerConstruction};
    
pub use self::sharing::{
    ShareGeneratorConstruction,
    ShareCombinerConstruction,
    SecretReconstructorConstruction};
    
pub use self::encryption::{
    EncryptionKeypair, 
    EncryptorConstruction, 
    DecryptorConstruction};

pub type Secret = i64;
pub type Mask = i64;
pub type MaskedSecret = i64;
pub type Share = i64;

pub trait KeyGeneration<K> {
    fn new_key(&self) -> SdaClientResult<K>;
}

/// Trait for accessing keys stored in keystore.
pub trait KeyStorage<ID, K> {
    fn put(&self, id: &ID, key: &K) -> SdaClientResult<()>;
    fn get(&self, id: &ID) -> SdaClientResult<Option<K>>;
}

/// Requirements for any keystore used by the client.
pub trait Keystore :
    KeyStorage<EncryptionKeyId, EncryptionKeypair>
    + KeyStorage<VerificationKeyId, SignatureKeypair>
{}

pub trait Suitable<S> {
    fn suitable_for(&self, scheme: &S) -> bool;
}

pub struct CryptoModule {
    keystore: Arc<Keystore>
}

impl CryptoModule {
    pub fn new(keystore: Arc<Keystore>) -> CryptoModule {
        CryptoModule { keystore: keystore }
    }
}
