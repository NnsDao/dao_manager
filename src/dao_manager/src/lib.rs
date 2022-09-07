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
use types::{ControllerAction, CreateDaoInfo, DaoID, DaoInfo};

use crate::canister::ledger::{ICPService, TransactionItem};
use crate::types::{AddDaoInfo, CanisterIdText};

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
async fn get_pay_info() -> Result<TransactionItem, String> {
    ic::get_mut::<Data>().icp_service.get_pay_info().await
}

#[update]
#[candid::candid_method(update)]
fn add_dao(canister_id: CanisterIdText, info: AddDaoInfo) -> Result<DaoInfo, String> {
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
