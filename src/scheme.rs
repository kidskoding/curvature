use crate::types::{PublicKey, SecretKey, Signature};

pub trait Scheme {
    fn keygen(&self) -> (SecretKey, PublicKey);
    fn sign(&self, key: &SecretKey, message: &[u8]) -> Signature;
    fn verify(&self, pub_key: &PublicKey, message: &[u8], signature: &Signature) -> bool;
    fn batch_verify(&self, collection: &[(PublicKey, Vec<u8>, Signature)]) -> bool;
    fn aggregate(&self, signatures: &[Signature]) -> Option<Signature>;
    fn verify_aggregate(&self, pub_keys: &[PublicKey], message: &[u8], signature: &Signature) -> Option<bool>;
    fn pk_len(&self) -> usize;
    fn sig_len(&self) -> usize;
    fn name(&self) -> &'static str;
}
