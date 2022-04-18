pub mod contract;
pub mod msg;
pub mod state;
pub mod amm_pair;
pub mod token_pair_amount;
pub mod token_type_amount;
pub use fadroma;
pub use composable-snip20;
pub use token_pair::*;
pub use token_type::*;
mod token_pair;
mod token_type;
mod display;

#[cfg(not(target_arch = "wasm32"))]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub start: u64,
    pub limit: u8,
}
