import * as wasm from './Cargo.toml';

const rage_wasm = wasm.default();

export async function keygen() {
	return (await rage_wasm).keygen();
}

export async function encrypt_with_x25519(public_key, data, armor) {
	return (await rage_wasm).encrypt_with_x25519(public_key, data, armor);
}

export async function decrypt_with_x25519(secret_key, data) {
	return (await rage_wasm).decrypt_with_x25519(secret_key, data);
}

export async function encrypt_with_user_passphrase(passphrase, data, armor) {
	return (await rage_wasm).encrypt_with_user_passphrase(passphrase, data, armor);
}

export async function decrypt_with_user_passphrase(passphrase, data) {
	return (await rage_wasm).decrypt_with_user_passphrase(passphrase, data);
}