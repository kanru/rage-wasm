use age::{
    armor::{ArmoredReader, ArmoredWriter, Format},
    x25519, Decryptor, Encryptor,
};
use bech32::ToBase32;
use secrecy::{ExposeSecret, Secret};
use std::{
    convert::TryInto,
    io::{Read, Write},
    iter, vec,
};
use wasm_bindgen::prelude::*;
use zeroize::Zeroize;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Use lower-case HRP to avoid https://github.com/rust-bitcoin/rust-bech32/issues/40
const SECRET_KEY_PREFIX: &str = "age-secret-key-";

fn keygen_error<T>(_: T) -> JsValue {
    js_sys::Error::new("keygen error").into()
}

fn encrypt_error<T>(_: T) -> JsValue {
    js_sys::Error::new("encryption error").into()
}

fn decrypt_error<T>(_: T) -> JsValue {
    js_sys::Error::new("decryption error").into()
}

#[wasm_bindgen]
pub fn keygen_from_random_bytes(sk_bytes: &[u8]) -> Result<Vec<JsValue>, JsValue> {
    let mut sk_bytes: [u8; 32] = sk_bytes.to_vec().try_into().map_err(keygen_error)?;
    let secret = x25519_dalek::StaticSecret::from(sk_bytes);
    let sk_base32 = secret.to_bytes().to_base32();
    let mut encoded = bech32::encode(SECRET_KEY_PREFIX, sk_base32).map_err(keygen_error)?;
    let secret_string = encoded.to_uppercase();

    // Clear intermediates
    sk_bytes.zeroize();
    // TODO: bech32::u5 doesn't implement Zeroize
    // sk_base32.zeroize();
    encoded.zeroize();

    let secret: x25519::Identity = secret_string.parse().map_err(keygen_error)?;
    let public = secret.to_public();
    Ok(vec![
        JsValue::from(secret.to_string().expose_secret()),
        JsValue::from(public.to_string()),
    ])
}

#[wasm_bindgen]
pub fn keygen() -> Vec<JsValue> {
    let secret = x25519::Identity::generate();
    let public = secret.to_public();
    vec![
        JsValue::from(secret.to_string().expose_secret()),
        JsValue::from(public.to_string()),
    ]
}

#[wasm_bindgen]
pub fn encrypt_with_x25519(
    public_key: &str,
    data: &[u8],
    armor: bool,
) -> Result<Box<[u8]>, JsValue> {
    let key: x25519::Recipient = public_key.parse().map_err(encrypt_error)?;
    let recipients = vec![Box::new(key) as Box<dyn age::Recipient>];
    let encryptor = Encryptor::with_recipients(recipients);
    let mut output = vec![];
    let format = if armor {
        Format::AsciiArmor
    } else {
        Format::Binary
    };
    let armor = ArmoredWriter::wrap_output(&mut output, format).map_err(encrypt_error)?;
    let mut writer = encryptor.wrap_output(armor).map_err(encrypt_error)?;
    writer.write_all(data).map_err(encrypt_error)?;
    writer
        .finish()
        .and_then(|armor| armor.finish())
        .map_err(encrypt_error)?;
    Ok(output.into_boxed_slice())
}

#[wasm_bindgen]
pub fn decrypt_with_x25519(secret_key: &str, data: &[u8]) -> Result<Box<[u8]>, JsValue> {
    let identity: x25519::Identity = secret_key.parse().map_err(encrypt_error)?;
    let armor = ArmoredReader::new(data);
    let decryptor = match Decryptor::new(armor).map_err(decrypt_error)? {
        Decryptor::Recipients(d) => d,
        _ => return Err(decrypt_error(())),
    };
    let mut decrypted = vec![];
    let mut reader = decryptor
        .decrypt(iter::once(Box::new(identity) as Box<dyn age::Identity>))
        .map_err(decrypt_error)?;
    reader.read_to_end(&mut decrypted).map_err(decrypt_error)?;
    Ok(decrypted.into_boxed_slice())
}

#[wasm_bindgen]
pub fn encrypt_with_user_passphrase(
    passphrase: &str,
    data: &[u8],
    armor: bool,
) -> Result<Box<[u8]>, JsValue> {
    let encryptor = Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));
    let mut output = vec![];
    let format = if armor {
        Format::AsciiArmor
    } else {
        Format::Binary
    };
    let armor = ArmoredWriter::wrap_output(&mut output, format).map_err(encrypt_error)?;
    let mut writer = encryptor.wrap_output(armor).map_err(encrypt_error)?;
    writer.write_all(data).map_err(encrypt_error)?;
    writer
        .finish()
        .and_then(|armor| armor.finish())
        .map_err(encrypt_error)?;
    Ok(output.into_boxed_slice())
}

#[wasm_bindgen]
pub fn decrypt_with_user_passphrase(passphrase: &str, data: &[u8]) -> Result<Box<[u8]>, JsValue> {
    let armor = ArmoredReader::new(data);
    let decryptor = match age::Decryptor::new(armor).map_err(decrypt_error)? {
        age::Decryptor::Passphrase(d) => d,
        _ => return Err(decrypt_error(())),
    };
    let mut decrypted = vec![];
    let mut reader = decryptor
        .decrypt(&Secret::new(passphrase.to_owned()), None)
        .map_err(decrypt_error)?;
    reader.read_to_end(&mut decrypted).map_err(decrypt_error)?;
    Ok(decrypted.into_boxed_slice())
}
