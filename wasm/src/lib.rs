use wasm_bindgen::prelude::*;


// Using the wee_alloc allocator

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// All the function that you would like to export in the wasm file must have the following
// #[wasm_bindgen] as the decorator
// must be a public function or struct
