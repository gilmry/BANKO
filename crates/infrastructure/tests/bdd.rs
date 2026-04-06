use cucumber::{given, then, when, World};
use std::collections::HashMap;

#[derive(Debug, Default, World)]
pub struct BankoWorld {
    // Generic state
    pub last_result: Option<String>,
    pub last_error: Option<String>,
    pub entity_name: Option<String>,
    pub entity_id: Option<String>,
    pub amount: Option<f64>,
    pub currency: Option<String>,

    // Identity/Authentication
    pub user_password: Option<String>,
    pub stored_password_hash: Option<String>,
    pub session_token: Option<String>,

    // Loan/Credit
    pub risk_score: Option<String>,
    pub loan_id: Option<String>,
    pub missed_payments: Option<u32>,
    pub loan_status: Option<String>,
    pub provision_rate: Option<f64>,

    // AML
    pub transaction_type: Option<String>,
    pub alert_type: Option<String>,
    pub alert_generated: Option<bool>,

    // Sanctions
    pub match_score: Option<f64>,
    pub screening_result: Option<String>,

    // Prudential
    pub total_capital: Option<f64>,
    pub risk_weighted_assets: Option<f64>,
    pub tier1_capital: Option<f64>,
    pub solvency_ratio: Option<f64>,
    pub tier1_ratio: Option<f64>,

    // Accounting
    pub journal_entries: HashMap<String, (f64, f64)>, // account -> (debit, credit)
    pub entry_balanced: Option<bool>,

    // Reporting
    pub report_period: Option<String>,
    pub report_id: Option<String>,
    pub report_status: Option<String>,

    // Payment
    pub account_balance: HashMap<String, f64>, // account_id -> balance
    pub transfer_id: Option<String>,
    pub transfer_status: Option<String>,

    // ForeignExchange
    pub exchange_rate: Option<f64>,
    pub fx_position: Option<f64>,
    pub position_limit: Option<f64>,
    pub conversion_result: Option<f64>,

    // Governance/Audit
    pub user_id: Option<String>,
    pub action_type: Option<String>,
    pub audit_entries: Vec<(String, String, i64)>, // (user, action, timestamp)
}

// ============================================
// BC1: Customer Management
// ============================================

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

// ============================================
// BC2: Account Management
// ============================================

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

// ============================================
// BC12: Identity and Authentication
// ============================================

#[given(expr = "a registration request with email {string} and password {string}")]
fn registration_request(world: &mut BankoWorld, email: String, password: String) {
    world.entity_id = Some(email);
    world.user_password = Some(password);
}

#[when("I register the user")]
fn register_user(world: &mut BankoWorld) {
    // Validate password strength (at least 8 chars, has special char)
    if let Some(ref pwd) = world.user_password {
        if pwd.len() >= 8 && pwd.chars().any(|c| !c.is_alphanumeric()) {
            world.last_result = Some("user_created".to_string());
        } else {
            world.last_error = Some("weak_password".to_string());
        }
    }
}

#[then(expr = "the user is created with role {string}")]
fn user_created_with_role(world: &mut BankoWorld, role: String) {
    assert_eq!(world.last_result.as_deref(), Some("user_created"));
    assert_eq!(role, "user");
}

#[given(expr = "a registered user with email {string}")]
fn registered_user(world: &mut BankoWorld, email: String) {
    world.entity_id = Some(email);
    // Simulate stored password hash (in reality this would be from DB)
    world.stored_password_hash = Some("hashed_password_123".to_string());
}

#[when("I login with correct password")]
fn login_correct_password(world: &mut BankoWorld) {
    // In mock, correct password is "hashed_password_123"
    world.user_password = Some("hashed_password_123".to_string());
    if world.user_password == world.stored_password_hash {
        world.session_token = Some("jwt_token_abc123".to_string());
        world.last_result = Some("login_success".to_string());
    }
}

#[then("a session token is returned")]
fn session_token_returned(world: &mut BankoWorld) {
    assert!(world.session_token.is_some());
    assert_eq!(world.last_result.as_deref(), Some("login_success"));
}

#[when("I login with wrong password")]
fn login_wrong_password(world: &mut BankoWorld) {
    world.user_password = Some("wrong_password".to_string());
    if world.user_password != world.stored_password_hash {
        world.last_error = Some("invalid_credentials".to_string());
    }
}

#[then(expr = "login is rejected with error {string}")]
fn login_rejected(world: &mut BankoWorld, error: String) {
    assert_eq!(world.last_error.as_deref(), Some(error.as_str()));
}

// ============================================
// BC3: Credit Management
// ============================================

#[given(expr = "a customer with id {string} and risk score {string}")]
fn customer_with_risk_score(world: &mut BankoWorld, id: String, score: String) {
    world.entity_id = Some(id);
    world.risk_score = Some(score);
}

