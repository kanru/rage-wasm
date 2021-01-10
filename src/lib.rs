use secrecy::{ExposeSecret, Secret};
use std::{
    io::{Read, Write},
    iter, vec,
};
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn encrypt_error<T>(_: T) -> JsValue {
    js_sys::Error::new("encryption error").into()
}

fn decrypt_error<T>(_: T) -> JsValue {
    js_sys::Error::new("decryption error").into()
}

#[wasm_bindgen]
pub fn keygen() -> Vec<JsValue> {
    let secret = age::x25519::Identity::generate();
    let public = secret.to_public();
    vec![
        JsValue::from(secret.to_string().expose_secret()),
        JsValue::from(public.to_string()),
    ]
}

#[wasm_bindgen]
pub fn encrypt_with_x25519(public_key: &str, data: &[u8]) -> Result<Box<[u8]>, JsValue> {
    let key: age::x25519::Recipient = public_key.parse().map_err(encrypt_error)?;
    let recipients = vec![Box::new(key) as Box<dyn age::Recipient>];
    let encryptor = age::Encryptor::with_recipients(recipients);
    let mut output = vec![];
    let mut stream_writer = encryptor.wrap_output(&mut output).map_err(encrypt_error)?;
    stream_writer.write_all(data).map_err(encrypt_error)?;
    stream_writer.finish().map_err(encrypt_error)?;
    Ok(output.into_boxed_slice())
}

#[wasm_bindgen]
pub fn decrypt_with_x25519(secret_key: &str, data: &[u8]) -> Result<Box<[u8]>, JsValue> {
    let identity: age::x25519::Identity = secret_key.parse().map_err(encrypt_error)?;
    let decryptor = match age::Decryptor::new(data).map_err(decrypt_error)? {
        age::Decryptor::Recipients(d) => d,
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
pub fn encrypt_with_user_passphrase(passphrase: &str, data: &[u8]) -> Result<Box<[u8]>, JsValue> {
    let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));
    let mut output = vec![];
    let mut stream_writer = encryptor.wrap_output(&mut output).map_err(encrypt_error)?;
    stream_writer.write_all(data).map_err(encrypt_error)?;
    stream_writer.finish().map_err(encrypt_error)?;
    Ok(output.into_boxed_slice())
}

#[wasm_bindgen]
pub fn decrypt_with_user_passphrase(passphrase: &str, data: &[u8]) -> Result<Box<[u8]>, JsValue> {
    let decryptor = match age::Decryptor::new(data).map_err(decrypt_error)? {
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
