use crate::canister_manager::{
    nnsdao_canister_status, nnsdao_change_controller, nnsdao_create_canister, nnsdao_install_code,
    nnsdao_reinstall_code, nnsdao_upgrade_code,
};
use crate::types::{
     CanisterIdText, ControllerAction, CreateDaoOptions, DaoInfo, DaoStatusCode,
};
use crate::Data;
use candid::{Deserialize, Principal};
use ic_cdk::api::time;
use ic_cdk::caller;
use ic_kit::interfaces::management::CanisterStatusResponse;
use ic_kit::{ic, RejectionCode};
use serde::Serialize;
use std::collections::HashMap;
use std::vec;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct DaoAdmin {
    pub dao_map: HashMap<Principal, DaoInfo>,
}
pub fn handle_tuple_err(err: (RejectionCode, String)) -> Result<(), String> {
    let (code, reason) = err;
    Err(format!("RejectionCode:{:?}, reason: {:?}", code, reason))
}

impl DaoAdmin {
    pub async fn upgrade_canister(&self) -> Result<(), (RejectionCode, String)> {
        for (_, dao_info) in self.dao_map.iter() {
            nnsdao_upgrade_code(dao_info.canister_id).await?
        }
        Ok(())
    }
    pub async fn reinstall_canister(&self) -> Result<(), (RejectionCode, String)> {
        let caller = ic_cdk::caller();
        for (_, dao_info) in self.dao_map.iter() {
            nnsdao_reinstall_code(caller, dao_info.canister_id).await?
        }
        Ok(())
    }
    pub async fn dao_status(
        &self,
        canister_id: Principal,
    ) -> Result<CanisterStatusResponse, (RejectionCode, String)> {
        nnsdao_canister_status(canister_id).await
    }
    fn dao_exist(&self, canister_id:Principal) -> Result<bool, String> {
        self.dao_map
            .get(&canister_id)
            .ok_or("Current DAO does not exist")?;
        Ok(true)
    }
    pub fn dao_list(&self) -> Vec<DaoInfo> {
        self.dao_map.clone().into_values().collect()
    }
    pub fn add_dao(
        &mut self,
        canister_id: CanisterIdText,
    ) -> Result<DaoInfo, String> {

        let canister_id = Principal::from_text(canister_id).unwrap();
        let dao_info = DaoInfo {
            owner: ic_cdk::caller(),
            canister_id,
            controller: vec![ic_cdk::caller()],
            status: DaoStatusCode::Active,
            create_at: time(),
        };
        self.dao_map.insert(canister_id, dao_info.clone());
        Ok(dao_info)
    }
    pub async fn create_dao(&mut self, info: CreateDaoOptions) -> Result<DaoInfo, String> {
        // create dao


        let caller = ic_cdk::caller();

        // validate transfer
        // transer 1ICP
        ic::get_mut::<Data>()
            .icp_service
            .validate_transfer(caller, info.block_height, info.memo, None)
            .await?;

        // 1T
        let cycles = 1_000_000_000_000;

        let canister_id = nnsdao_create_canister(vec![caller], cycles)
            .await
            .map_err(|err| {
                let (code, reason) = err;
                format!("RejectionCode:{:?}, reason: {:?}", code, reason)
            })?;

        nnsdao_install_code(caller, canister_id)
            .await
            .map_err(|err| {
                let (code, reason) = err;
                format!("RejectionCode:{:?}, reason: {:?}", code, reason)
            })?;

        let dao_info = DaoInfo {
            owner: caller,
            canister_id,
            controller: vec![caller],
            status: DaoStatusCode::Active,
            create_at: time()
            
        };
        self.dao_map.insert(canister_id, dao_info.clone());
        // set transaction status 1
        ic::get_mut::<Data>()
            .icp_service
            .validate_transfer(caller, info.block_height, info.memo, Some(1))
            .await?;
        Ok(dao_info)
    }
    pub async fn update_dao_controller(
        &mut self,
        canister_id: Principal,
        action: ControllerAction,
    ) -> Result<DaoInfo, String> {
        // if exist
        let dao = self
            .dao_map
            .get_mut(&canister_id)
            .expect("Current DAO does not exist");

        // validate controller
        if !dao.controller.contains(&caller()) {
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
