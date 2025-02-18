//! MinerApp submit_tx module
#![forbid(unsafe_code)]
use anyhow::Error;
use cli::{libra_client::LibraClient, AccountData, AccountStatus};
use txs::{sign_tx::sign_tx, submit_tx::{TxParams, submit_tx}};
use libra_types::transaction::{Script};
use libra_json_rpc_types::views::{TransactionView};
use libra_types::chain_id::ChainId;

/// Submit a miner transaction to the network.
pub fn commit_proof_tx(
    tx_params: &TxParams,
    preimage: Vec<u8>,
    proof: Vec<u8>,
    is_operator: bool,
) -> Result<TransactionView, Error> {

    // Create a client object
    let mut client = LibraClient::new(tx_params.url.clone(), tx_params.waypoint).unwrap();

    let chain_id = ChainId::new(client.get_metadata().unwrap().chain_id);

    // For sequence number
    let (account_state,_) = client.get_account(tx_params.signer_address.clone(), true).unwrap();
    let sequence_number = match account_state {
        Some(av) => av.sequence_number,
        None => 0,
    };

    let script: Script = if is_operator {
        transaction_builder::encode_minerstate_commit_by_operator_script(tx_params.owner_address.clone(), preimage, proof)
    } else {
        // if owner sending with mnemonic
        transaction_builder::encode_minerstate_commit_script(preimage, proof)
    };

    let signed_tx = sign_tx(&script, tx_params, sequence_number, chain_id )?;

        // get account_data struct
    let mut signer_account_data = AccountData {
        address: tx_params.signer_address,
        authentication_key: Some(tx_params.auth_key.to_vec()),
        key_pair: Some(tx_params.keypair.clone()),
        sequence_number,
        status: AccountStatus::Persisted,
    };

    submit_tx(client, signed_tx, &mut signer_account_data )

}
