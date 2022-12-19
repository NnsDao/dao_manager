mod canister;
mod canister_manager;
mod dao_admin;
mod heartbeat;
mod init;
mod owner;
pub mod tool;
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
use types::{ControllerAction, CreateDaoOptions, DaoInfo};

use crate::canister::ledger::{ICPService, TransactionItem};
use crate::types::{CanisterIdText, Dao};

#[derive(Default)]
pub struct Data {
    pub owners: OwnerService,
    pub dao_admin: DaoAdmin,
    pub icp_service: ICPService,
}

#[derive(Serialize, Deserialize, Default)]
pub struct DataV0 {
    #[serde(default)]
    pub owners: OwnerService,
    #[serde(default)]
    pub dao_admin: DaoAdmin,
    #[serde(default)]
    pub icp_service: ICPService,
}

#[query]
#[candid::candid_method(query)]
fn dao_list() -> Dao {
    ic::get::<Data>().dao_admin.dao_list()
}

#[update]
#[candid::candid_method(update)]
async fn canister_status() -> Result<CanisterStatusResponse, (RejectionCode, String)> {
    ic::get::<Data>()
        .dao_admin
        .canister_status(ic_cdk::id())
        .await
}

#[query]
#[candid::candid_method(query)]
fn transaction_log() -> Vec<TransactionItem> {
    ic::get::<Data>().icp_service.transaction_log()
}

#[update]
#[candid::candid_method(update)]
async fn get_pay_info() -> Result<TransactionItem, String> {
    ic::get_mut::<Data>().icp_service.get_pay_info().await
}

#[update]
#[candid::candid_method(update)]
fn add_dao(canister_id: CanisterIdText) -> Dao {
    ic::get_mut::<Data>().dao_admin.add_dao(canister_id)
}

#[update]
#[candid::candid_method(update)]
async fn create_dao(info: CreateDaoOptions) -> Result<String, String> {
    ic::get_mut::<Data>().dao_admin.create_dao(info).await
}

#[update(guard = "is_owner")]
#[candid::candid_method(update)]
async fn update_dao_controller(action: ControllerAction) -> Result<(), String> {
    ic::get_mut::<Data>()
        .dao_admin
        .update_dao_controller(action)
        .await
}

#[update(guard = "is_owner")]
#[candid::candid_method(update)]
fn add_owner() -> Vec<Principal> {
    ic::get_mut::<Data>().owners.add_owner(ic_cdk::caller())
}

#[query(guard = "is_owner")]
#[candid::candid_method(query)]
fn get_owner() -> Vec<Principal> {
    ic::get::<Data>().owners.get_owners()
}

#[update(guard = "is_owner")]
#[candid::candid_method(update)]
async fn upgrade_canister() -> Result<(), (RejectionCode, String)> {
    ic::get::<Data>().dao_admin.upgrade_canister().await
}

#[update(guard = "is_owner")]
#[candid::candid_method(update)]
async fn reinstall_canister() -> Result<(), (RejectionCode, String)> {
    ic::get::<Data>().dao_admin.reinstall_canister().await
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
            icp_service: data.icp_service.clone(),
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
        icp_service: data.icp_service,
    });
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
