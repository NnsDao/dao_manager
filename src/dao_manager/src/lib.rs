mod dao_admin;
mod heartbeat;
mod init;
mod owner;
mod types;
mod canister_manager;

use dao_admin::DaoAdmin;
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk::export::candid::Principal;
use ic_cdk_macros::*;
use ic_kit::ic;
use owner::{is_owner, OwnerService};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::result::Result;
use std::string::String;
use types::{ControllerAction, CreateDaoInfo, DaoID, DaoInfo};

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
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[query]
#[candid::candid_method(query)]
fn dao_list() -> Vec<DaoInfo> {
    ic::get::<Data>().dao_admin.dao_list()
}
#[update]
#[candid::candid_method(update)]
fn add_dao(dao_id: DaoID, canister_id: Principal) -> Result<(), String> {
    ic::get_mut::<Data>().dao_admin.add_dao(dao_id, canister_id)
}

#[update]
#[candid::candid_method(update)]
fn create_dao(info: CreateDaoInfo) -> Result<(), String> {
    ic::get_mut::<Data>().dao_admin.create_dao(info)
}

#[update]
#[candid::candid_method(update)]
fn update_dao_controller(dao_id: DaoID, action: ControllerAction) -> Result<(), String> {
    ic::get_mut::<Data>()
        .dao_admin
        .update_dao_controller(dao_id, action)
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