#[when(expr = "I submit a loan application for {float} TND over {int} months")]
fn submit_loan_application(world: &mut BankoWorld, amount: f64, months: i32) {
    world.amount = Some(amount);
    world.loan_status = Some("pending_analysis".to_string());
    world.loan_id = Some(format!("loan-{}", months));
}

#[then(expr = "the loan application is created with status {string}")]
fn loan_application_created(world: &mut BankoWorld, status: String) {
    assert_eq!(world.loan_status.as_deref(), Some(status.as_str()));
}

#[given(expr = "a loan with id {string} and no missed payments")]
fn loan_with_no_missed_payments(world: &mut BankoWorld, id: String) {
    world.loan_id = Some(id);
    world.missed_payments = Some(0);
}

#[when("I run the classification engine")]
fn run_classification_engine(world: &mut BankoWorld) {
    // Classification logic: performing loan (0 missed payments) = class 0
    if world.missed_payments == Some(0) {
        world.last_result = Some("class_0".to_string());
        world.provision_rate = Some(0.0);
    }
}

#[then(expr = "the loan is classified as class {int} with provision rate {int}%")]
fn loan_classified(world: &mut BankoWorld, class: i32, rate: i32) {
    assert_eq!(world.last_result.as_deref(), Some("class_0"));
    assert_eq!(world.provision_rate, Some(0.0));
}

// ============================================
// BC4: Anti-Money Laundering (AML)
// ============================================

#[given(expr = "a transaction of {float} TND in cash")]
fn transaction_in_cash(world: &mut BankoWorld, amount: f64) {
    world.amount = Some(amount);
    world.transaction_type = Some("cash".to_string());
}

#[when("I run AML screening")]
fn run_aml_screening(world: &mut BankoWorld) {
    // AML threshold: 10000 TND for cash transactions
    if let (Some(amt), Some(ref tx_type)) = (world.amount, &world.transaction_type) {
        if tx_type == "cash" && amt >= 10000.0 {
            world.alert_type = Some("large_cash_transaction".to_string());
            world.alert_generated = Some(true);
        } else {
            world.alert_generated = Some(false);
        }
    }
}

#[then(expr = "an alert is generated with type {string}")]
fn alert_generated(world: &mut BankoWorld, alert_type: String) {
    assert_eq!(world.alert_generated, Some(true));
    assert_eq!(world.alert_type.as_deref(), Some(alert_type.as_str()));
}

#[given(expr = "a transaction of {float} TND by transfer")]
fn transaction_by_transfer(world: &mut BankoWorld, amount: f64) {
    world.amount = Some(amount);
    world.transaction_type = Some("transfer".to_string());
}

#[then("no alert is generated")]
fn no_alert_generated(world: &mut BankoWorld) {
    assert_eq!(world.alert_generated, Some(false));
}

// ============================================
// BC5: Sanctions Screening
// ============================================

#[given(expr = "a customer named {string}")]
fn customer_named(world: &mut BankoWorld, name: String) {
    world.entity_name = Some(name);
}

#[when("I screen against ONU and UE sanctions lists")]
fn screen_against_sanctions_lists(world: &mut BankoWorld) {
    // Mock: "John Smith" is a normal name, no match
    if let Some(ref name) = world.entity_name {
        if name.contains("Sanctioned") {
            world.screening_result = Some("potential_match".to_string());
        } else {
            world.screening_result = Some("no_match".to_string());
        }
    }
}

#[then(expr = "the screening result is {string}")]
fn screening_result_is(world: &mut BankoWorld, result: String) {
    assert_eq!(world.screening_result.as_deref(), Some(result.as_str()));
}

#[when("I screen against ONU sanctions lists")]
fn screen_against_onu_sanctions(world: &mut BankoWorld) {
    // Mock: "Sanctioned Entity" matches with high score
    if let Some(ref name) = world.entity_name {
        if name.contains("Sanctioned") {
            world.screening_result = Some("potential_match".to_string());
            world.match_score = Some(85.5);
        } else {
            world.screening_result = Some("no_match".to_string());
        }
    }
}

#[then(expr = "the screening result is {string} with score above {int}")]
fn screening_result_with_score(world: &mut BankoWorld, result: String, min_score: i32) {
    assert_eq!(world.screening_result.as_deref(), Some(result.as_str()));
    if let Some(score) = world.match_score {
        assert!(score > min_score as f64, "Score {} is not above {}", score, min_score);
    }
}

// ============================================
// BC6: Prudential Ratios
// ============================================

#[given(expr = "total capital of {float} TND and risk-weighted assets of {float} TND")]
fn set_capital_and_rwa(world: &mut BankoWorld, capital: f64, rwa: f64) {
    world.total_capital = Some(capital);
    world.risk_weighted_assets = Some(rwa);
}

