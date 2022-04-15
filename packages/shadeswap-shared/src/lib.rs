pub mod contract;
pub mod msg;
pub mod state;
pub mod amm_pair;
pub use fadroma;
pub use token_pair::*;
pub use token_type::*;
mod token_pair;
mod token_type;

#[cfg(not(target_arch = "wasm32"))]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Pagination {
    pub start: u64,
    pub limit: u8,
}
