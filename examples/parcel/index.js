import { html, render } from 'htm/preact';
import * as rage from '@kanru/rage-wasm';

async function run() {
    let key = await rage.keygen();
    render(html`<div>
        <p>Generating AGE key...</p>
        <p>Private key: ${key[0]}</p>
        <p>Public key: ${key[1]}</p>
    </div>`, document.body);
}

run();