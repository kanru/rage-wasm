#!/bin/sh
wasm-pack build --scope kanru
echo "Convert wasm to asmjs with \`wasm2js\`..."
wasm2js -Oz --enable-mutable-globals pkg/rage_wasm_bg.wasm > pkg/rage_wasm_bg.wasm.js
echo "Update file references..."
sed -i 's/rage_wasm_bg.wasm/rage_wasm_bg.wasm.js/' pkg/package.json pkg/rage_wasm_bg.js pkg/rage_wasm.js
echo "Patch asmjs exports..."
sed -i 's/"__wbindgen_export_0": global$0/"__wbindgen_export_0": { value: global$0 }/' pkg/rage_wasm_bg.wasm.js
echo "export var __wbindgen_export_0 = retasmFunc.__wbindgen_export_0;" >> pkg/rage_wasm_bg.wasm.js
echo "Done"