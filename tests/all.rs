#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use rage_wasm::{
    decrypt_with_user_passphrase, decrypt_with_x25519, encrypt_with_user_passphrase,
    encrypt_with_x25519, keygen, keygen_from_random_bytes,
};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn x25519_keygen_from_random_bytes() {
    let random = [
        255, 148, 143, 240, 115, 122, 42, 70, 94, 185, 119, 173, 18, 197, 26, 160, 101, 77, 81,
        186, 53, 22, 122, 247, 229, 230, 181, 153, 209, 156, 188, 253,
    ];
    let keys = keygen_from_random_bytes(&random).unwrap();
    let secret_key = keys[0].as_string().unwrap();
    let public_key = keys[1].as_string().unwrap();

    assert_eq!(
        "AGE-SECRET-KEY-1LZ2GLURN0G4YVH4EW7K393G65PJ565D6X5T84AL9U66EN5VUH37SRQ997N",
        secret_key
    );
    assert_eq!(
        "age1qa2v3e6qftcce6l66n7ej8urar9lmctyjkkx8ulk92x63duq546sqv2p3p",
        public_key
    );
}

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
