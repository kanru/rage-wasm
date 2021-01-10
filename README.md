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

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```
