use ic_cdk_macros::heartbeat;

#[heartbeat]
fn heartbeat() {
   ic_cdk::println!("heat");
}