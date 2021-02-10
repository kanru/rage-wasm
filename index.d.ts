/* tslint:disable */
/* eslint-disable */
/**
* @returns {any[]}
*/
export async function keygen(): any[];
/**
* @param {string} public_key
* @param {Uint8Array} data
* @param {boolean} armor
* @returns {Uint8Array}
*/
export async function encrypt_with_x25519(public_key: string, data: Uint8Array, armor: boolean): Uint8Array;
/**
* @param {string} secret_key
* @param {Uint8Array} data
* @returns {Uint8Array}
*/
export async function decrypt_with_x25519(secret_key: string, data: Uint8Array): Uint8Array;
/**
* @param {string} passphrase
* @param {Uint8Array} data
* @param {boolean} armor
* @returns {Uint8Array}
*/
export async function encrypt_with_user_passphrase(passphrase: string, data: Uint8Array, armor: boolean): Uint8Array;
/**
* @param {string} passphrase
* @param {Uint8Array} data
* @returns {Uint8Array}
*/
export async function decrypt_with_user_passphrase(passphrase: string, data: Uint8Array): Uint8Array;
