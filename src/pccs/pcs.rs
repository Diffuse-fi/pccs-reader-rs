use anyhow::Result;

use std::env;

use alloy::{primitives::Address, providers::ProviderBuilder, sol};

sol! {
    #[sol(rpc)]
    interface IPCSDao {
        #[derive(Debug)]
        enum CA {
            ROOT,
            PROCESSOR,
            PLATFORM,
            SIGNING
        }

        #[derive(Debug)]
        function getCertificateById(CA ca) external view returns (bytes memory cert, bytes memory crl);
    }
}

pub async fn get_certificate_by_id(ca_id: IPCSDao::CA) -> Result<(Vec<u8>, Vec<u8>)> {
    let rpc_url = env::var("RPC_URL").expect("RPC_URL env var not set").parse().expect("Invalid RPC URL format");
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let mut pcs_dao_address = env::var("PCS_DAO")
        .expect("PCS_DAO env var not set");
    pcs_dao_address = pcs_dao_address.trim_start_matches("0x").to_string();

    let pcs_dao_address_slice =
        hex::decode(pcs_dao_address).expect("Invalid address hex");

    let pcs_dao_contract = IPCSDao::new(Address::from_slice(&pcs_dao_address_slice), &provider);

    let call_builder = pcs_dao_contract.getCertificateById(ca_id);

    let call_return = call_builder.call().await?;

    let cert = call_return.cert.to_vec();
    let crl = call_return.crl.to_vec();

    Ok((cert, crl))
}
