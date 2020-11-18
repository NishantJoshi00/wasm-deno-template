const wasmCode = await Deno.readFile("./wasm/pkg/wasm_bg.wasm");
const wasmModule = new WebAssembly.Module(wasmCode);
const wasmInstance = new WebAssembly.Instance(wasmModule);
const exports = wasmInstance.exports;


// Use the functions from the exports variable for further code where you would like to use the features of webassembly