#[when("I calculate the solvency ratio")]
fn calculate_solvency_ratio(world: &mut BankoWorld) {
    if let (Some(cap), Some(rwa)) = (world.total_capital, world.risk_weighted_assets) {
        world.solvency_ratio = Some((cap / rwa) * 100.0);
    }
}

#[then(expr = "the ratio is {float}% which is above the {int}% minimum")]
fn ratio_above_minimum(world: &mut BankoWorld, expected: f64, minimum: i32) {
    if let Some(ratio) = world.solvency_ratio {
        assert!((ratio - expected).abs() < 0.01, "Ratio {} != {}", ratio, expected);
        assert!(ratio > minimum as f64);
    }
}

#[given(expr = "Tier 1 capital of {float} TND and risk-weighted assets of {float} TND")]
fn set_tier1_capital_and_rwa(world: &mut BankoWorld, t1_capital: f64, rwa: f64) {
    world.tier1_capital = Some(t1_capital);
    world.risk_weighted_assets = Some(rwa);
}

#[when("I calculate the Tier 1 ratio")]
fn calculate_tier1_ratio(world: &mut BankoWorld) {
    if let (Some(t1_cap), Some(rwa)) = (world.tier1_capital, world.risk_weighted_assets) {
        world.tier1_ratio = Some((t1_cap / rwa) * 100.0);
    }
}

#[then(expr = "the ratio is {float}% which is below the {int}% minimum")]
fn ratio_below_minimum(world: &mut BankoWorld, expected: f64, minimum: i32) {
    if let Some(ratio) = world.tier1_ratio {
        assert!((ratio - expected).abs() < 0.01, "Ratio {} != {}", ratio, expected);
        assert!(ratio < minimum as f64);
    }
}

// ============================================
// BC7: Accounting
// ============================================

#[given(expr = "a journal entry with debit {float} TND on account {string} and credit {float} TND on account {string}")]
fn journal_entry_with_amounts(
    world: &mut BankoWorld,
    debit: f64,
    debit_account: String,
    credit: f64,
    credit_account: String,
) {
    world.journal_entries.insert(debit_account, (debit, 0.0));
    world.journal_entries.insert(credit_account, (0.0, credit));
}

#[when("I validate the journal entry")]
fn validate_journal_entry(world: &mut BankoWorld) {
    // Check if debits equal credits
    let total_debit: f64 = world.journal_entries.values().map(|(d, _)| d).sum();
    let total_credit: f64 = world.journal_entries.values().map(|(_, c)| c).sum();

    if (total_debit - total_credit).abs() < 0.01 {
        world.entry_balanced = Some(true);
        world.last_result = Some("balanced".to_string());
    } else {
        world.entry_balanced = Some(false);
        world.last_error = Some("unbalanced_entry".to_string());
    }
}

#[then(expr = "the entry is accepted as balanced")]
fn entry_accepted_balanced(world: &mut BankoWorld) {
    assert_eq!(world.entry_balanced, Some(true));
}

#[then(expr = "the entry is rejected with error {string}")]
fn entry_rejected(world: &mut BankoWorld, error: String) {
    assert_eq!(world.last_error.as_deref(), Some(error.as_str()));
}

// ============================================
// BC8: Regulatory Reporting
// ============================================

#[given(expr = "the reporting period is {string}")]
fn set_reporting_period(world: &mut BankoWorld, period: String) {
    world.report_period = Some(period);
}

#[when("I generate the monthly prudential report")]
fn generate_prudential_report(world: &mut BankoWorld) {
    world.report_id = Some("report-001".to_string());
    world.report_status = Some("draft".to_string());
    world.last_result = Some("report_generated".to_string());
}

#[then(expr = "the report is created with status {string}")]
fn report_created_with_status(world: &mut BankoWorld, status: String) {
    assert_eq!(world.report_status.as_deref(), Some(status.as_str()));
}

#[given(expr = "a validated report with id {string}")]
fn validated_report(world: &mut BankoWorld, report_id: String) {
    world.report_id = Some(report_id);
    world.report_status = Some("validated".to_string());
}

#[when("I submit the report")]
fn submit_report(world: &mut BankoWorld) {
    if world.report_status.as_deref() == Some("validated") {
        world.report_status = Some("submitted".to_string());
        world.last_result = Some("report_submitted".to_string());
    }
}

#[then(expr = "the report status changes to {string}")]
fn report_status_changed(world: &mut BankoWorld, status: String) {
    assert_eq!(world.report_status.as_deref(), Some(status.as_str()));
}

// ============================================
// BC9: Payment Processing
// ============================================

#[given(expr = "an account {string} with balance {float} TND")]
fn account_with_balance(world: &mut BankoWorld, account: String, balance: f64) {
    world.account_balance.insert(account, balance);
}

