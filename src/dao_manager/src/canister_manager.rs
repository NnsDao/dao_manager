use candid::Principal;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::api::management_canister::main::*;
use ic_kit::candid::encode_args;
use ic_kit::candid::CandidType;
use ic_kit::interfaces::management::InstallMode;
use ic_kit::interfaces::management::{CanisterStatus, CanisterStatusResponse, WithCanisterId};
use ic_kit::{ic, interfaces::Method};
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct InstallCodeArgumentBorrowed<'a> {
    pub mode: InstallMode,
    pub canister_id: Principal,
    #[serde(with = "serde_bytes")]
    pub wasm_module: &'a [u8],
    pub arg: Vec<u8>,
}

pub const WASM: &[u8] = include_bytes!("./dao/nnsdao.wasm.gz");

/// Create a default store of 1T cycles
/// 1T = 1_000_000_000_000
pub async fn nnsdao_create_canister(
    mut controllers: Vec<Principal>,
    cycles: u128,
) -> Result<Principal, (RejectionCode, String)> {
    controllers.push(ic::id());
    let canister_id = create_canister(CreateCanisterArgument {
        settings: Some(CanisterSettings {
            controllers: Some(controllers),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        }),
    })
    .await?
    .0
    .canister_id;

    let need_deposit = cycles - 1_000_000_000_000;

    if cycles > 0 && need_deposit > 0 {
        nnsdao_deposit_cycles(canister_id, need_deposit).await?;
    }

    Ok(canister_id)
}

pub async fn nnsdao_change_controller(
    mut controllers: Vec<Principal>,
    canister_id: Principal,
) -> Result<(), (RejectionCode, String)> {
    controllers.push(ic::id());
    update_settings(UpdateSettingsArgument {
        canister_id,
        settings: CanisterSettings {
            controllers: Some(controllers),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        },
    })
    .await?;

    Ok(())
}

pub async fn nnsdao_install_code(
    owner: Principal,
    canister_id: Principal,
) -> Result<(), (RejectionCode, String)> {
    let arg = encode_args((owner,)).expect("Failed to serialize the install argument.");
    let install_config = InstallCodeArgumentBorrowed {
        mode: InstallMode::Install,
        canister_id,
        wasm_module: WASM,
        arg,
    };
    ic::call(
        Principal::management_canister(),
        "install_code",
        (install_config,),
    )
    .await?;

    Ok(())
}

pub async fn nnsdao_reinstall_code(
    owner: Principal,
    canister_id: Principal,
) -> Result<(), (RejectionCode, String)> {
    let arg = encode_args((owner,)).expect("Failed to serialize the install argument.");
    let install_config = InstallCodeArgumentBorrowed {
        mode: InstallMode::Reinstall,
        canister_id,
        wasm_module: WASM,
        arg,
    };
    let _: () = ic::call(
        Principal::management_canister(),
        "install_code",
        (install_config,),
    )
    .await?;

    Ok(())
}

pub async fn nnsdao_upgrade_code(canister_id: Principal) -> Result<(), (RejectionCode, String)> {
    let install_config = InstallCodeArgumentBorrowed {
        mode: InstallMode::Upgrade,
        canister_id,
        wasm_module: WASM,
        arg: vec![],
    };
    let _: () = ic::call(
        Principal::management_canister(),
        "install_code",
        (install_config,),
    )
    .await?;

    Ok(())
}

pub async fn nnsdao_canister_status(
    canister_id: Principal,
) -> Result<CanisterStatusResponse, (RejectionCode, String)> {
    let status = CanisterStatus::perform(
        Principal::management_canister(),
        (WithCanisterId { canister_id },),
    )
    .await?
    .0;
    Ok(status)
}

pub async fn nnsdao_deposit_cycles(
    canister_id: Principal,
    cycles: u128,
) -> Result<(), (RejectionCode, String)> {
    deposit_cycles(CanisterIdRecord { canister_id }, cycles).await
}
