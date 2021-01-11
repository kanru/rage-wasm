#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use rage_wasm::{
    decrypt_with_user_passphrase, decrypt_with_x25519, encrypt_with_user_passphrase,
    encrypt_with_x25519, keygen,
};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn x25519_encryption_and_decryption() {
    let secret_key = "AGE-SECRET-KEY-17QZPZDKGN49PAJHQGKDT056ND8MEZ6ZQK9HPXGMCS85VXNXEPATSQTYK6T";
    let public_key = "age1mfqmqkz9ga3a3lrgw8yatm79h5pqdu7z2hclghck5v8lrtwerysq97u6j8";
    let data = "test";
    let encrypted = encrypt_with_x25519(&public_key, data.as_bytes(), true).unwrap();
    let decrypted = decrypt_with_x25519(&secret_key, &encrypted).unwrap();

    assert_eq!(data.as_bytes(), decrypted.as_ref());
}

#[wasm_bindgen_test]
fn x25519_keygen_and_encryption_and_decryption() {
    let keys = keygen();
    let secret_key = keys[0].as_string().unwrap();
    let public_key = keys[1].as_string().unwrap();
    let data = "test";
    let encrypted = encrypt_with_x25519(&public_key, data.as_bytes(), true).unwrap();
    let decrypted = decrypt_with_x25519(&secret_key, &encrypted).unwrap();

    assert_eq!(data.as_bytes(), decrypted.as_ref());
}

#[wasm_bindgen_test]
fn x25519_keygen_and_unarmored_encryption_and_decryption() {
    let keys = keygen();
    let secret_key = keys[0].as_string().unwrap();
    let public_key = keys[1].as_string().unwrap();
    let data = "test";
    let encrypted = encrypt_with_x25519(&public_key, data.as_bytes(), false).unwrap();
    let decrypted = decrypt_with_x25519(&secret_key, &encrypted).unwrap();

    assert_eq!(data.as_bytes(), decrypted.as_ref());
}

#[wasm_bindgen_test]
fn passphrase_encryption_and_decryption() {
    let key = "password";
    let data = "test";
    let encrypted = encrypt_with_user_passphrase(&key, data.as_bytes(), true).unwrap();
    let decrypted = decrypt_with_user_passphrase(&key, &encrypted).unwrap();

    assert_eq!(data.as_bytes(), decrypted.as_ref());
}

#[wasm_bindgen_test]
fn passphrase_unarmored_encryption_and_decryption() {
    let key = "password";
    let data = "test";
    let encrypted = encrypt_with_user_passphrase(&key, data.as_bytes(), true).unwrap();
    let decrypted = decrypt_with_user_passphrase(&key, &encrypted).unwrap();

    assert_eq!(data.as_bytes(), decrypted.as_ref());
}
