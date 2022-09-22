# rage-wasm: WebAssembly wrapper of rage

rage is a simple, modern, and secure file encryption tool, using the age format.
It features small explicit keys, no config options, and UNIX-style
composability.

The format specification is at [age-encryption.org/v1](https://age-encryption.org/v1).
To discuss the spec or other age related topics, please email
[the mailing list](https://groups.google.com/d/forum/age-dev) at
age-dev@googlegroups.com. age was designed by
[@Benjojo12](https://twitter.com/Benjojo12) and
[@FiloSottile](https://twitter.com/FiloSottile).

This package is a WebAssembly wrapper of the Rust
[rage](https://github.com/str4d/rage) package, providing basic encryption and
descryption operations.

## 🚴 Usage

### 🐑 Use NPM or Yarn to install the package

```
npm install @kanru/rage-wasm
```

The package exports a single module with 5 async methods. Upon first use
an inlined webassembly module will be loaded asynchronously.

- keygen - generate x25519 key pairs
- encrypt_with_x25519
- decrypt_with_x25519
- encrypt_with_user_passphrase
- decrypt_with_user_passphrase

### Examples

Some examples with parcel or shadow-cljs are available under the `examples/` directory.

## Contribute

### 🛠️ Build

```
npm install
npm run build
```

### 🎁 Publish to NPM

```
npm publish
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack build
wasm-pack test --headless --firefox
```
