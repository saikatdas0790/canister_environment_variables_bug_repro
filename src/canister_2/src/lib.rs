use candid::export_service;

#[cfg(test)]
mod test;

#[ic_cdk_macros::query]
#[candid::candid_method(query)]
fn log_some_env_variables() {
    ic_cdk::println!("{:?}", option_env!("DFX_NETWORK"));
    ic_cdk::println!("{:?}", option_env!("CANISTER_ID_canister_1"));
    ic_cdk::println!("{:?}", option_env!("CANISTER_ID_canister_2"));
}

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}
