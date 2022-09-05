use crate::canister_manager::{nnsdao_change_controller, nnsdao_create_canister};
use crate::types::{
    Canister_id_text, ControllerAction, CreateDaoInfo, DaoID, DaoInfo, DaoStatusCode,
};
use candid::types::ic_types::principal;
use candid::{Deserialize, Principal};
use ic_kit::RejectionCode;
use serde::Serialize;
use std::collections::HashMap;
use std::vec;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct DaoAdmin {
    pub dao_map: HashMap<DaoID, DaoInfo>,
    pub dao_index: DaoID,
}
pub fn handle_tuple_err(err: (RejectionCode, String)) -> Result<(), String> {
    let (code, reason) = err;
    Err(format!("RejectionCode:{:?}, reason: {:?}", code, reason))
}

impl DaoAdmin {
    fn dao_exist(&self, dao_id: DaoID) -> Result<(), String> {
        self.dao_map
            .get(&dao_id)
            .expect("Current DAO does not exist");
        Ok(())
    }
    pub fn dao_list(&self) -> Vec<DaoInfo> {
        self.dao_map.clone().into_values().collect()
    }
    pub fn add_dao(
        &mut self,
        canister_id: Canister_id_text,
        info: CreateDaoInfo,
    ) -> Result<DaoInfo, String> {
        self.dao_index += 1;
        let dao_id = self.dao_index;
        self.dao_exist(dao_id)?;
        let canister_id = Principal::from_text(canister_id).unwrap();
        let dao_info = DaoInfo {
            id: dao_id,
            owner: ic_cdk::caller(),
            canister_id,
            controller: vec![canister_id],
            status: DaoStatusCode::Normal,
            tags: info.tags,
        };
        self.dao_map.insert(dao_id, dao_info.clone());
        Ok(dao_info)
    }
    pub async fn create_dao(&mut self, info: CreateDaoInfo) -> Result<DaoInfo, String> {
        // create dao
        self.dao_index += 1;
        let dao_id = self.dao_index;
        let caller = ic_cdk::caller();
        let canister_id: Principal;
        unimplemented!("need cycles params");
        match nnsdao_create_canister(vec![caller], 1).await {
            Ok(canister) => {
                canister_id = canister;
            }
            Err(err) => handle_tuple_err(err)?,
        }

        let dao_info = DaoInfo {
            id: dao_id,
            owner: caller,
            canister_id,
            controller: vec![canister_id],
            status: DaoStatusCode::Normal,
            tags: info.tags,
        };

        // if self.dao_exist(dao_id).is_ok() {
        //     return Err(String::from("The current DAO already exists"));
        // }
        self.dao_map.insert(dao_id, dao_info);
        Ok(dao_info)
    }
    pub async fn update_dao_controller(
        &mut self,
        dao_id: DaoID,
        action: ControllerAction,
    ) -> Result<DaoInfo, String> {
        // if exist
        let dao = self
            .dao_map
            .get_mut(&dao_id)
            .expect("Current DAO does not exist");

        // validate owner
        if dao.owner != ic_cdk::caller() {
            return Err(String::from("Only owners has permission to operate"));
        }
        nnsdao_change_controller(dao.controller.clone(), dao.canister_id)
            .await
            .or_else(handle_tuple_err)?;

        match action {
            ControllerAction::add(principal) => {
                for id in &dao.controller {
                    if principal == *id {
                        return Err(String::from("User already an administrator"));
                    }
                }
                dao.controller.push(principal);
                Ok(dao.clone())
            }
            ControllerAction::remove(principal) => {
                dao.controller.retain(|&x| x != principal);
                Ok(dao.clone())
            }
            ControllerAction::clear => {
                dao.controller.clear();
                Ok(dao.clone())
            }
        }
    }
}
