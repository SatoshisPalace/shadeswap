use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use shadeswap_shared::*;
use std::env::current_dir;
use std::fs::create_dir_all;
fn main() {
    let base_out_dir = current_dir().unwrap().join("schema");
    create_dir_all(&base_out_dir).unwrap();
    remove_schemas(&base_out_dir).unwrap();

    // amm_pair
    let amm_pair_out_dir = base_out_dir.join("amm_pair");
    create_dir_all(&amm_pair_out_dir).unwrap();
    export_schema(&schema_for!(amm_pair::ExecuteMsg), &amm_pair_out_dir);
    export_schema(
        &schema_for!(amm_pair::ExecuteMsgResponse),
        &amm_pair_out_dir,
    );
    export_schema(&schema_for!(amm_pair::InitMsg), &amm_pair_out_dir);
    export_schema(&schema_for!(amm_pair::QueryMsg), &amm_pair_out_dir);
    export_schema(&schema_for!(amm_pair::QueryMsgResponse), &amm_pair_out_dir);

    // factory
    let factory_out_dir = base_out_dir.join("factory");
    create_dir_all(&factory_out_dir).unwrap();
    export_schema(&schema_for!(factory::ExecuteMsg), &factory_out_dir);
    export_schema(&schema_for!(factory::InitMsg), &factory_out_dir);
    export_schema(&schema_for!(factory::QueryMsg), &factory_out_dir);
    export_schema(&schema_for!(factory::QueryResponse), &factory_out_dir);

    // lp_token
    let lp_token_out_dir = base_out_dir.join("lp_token");
    create_dir_all(&lp_token_out_dir).unwrap();
    export_schema(&schema_for!(lp_token::InstantiateMsg), &lp_token_out_dir);

    // router
    let router_out_dir = base_out_dir.join("router");
    create_dir_all(&router_out_dir).unwrap();
    export_schema(&schema_for!(router::ExecuteMsg), &router_out_dir);
    export_schema(&schema_for!(router::ExecuteMsgResponse), &router_out_dir);
    export_schema(&schema_for!(router::InitMsg), &router_out_dir);
    export_schema(&schema_for!(router::QueryMsg), &router_out_dir);
    export_schema(&schema_for!(router::QueryMsgResponse), &router_out_dir);

    // staking
    let staking_out_dir = base_out_dir.join("staking");
    create_dir_all(&staking_out_dir).unwrap();
    export_schema(&schema_for!(staking::ExecuteMsg), &staking_out_dir);
    export_schema(&schema_for!(staking::InitMsg), &staking_out_dir);
    export_schema(&schema_for!(staking::QueryMsg), &staking_out_dir);
    export_schema(&schema_for!(staking::QueryResponse), &staking_out_dir);

    //Admin
    let admin_out_dir = base_out_dir.join("admin");
    create_dir_all(&admin_out_dir).unwrap();
    export_schema(&schema_for!(admin::ExecuteMsg), &admin_out_dir);
    export_schema(&schema_for!(admin::QueryMsg), &admin_out_dir);
    export_schema(&schema_for!(admin::RegistryAction), &admin_out_dir);
    export_schema(&schema_for!(admin::InstantiateMsg), &admin_out_dir);
    export_schema(&schema_for!(admin::AdminsResponse), &admin_out_dir);
    export_schema(
        &schema_for!(admin::ValidateAdminPermissionResponse),
        &admin_out_dir,
    );
    export_schema(&schema_for!(admin::ConfigResponse), &admin_out_dir);
    export_schema(&schema_for!(admin::PermissionsResponse), &admin_out_dir);
    export_schema(&schema_for!(admin::AdminAuthStatus), &admin_out_dir);
}
