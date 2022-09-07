use candid::{CandidType, Deserialize};
use dfn_core::api::call_with_cleanup;
use dfn_protobuf::{protobuf, ProtoBuf};

use ic_cdk::export::candid::Principal;
use on_wire::FromWire;

use ic_ledger_types::AccountIdentifier;
use ic_nns_constants::LEDGER_CANISTER_ID;

use ledger_canister::{Block, BlockArg, BlockRes, Memo, Operation};
use serde::Serialize;

use crate::tool::subnet_raw_rand;

#[derive(Serialize, CandidType, Deserialize, Default, Clone)]
pub struct ICPService {
    pub transactions: Vec<TransactionItem>,
}

impl ICPService {
    pub async fn get_pay_info(&mut self) -> Result<TransactionItem, String> {
        let to = AccountIdentifier::new(&ic_cdk::api::id(), &ic_ledger_types::DEFAULT_SUBACCOUNT)
            .to_string();
        let caller = ic_cdk::caller();
        let from =
            AccountIdentifier::new(&caller, &ic_ledger_types::DEFAULT_SUBACCOUNT).to_string();

        let memo = subnet_raw_rand().await?;
        let amount = 100_000_000_u64;
        let item = TransactionItem {
            from,
            to,
            memo,
            amount,
            status: 0,
        };
        self.transactions.push(item.clone());
        Ok(item)
    }
    pub async fn validate_transfer(
        &mut self,
        block_height: u64,
        memo: u64,
    ) -> Result<bool, String> {
        let caller = ic_cdk::caller();
        let from =
            AccountIdentifier::new(&caller, &ic_ledger_types::DEFAULT_SUBACCOUNT).to_string();
        let to = AccountIdentifier::new(&ic_cdk::api::id(), &ic_ledger_types::DEFAULT_SUBACCOUNT)
            .to_string();

        for transaction in &mut self.transactions {
            if transaction.from == from
                && memo == transaction.memo
                && transaction.status == 0
                && transaction.to == to
            {
                // transaction.status = 1; after crated dao, set to 1
                return check_transfer(from, to, block_height, memo, transaction.amount).await;
            }
        }
        Err(format!(
            "Invalid transfer params, block_height: {},memo: {}",
            block_height, memo
        ))
    }
}

#[derive(Serialize, Clone, CandidType, Deserialize, Default)]
pub struct TransactionItem {
    from: String,
    to: String,
    memo: u64,
    amount: u64,
    status: u8, // 0 to_pay | 1 paid
}

pub async fn get_block(block_height: u64) -> Result<Block, String> {
    let BlockRes(res) = call_with_cleanup(
        LEDGER_CANISTER_ID,
        "block_pb",
        protobuf,
        BlockArg(block_height),
    )
    .await
    .map_err(|e| format!("Failed to fetch block {}", e.1))?;
    let res = res.ok_or("Block not found")?;

    res.map_or_else(
        |canister_id| Err(format!("canisterId is {:?}", canister_id)),
        |encoded_block| {
            let bytes = encoded_block.into_vec();
            Ok(ProtoBuf::from_bytes(bytes)?.get())
        },
    )
}
pub async fn check_transfer(
    payer: String,
    receiver: String,
    block_height: u64,
    memo: u64,
    price: u64,
) -> Result<bool, String> {
    let block = get_block(block_height).await?;
    match block.transaction.operation {
        Operation::Transfer {
            from, to, amount, ..
        } => {
            if to.to_string() == receiver
                && from.to_string() == payer
                && amount.get_e8s() == price
                && block.transaction.memo == Memo(memo)
            {
                Ok(true)
            } else {
                Err("Transaction discipline query failed".to_string())
            }
        }
        _ => Err("Transaction discipline query failed".to_string()),
    }
}
