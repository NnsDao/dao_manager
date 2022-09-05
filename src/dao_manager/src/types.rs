use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::collections::HashMap;

pub type PrincipalText = String;

pub type DaoID = u128;
#[derive(Deserialize, Serialize, Clone, CandidType)]
pub struct DaoInfo {
    pub id: DaoID,
    pub owner: Principal,
    pub canister_id: Principal,
    pub controller: Vec<Principal>,
    pub status: DaoStatusCode,
    // pub dao_type: String, // different Type of dao, such as education, music
    pub tags: Vec<String>, // type
}

#[derive(Deserialize, Serialize, Clone, CandidType)]
pub enum DaoStatusCode {
    Normal = 0,
    Closed = -1,
}

#[derive(Deserialize, Serialize, Default, Clone, CandidType)]
pub struct CreateDaoInfo {
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
pub type Canister_id_text = String;
