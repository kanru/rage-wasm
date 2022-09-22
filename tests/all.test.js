import { expect } from "@esm-bundle/chai";
import { keygen, encrypt_with_x25519, decrypt_with_x25519, encrypt_with_user_passphrase, decrypt_with_user_passphrase } from "../dist/index.js";

const ENCODER = new TextEncoder("utf-8");
const DECODER = new TextDecoder("utf-8");

function encode(s) {
    return ENCODER.encode(s);
}

function decode(arr) {
    return DECODER.decode(arr);
}

it('x25519 encryption and decryption', async () => {
    let secret_key = "AGE-SECRET-KEY-17QZPZDKGN49PAJHQGKDT056ND8MEZ6ZQK9HPXGMCS85VXNXEPATSQTYK6T";
    let public_key = "age1mfqmqkz9ga3a3lrgw8yatm79h5pqdu7z2hclghck5v8lrtwerysq97u6j8";
    let data = "test";
    let encrypted = await encrypt_with_x25519(public_key, encode(data), true);
    let decrypted = decode(await decrypt_with_x25519(secret_key, encrypted));
    expect(decrypted).to.equal(data);
});

it('x25519 keygen and encryption and decryption', async () => {
    let keys = await keygen();
    let secret_key = keys[0];
    let public_key = keys[1];
    let data = "test";
    let encrypted = await encrypt_with_x25519(public_key, encode(data), true);
    let decrypted = decode(await decrypt_with_x25519(secret_key, encrypted));
    expect(decrypted).to.equal(data);
});

it('x25519 keygen and unaromored encryption and decryption', async () => {
    let keys = await keygen();
    let secret_key = keys[0];
    let public_key = keys[1];
    let data = "test";
    let encrypted = await encrypt_with_x25519(public_key, encode(data), false);
    let decrypted = decode(await decrypt_with_x25519(secret_key, encrypted));
    expect(decrypted).to.equal(data);
});

it('passphrase encryption and decryption', async function () {
    this.timeout(5000);
    let key = "passphrase";
    let data = "test";
    let encrypted = await encrypt_with_user_passphrase(key, encode(data), true);
    let decrypted = decode(await decrypt_with_user_passphrase(key, encrypted));
    expect(decrypted).to.equal(data);
});

it('passphrase unarmored encryption and decryption', async function () {
    this.timeout(5000);
    let key = "passphrase";
    let data = "test";
    let encrypted = await encrypt_with_user_passphrase(key, encode(data), false);
    let decrypted = decode(await decrypt_with_user_passphrase(key, encrypted));
    expect(decrypted).to.equal(data);
});