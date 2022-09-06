use ic_cdk::export::candid::Principal;

use ic_ledger_types::{
    AccountBalanceArgs, AccountIdentifier, BlockIndex, Memo, Subaccount, Tokens, TransferArgs,
    DEFAULT_FEE, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};

pub async fn icp_balance(
    user: Principal,
    user_subaccount: Option<Subaccount>,
) -> Result<u128, String> {
    let arg = AccountBalanceArgs {
        account: AccountIdentifier::new(&user, &user_subaccount.unwrap_or(DEFAULT_SUBACCOUNT)),
    };

    let tokens = ic_ledger_types::account_balance(MAINNET_LEDGER_CANISTER_ID, arg)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e));

    match tokens {
        Ok(t) => Ok(t.e8s() as u128),
        Err(err) => Err(err),
    }

    // return Ok(100000000);
}

pub async fn icp_transfer(
    from_sub_account: Option<Subaccount>,
    to: Principal,
    to_sub_account: Option<Subaccount>,
    amount: u64,
    memo: Memo,
) -> Result<BlockIndex, String> {
    let arg = TransferArgs {
        memo,
        amount: Tokens::from_e8s(amount),
        fee: DEFAULT_FEE,
        from_subaccount: from_sub_account,
        to: AccountIdentifier::new(&to, &to_sub_account.unwrap_or(DEFAULT_SUBACCOUNT)),
        created_at_time: None,
    };

    ic_ledger_types::transfer(MAINNET_LEDGER_CANISTER_ID, arg)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e))?
        .map_err(|e| format!("ledger transfer error {:?}", e))
}
