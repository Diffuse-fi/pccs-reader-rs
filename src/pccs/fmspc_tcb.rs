use anyhow::Result;

use std::env;

use alloy::{
    primitives::{Address, U256},
    providers::ProviderBuilder,
    sol,
};

sol! {
    #[sol(rpc)]
    interface IFmspcTcbDao {
        #[derive(Debug)]
        struct TcbInfoJsonObj {
            string tcbInfoStr;
            bytes signature;
        }

        #[derive(Debug)]
        function getTcbInfo(uint256 tcbType, string calldata fmspc, uint256 version) returns (TcbInfoJsonObj memory tcbObj);
    }
}

pub async fn get_tcb_info(tcb_type: u8, fmspc: &str, version: u32) -> Result<Vec<u8>> {
    let rpc_url = env::var("RPC_URL").expect("RPC_URL env var not set").parse().expect("Invalid RPC URL format");
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let mut fmspc_tcb_dao_address = env::var("FMSPC_TCB_DAO")
        .expect("FMSPC_TCB_DAO env var not set");
    fmspc_tcb_dao_address = fmspc_tcb_dao_address.trim_start_matches("0x").to_string();

    let fmspc_tcb_dao_address_slice =
        hex::decode(fmspc_tcb_dao_address).expect("Invalid address hex");

    let fmspc_tcb_dao_contract =
        IFmspcTcbDao::new(Address::from_slice(&fmspc_tcb_dao_address_slice), &provider);

    let call_builder = fmspc_tcb_dao_contract.getTcbInfo(
        U256::from(tcb_type),
        String::from(fmspc),
        U256::from(version),
    );

    let call_return = call_builder.call().await?;
    let tcb_info_str = call_return.tcbObj.tcbInfoStr;
    let signature_bytes = call_return.tcbObj.signature;

    if tcb_info_str.len() == 0 || signature_bytes.len() == 0 {
        return Err(anyhow::Error::msg("missing"));
    }

    let signature = signature_bytes.to_string();

    let ret_str = format!(
        "{{\"tcbInfo\": {}, \"signature\": \"{}\"}}",
        tcb_info_str,
        remove_prefix_if_found(signature.as_str())
    );

    let ret = ret_str.into_bytes();
    Ok(ret)
}

fn remove_prefix_if_found(h: &str) -> &str {
    if h.starts_with("0x") {
        &h[2..]
    } else {
        &h
    }
}