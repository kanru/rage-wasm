/* tslint:disable */
/* eslint-disable */
export function keygen(): Promise<string[]>;
export function encrypt_with_x25519(public_key: string, data: Uint8Array, armor: boolean): Promise<Uint8Array>;
export function decrypt_with_x25519(secret_key: string, data: Uint8Array): Promise<Uint8Array>;
export function encrypt_with_user_passphrase(passphrase: string, data: Uint8Array, armor: boolean): Promise<Uint8Array>;
export function decrypt_with_user_passphrase(passphrase: string, data: Uint8Array): Promise<Uint8Array>;
