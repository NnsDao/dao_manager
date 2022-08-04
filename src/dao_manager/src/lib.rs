mod init;
mod owner;
mod heartbeat;

use std::io::Read;
use ic_kit::ic;
use ic_cdk_macros::*;
use ic_cdk::export::candid::Principal;
use ic_cdk::api::stable::{StableReader, StableWriter};
use owner::{is_owner, OwnerService};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct Data {
    pub owners: OwnerService,
}

#[derive(Serialize, Deserialize, Default)]
pub struct DataV0 {
    #[serde(default)]
    pub owners: OwnerService,
}

#[query]
#[candid::candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[query(guard = "is_owner")]
#[candid::candid_method(query)]
fn get_owner() -> Vec<Principal> {
    ic::get::<Data>()
    .owners
    .get_owners()
}

#[pre_upgrade]
fn pre_upgrade() {
    let data = ic::get::<Data>();

    let writer = StableWriter::default();
    serde_cbor::to_writer(
        writer,
        &DataV0 {
            owners: data.owners.clone(),
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
    });
}


candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
