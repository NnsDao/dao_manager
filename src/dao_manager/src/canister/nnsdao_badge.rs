// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.

use ic_cdk::export::candid::{self, CandidType, Deserialize};
use ic_cdk::api::call::CallResult;

type Token = String;
#[derive(CandidType, Deserialize)]
struct Badge {
  url: String,
  token: Token,
  data: String,
  desc: String,
  name: String,
  reputation: candid::Nat,
}

#[derive(CandidType, Deserialize)]
enum MintResp { ok(String), err(String) }

#[derive(CandidType, Deserialize)]
enum Result { ok(bool), err(String) }

type Address = String;
#[derive(CandidType, Deserialize)]
struct MintBadge { token: Token, addr: Address }

struct SERVICE(candid::Principal);
impl SERVICE{
  pub async fn getAllBadgeList(&self) -> CallResult<(Vec<(Token,Badge,)>,)> {
    ic_cdk::call(self.0, "getAllBadgeList", ()).await
  }
  pub async fn getUserBadgeList(&self, arg0: candid::Principal) -> CallResult<
    (Vec<Option<Badge>>,)
  > { ic_cdk::call(self.0, "getUserBadgeList", (arg0,)).await }
  pub async fn mintBadge(&self, arg0: MintBadge) -> CallResult<(MintResp,)> {
    ic_cdk::call(self.0, "mintBadge", (arg0,)).await
  }
}