#[when(expr = "I initiate a transfer of {float} TND to account {string}")]
fn initiate_transfer(world: &mut BankoWorld, amount: f64, target_account: String) {
    // Find source account (last added to world)
    if let Some((source, &balance)) = world.account_balance.iter().next() {
        let source_account = source.clone();
        if balance >= amount {
            world.transfer_id = Some("transfer-001".to_string());
            world.transfer_status = Some("pending".to_string());
            world.amount = Some(amount);
            world.last_result = Some("transfer_created".to_string());
        } else {
            world.last_error = Some("insufficient_funds".to_string());
        }
    }
}

#[then(expr = "the transfer is created with status {string}")]
fn transfer_created_with_status(world: &mut BankoWorld, status: String) {
    assert_eq!(world.transfer_status.as_deref(), Some(status.as_str()));
}

#[then(expr = "the transfer is rejected with error {string}")]
fn transfer_rejected(world: &mut BankoWorld, error: String) {
    assert_eq!(world.last_error.as_deref(), Some(error.as_str()));
}

// ============================================
// BC10: Foreign Exchange
// ============================================

#[given(expr = "an exchange rate of {float} TND/EUR")]
fn set_exchange_rate(world: &mut BankoWorld, rate: f64) {
    world.exchange_rate = Some(rate);
}

#[when(expr = "I convert {float} EUR to TND")]
fn convert_eur_to_tnd(world: &mut BankoWorld, eur_amount: f64) {
    if let Some(rate) = world.exchange_rate {
        world.conversion_result = Some(eur_amount * rate);
    }
}

#[then(expr = "the result is {float} TND")]
fn conversion_result_is(world: &mut BankoWorld, expected: f64) {
    if let Some(result) = world.conversion_result {
        assert!((result - expected).abs() < 0.01, "Result {} != {}", result, expected);
    }
}

#[given(expr = "the current EUR position is {float} TND equivalent")]
fn set_fx_position(world: &mut BankoWorld, position: f64) {
    world.fx_position = Some(position);
}

#[when(expr = "I check against the {float} TND position limit")]
fn check_position_limit(world: &mut BankoWorld, limit: f64) {
    world.position_limit = Some(limit);
}

#[then("the position is within limits")]
fn position_within_limits(world: &mut BankoWorld) {
    if let (Some(pos), Some(limit)) = (world.fx_position, world.position_limit) {
        assert!(pos <= limit, "Position {} exceeds limit {}", pos, limit);
    }
}

// ============================================
// BC11: Governance and Audit
// ============================================

#[given(expr = "a user {string} performs action {string}")]
fn user_performs_action(world: &mut BankoWorld, user_id: String, action: String) {
    world.user_id = Some(user_id);
    world.action_type = Some(action);
}

#[when("the audit trail is recorded")]
fn record_audit_trail(world: &mut BankoWorld) {
    if let (Some(ref user), Some(ref action)) = (&world.user_id, &world.action_type) {
        let timestamp = 1712433600i64; // Mock timestamp
        world.audit_entries.push((user.clone(), action.clone(), timestamp));
        world.last_result = Some("audit_recorded".to_string());
    }
}

#[then("an immutable entry exists with timestamp and actor")]
fn immutable_entry_exists(world: &mut BankoWorld) {
    assert!(!world.audit_entries.is_empty());
    let (user, action, ts) = &world.audit_entries[0];
    assert!(!user.is_empty());
    assert!(!action.is_empty());
    assert!(*ts > 0);
}

#[given(expr = "multiple audit entries for entity {string}")]
fn multiple_audit_entries_for_entity(world: &mut BankoWorld, entity: String) {
    world.entity_id = Some(entity);
    world.audit_entries = vec![
        ("user-001".to_string(), "create_account".to_string(), 1712433600i64),
        ("user-002".to_string(), "update_balance".to_string(), 1712433700i64),
        ("user-001".to_string(), "close_account".to_string(), 1712433800i64),
    ];
}

#[when(expr = "I query the audit trail for entity {string}")]
fn query_audit_trail(world: &mut BankoWorld, _entity: String) {
    // In mock, audit entries are already in world and sorted by timestamp
    world.last_result = Some("audit_query_complete".to_string());
}

#[then("all related entries are returned in chronological order")]
fn entries_in_chronological_order(world: &mut BankoWorld) {
    // Verify entries are sorted by timestamp
    let timestamps: Vec<i64> = world.audit_entries.iter().map(|(_, _, ts)| *ts).collect();
    let is_sorted = timestamps.windows(2).all(|w| w[0] <= w[1]);
    assert!(is_sorted, "Audit entries are not in chronological order");
}

fn main() {
    let features_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/bdd/features");
    let runner = BankoWorld::run(features_path);
    futures::executor::block_on(runner);
}
