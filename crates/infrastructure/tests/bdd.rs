use cucumber::{given, then, when, World};

#[derive(Debug, Default, World)]
pub struct BankoWorld {
    pub last_result: Option<String>,
    pub last_error: Option<String>,
    pub entity_name: Option<String>,
    pub entity_id: Option<String>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
}

// --- Customer steps ---

#[given(expr = "a new customer with name {string} and email {string}")]
fn new_customer(world: &mut BankoWorld, name: String, email: String) {
    world.entity_name = Some(name);
    world.entity_id = Some(email);
}

#[when("I submit the customer onboarding form")]
fn submit_onboarding(world: &mut BankoWorld) {
    if let Some(ref email) = world.entity_id {
        if email.contains('@') {
            world.last_result = Some("pending_kyc".to_string());
        } else {
            world.last_error = Some("invalid_email".to_string());
        }
    }
}

#[then(expr = "the customer is created with status {string}")]
fn customer_created(world: &mut BankoWorld, status: String) {
    assert_eq!(world.last_result.as_deref(), Some(status.as_str()));
}

#[then(expr = "the onboarding is rejected with error {string}")]
fn onboarding_rejected(world: &mut BankoWorld, error: String) {
    assert_eq!(world.last_error.as_deref(), Some(error.as_str()));
}

// --- Account steps ---

#[given(expr = "a verified customer with id {string}")]
fn verified_customer(world: &mut BankoWorld, id: String) {
    world.entity_id = Some(id);
    world.last_result = Some("verified".to_string());
}

#[when("I open a current account in TND")]
fn open_account_tnd(world: &mut BankoWorld) {
    world.amount = Some(0.0);
    world.currency = Some("TND".to_string());
    world.last_result = Some("account_created".to_string());
}

#[then(expr = "the account is created with balance {float} TND")]
fn account_created_balance(world: &mut BankoWorld, balance: f64) {
    assert_eq!(world.amount, Some(balance));
}

#[given(expr = "an unverified customer with id {string}")]
fn unverified_customer(world: &mut BankoWorld, id: String) {
    world.entity_id = Some(id);
    world.last_result = Some("unverified".to_string());
}

#[when("I attempt to open a current account")]
fn attempt_open_account(world: &mut BankoWorld) {
    if world.last_result.as_deref() == Some("unverified") {
        world.last_error = Some("customer_not_verified".to_string());
    }
}

#[then(expr = "the account opening is rejected with error {string}")]
fn account_opening_rejected(world: &mut BankoWorld, error: String) {
    assert_eq!(world.last_error.as_deref(), Some(error.as_str()));
}

fn main() {
    let features_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/bdd/features");
    let runner = BankoWorld::run(features_path);
    futures::executor::block_on(runner);
}
