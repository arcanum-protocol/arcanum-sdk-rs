mod utils;
use ethers::prelude::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub async fn create_caller(middleware: SignerMiddleware, ) {

}
#[wasm_bindgen]
pub async fn estimate_mint_shares() {

}

#[wasm_bindgen]
pub fn estimate_mint_amount_out() {

}

#[wasm_bindgen]
pub fn estimate_burn_amount_out() {

}

#[wasm_bindgen]
pub fn estimate_burn_shares_in() {

}
