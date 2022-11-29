use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::collections::HashMap;

pub type PrincipalText = String;
pub type CanisterIdText = String;

pub type DaoID = u128;
#[derive(Deserialize, Serialize, Clone, CandidType)]
pub struct DaoInfo {
    pub id: DaoID,
    pub owner: Principal,
    pub canister_id: Principal,
    pub controller: Vec<Principal>,
    pub status: DaoStatusCode,
    pub create_at:u64,
    // pub dao_type: String, // different Type of dao, such as education, music
    pub tags: Vec<String>, // type
}

#[derive(Deserialize, Serialize, Clone, CandidType)]
pub enum DaoStatusCode {
    Active,
    Stopped
}

#[derive(Deserialize, Serialize, Default, Clone, CandidType)]
pub struct CreateDaoOptions {
    // name: String,                            // dao name
    // poster: String,                          // dao poster
    // avatar: String,                          // dao avatar
    // intro: String,                           // dao intro
    // option: Option<HashMap<String, String>>, // user custom expand field
    pub tags: Vec<String>, // dao tags
    pub block_height: u64, // block height
    pub memo: u64,         // memo, used to validate transfer
}

#[derive(Deserialize, Serialize, Default, Clone, CandidType)]
pub struct AddDaoInfo {
    name: String,                            // dao name
    poster: String,                          // dao poster
    avatar: String,                          // dao avatar
    pub tags: Vec<String>,                   // dao tags
    intro: String,                           // dao intro
    option: Option<HashMap<String, String>>, // user custom expand field
}

#[derive(Deserialize, Serialize, Clone, CandidType)]
pub enum ControllerAction {
    add(Principal),
    remove(Principal),
    clear,
}
