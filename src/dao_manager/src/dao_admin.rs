use crate::canister_manager::{
    nnsdao_canister_status, nnsdao_change_controller, nnsdao_create_canister, nnsdao_install_code,
    nnsdao_reinstall_code, nnsdao_upgrade_code,
};
use crate::types::{CanisterIdText, ControllerAction, CreateDaoOptions, Dao};
use crate::Data;
use candid::{Deserialize, Principal};

use ic_kit::interfaces::management::CanisterStatusResponse;
use ic_kit::{ic, RejectionCode};
use serde::Serialize;
use std::vec;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct DaoAdmin {
    pub dao: Dao,
}
pub fn handle_tuple_err(err: (RejectionCode, String)) -> Result<(), String> {
    let (code, reason) = err;
    Err(format!("RejectionCode:{:?}, reason: {:?}", code, reason))
}

impl DaoAdmin {
    pub async fn upgrade_canister(&self, cid: String) -> Result<(), (RejectionCode, String)> {
        nnsdao_upgrade_code(Principal::from_text(cid).unwrap()).await?;
        // for canister_id in self.dao.iter() {
        // }
        Ok(())
    }
    pub async fn reinstall_canister(&self, cid: String) -> Result<(), (RejectionCode, String)> {
        let caller = ic_cdk::caller();
        nnsdao_reinstall_code(caller, Principal::from_text(cid).unwrap()).await?;
        // for canister_id in self.dao.iter() {
        // }
        Ok(())
    }
    pub async fn canister_status(
        &self,
        canister_id: Principal,
    ) -> Result<CanisterStatusResponse, (RejectionCode, String)> {
        nnsdao_canister_status(canister_id).await
    }
    fn dao_exist(&self, canister_id: Principal) -> Result<bool, String> {
        for id in &self.dao {
            if Principal::from_text(&id).unwrap() == canister_id {
                return Ok(true);
            }
        }
        Err("Current DAO does not exist".to_owned())
    }
    pub fn dao_list(&self) -> Dao {
        self.dao.clone()
    }
    pub fn add_dao(&mut self, canister_id: CanisterIdText) -> Dao {
        self.dao.push(canister_id);
        self.dao_list()
    }
    pub async fn create_dao(&mut self, info: CreateDaoOptions) -> Result<String, String> {
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

        let canister_id = canister_id.to_text();
        self.dao.push(canister_id.clone());
        // set transaction status 1
        ic::get_mut::<Data>()
            .icp_service
            .validate_transfer(caller, info.block_height, info.memo, Some(1))
            .await?;
        Ok(canister_id)
    }
    pub async fn update_dao_controller(&mut self, action: ControllerAction) -> Result<(), String> {
        let mut owners = ic::get::<Data>().owners.get_owners();

        match action {
            ControllerAction::add(principal) => {
                owners.push(principal);
            }
            ControllerAction::remove(principal) => {
                owners.retain(|&x| x != principal);
            }
        };

        nnsdao_change_controller(owners, ic_cdk::id())
            .await
            .or_else(handle_tuple_err)
    }
}
