use wasm_bindgen::prelude::*;
use willi::WilliStundenplan;

#[wasm_bindgen]
pub struct OptimizerParams {
  #[wasm_bindgen(getter_with_clone)]
  pub subjects: Vec<usize>,
}

#[wasm_bindgen]
pub fn optimize(plan: &WilliStundenplan, parameters: OptimizerParams) -> () {
  todo!()
}
