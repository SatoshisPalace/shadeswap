use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use std::env::current_dir;
use std::fs::create_dir_all;
use amm_pair::operations{}

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(ExecuteAnswer), &out_dir);
    export_schema(&schema_for!(QueryAnswer), &out_dir);
    export_schema(&schema_for!(BuyNft), &out_dir);
}
