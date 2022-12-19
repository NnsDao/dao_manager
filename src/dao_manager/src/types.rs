use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::collections::HashMap;

pub type CanisterIdText = String;

#[derive(Deserialize, Serialize, Clone, CandidType)]
pub struct DaoInfo {
    pub canister_id: PrincipalText,
}

#[derive(Deserialize, Serialize, Clone, CandidType)]
pub enum DaoStatusCode {
    Active,
    Stopped,
}

#[derive(Deserialize, Serialize, Default, Clone, CandidType)]
pub struct CreateDaoOptions {
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
}

pub type PrincipalText = String;
pub type Dao = Vec<PrincipalText>;
