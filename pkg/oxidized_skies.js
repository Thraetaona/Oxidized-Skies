let wasm;
let memory;

/**
*/
export function main() {
    wasm.main();
}

async function load(module, imports, maybe_memory) {
        try {
            return await WebAssembly.instantiateStreaming(module, imports);
        } catch (e) {
            throw e;
        }
    }
}

async function init(input, maybe_memory) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};


    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports, maybe_memory);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    wasm.__wbindgen_start();
    return wasm;
}

export default init;

