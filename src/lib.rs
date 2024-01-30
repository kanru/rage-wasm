use age::{
    armor::{ArmoredReader, ArmoredWriter, Format},
    x25519, Decryptor, Encryptor,
};
use secrecy::{ExposeSecret, Secret};
use std::{
    io::{Read, Write},
    iter, vec,
};
use wasm_bindgen::prelude::*;

fn encrypt_error<T>(_: T) -> JsValue {
    js_sys::Error::new("encryption error").into()
}

fn decrypt_error<T>(_: T) -> JsValue {
    js_sys::Error::new("decryption error").into()
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
    let recipients = vec![Box::new(key) as Box<dyn age::Recipient + Send + 'static>];
    let encryptor = Encryptor::with_recipients(recipients).unwrap();
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
        .decrypt(iter::once(&identity as &dyn age::Identity))
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
