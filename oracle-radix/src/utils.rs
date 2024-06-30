use std::str::FromStr;

use scrypto::crypto::{Bls12381G1PublicKey, Bls12381G2Signature};
use scrypto::crypto_utils::CryptoUtils;
use scrypto::prelude::{Clock, Instant, TimePrecision};

pub fn get_time() -> u64 {
    let instant: Instant = Clock::current_time(TimePrecision::Minute);
    instant.seconds_since_unix_epoch.try_into().unwrap()
}

pub fn check_signature(message_str: &str, signature_str: &str, public_key: Bls12381G1PublicKey) {
    let signature = match Bls12381G2Signature::from_str(signature_str) {
        Ok(signature) => signature,
        Err(err) => panic!(
            "Error getting Bls12381G2Signature from str, error: {:?}",
            err
        ),
    };

    if !CryptoUtils::bls12381_v1_verify(message_str.bytes().collect(), public_key, signature) {
        panic!("Verification of signature failed!");
    }
}
