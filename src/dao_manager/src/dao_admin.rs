use crate::types::{ControllerAction, CreateDaoInfo, DaoID, DaoInfo, DaoStatusCode};
use candid::{Deserialize, Principal};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct DaoAdmin {
    pub dao_map: HashMap<DaoID, DaoInfo>,
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
    pub fn add_dao(&mut self, dao_id: DaoID, canister_id: Principal) -> Result<(), String> {
        self.dao_exist(dao_id)?;
        self.dao_map.insert(
            dao_id,
            DaoInfo {
                id: dao_id,
                owner: ic_cdk::caller(),
                canister_id,
                controller: vec![],
                status: DaoStatusCode::Normal,
                dao_type: "x".to_string(),
            },
        );
        Ok(())
    }
    pub fn create_dao(&self, info: CreateDaoInfo) -> Result<(), String> {
        todo!();
        // create dao
        let dao_id: DaoID = 1;
        let canister_id = Principal::from_text("xxx").unwrap();
        if self.dao_exist(dao_id).is_ok() {
            return Err(String::from("The current DAO already exists"));
        }
        self.dao_map.insert(
            dao_id,
            DaoInfo {
                id: dao_id,
                owner: ic_cdk::caller(),
                canister_id,
                controller: vec![],
                status: DaoStatusCode::Normal,
                dao_type: "x".to_string(),
            },
        );
        Ok(())
    }
    pub fn update_dao_controller(
        &mut self,
        dao_id: DaoID,
        action: ControllerAction,
    ) -> Result<(), String> {
        // if exist
        let dao = self
            .dao_map
            .get_mut(&dao_id)
            .expect("Current DAO does not exist");

        // validate owner
        if dao.owner != ic_cdk::caller() {
            return Err(String::from("Please authorize first"));
        }

        match action {
            ControllerAction::add(principal) => {
                for id in &dao.controller {
                    if principal == *id {
                        return Err(String::from("User Already an administrator"));
                    }
                }
                dao.controller.push(principal);
                Ok(())
            }
            ControllerAction::remove(principal) => {
                dao.controller.retain(|&x| x != principal);
                Ok(())
            }
            ControllerAction::clear => {
                dao.controller.clear();
                Ok(())
            }
        }
    }
}
