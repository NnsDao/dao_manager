mod canister;
mod canister_manager;
mod dao_admin;
mod heartbeat;
mod init;
mod owner;
mod types;

use dao_admin::DaoAdmin;
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk::export::candid::Principal;
use ic_cdk_macros::*;
use ic_kit::interfaces::management::CanisterStatusResponse;
use ic_kit::{ic, RejectionCode};
use owner::{is_owner, OwnerService};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::result::Result;
use std::string::String;
use types::{ControllerAction, CreateDaoInfo, DaoID, DaoInfo};

use crate::types::{CanisterIdText, PrincipalText};

#[derive(Default)]
pub struct Data {
    pub owners: OwnerService,
    pub dao_admin: DaoAdmin,
}

#[derive(Serialize, Deserialize, Default)]
pub struct DataV0 {
    #[serde(default)]
    pub owners: OwnerService,
    #[serde(default)]
    pub dao_admin: DaoAdmin,
}

#[query]
#[candid::candid_method(query)]
fn dao_list() -> Vec<DaoInfo> {
    ic::get::<Data>().dao_admin.dao_list()
}

#[query]
#[candid::candid_method(query)]
async fn dao_status(
    canister_id: CanisterIdText,
) -> Result<CanisterStatusResponse, (RejectionCode, String)> {
    ic::get::<Data>()
        .dao_admin
        .dao_status(Principal::from_text(canister_id).unwrap())
        .await
}

#[update]
#[candid::candid_method(update)]
fn add_dao(canister_id: CanisterIdText, info: CreateDaoInfo) -> Result<DaoInfo, String> {
    ic::get_mut::<Data>().dao_admin.add_dao(canister_id, info)
}

#[update]
#[candid::candid_method(update)]
async fn create_dao(info: CreateDaoInfo) -> Result<DaoInfo, String> {
    ic::get_mut::<Data>().dao_admin.create_dao(info).await
}

#[update]
#[candid::candid_method(update)]
async fn update_dao_controller(dao_id: DaoID, action: ControllerAction) -> Result<DaoInfo, String> {
    ic::get_mut::<Data>()
        .dao_admin
        .update_dao_controller(dao_id, action)
        .await
}

#[query(guard = "is_owner")]
#[candid::candid_method(query)]
fn get_owner() -> Vec<Principal> {
    ic::get::<Data>().owners.get_owners()
}

#[pre_upgrade]
fn pre_upgrade() {
    let data = ic::get::<Data>();

    let writer = StableWriter::default();
    serde_cbor::to_writer(
        writer,
        &DataV0 {
            owners: data.owners.clone(),
            dao_admin: data.dao_admin.clone(),
        },
    )
    .expect("Failed to serialize data.");
}

#[post_upgrade]
fn post_upgrade() {
    let reader = StableReader::default();

    let data: DataV0 = match serde_cbor::from_reader(reader) {
        Ok(t) => t,
        Err(err) => {
            let limit = err.offset() - 1;
            let reader = StableReader::default().take(limit);
            serde_cbor::from_reader(reader).expect("Failed to deserialize.")
        }
    };

    ic::store(Data {
        owners: data.owners,
        dao_admin: data.dao_admin,
    });
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::canister::ledger::icp_balance;

    use super::*;

    #[test]
    fn candid_transform() {
        let amount = 1_00_000_000_u128;
        println!("amount is {}", amount);
        let amount_bytes = amount.to_be_bytes();

        println!("bytes is {:?}", amount_bytes);
        match candid::Nat::parse(&amount_bytes) {
            Ok(num) => {
                println!("Nat is {}", num);
            }
            Err(err) => {
                println!("err occured is {}", err);
            }
        };
        let amount = amount.to_string();
        println!("string amount is {}", amount);
        match candid::Nat::from_str(&amount) {
            Ok(num) => {
                println!("Nat is {}", num);
            }
            Err(err) => {
                println!("err occured is {}", err);
            }
        };
    }
    #[test]
    fn get_icp() {
        async {
            let balance = icp_balance(
                Principal::from_text(
                    "c526v-pnjpe-x57vs-xe3qb-idgh7-xre3a-jdzef-l654c-5sg4x-5iigp-xae",
                )
                .unwrap(),
                None,
            )
            .await;

            match balance {
                Ok(num) => {
                    println!("num is {}", num);
                }
                Err(err) => {
                    println!("balance err is {}", err);
                }
            };
        };
    }
}
