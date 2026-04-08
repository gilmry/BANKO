use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::errors::DomainError;
use crate::shared::value_objects::{CustomerId, Money};

use super::value_objects::{
    BankGuaranteeId, CollectionStatus, CollectionType, DocumentaryCollectionId, GuaranteeStatus,
    GuaranteeType, LCStatus, LCType, LetterOfCreditId, LimitType, TradeFinanceLimitId,
};

// --- LetterOfCredit aggregate root ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LetterOfCredit {
    id: LetterOfCreditId,
    lc_type: LCType,
    applicant_id: CustomerId,
    beneficiary_name: String,
    issuing_bank: String,
    advising_bank: String,
    amount: Money,
    issue_date: DateTime<Utc>,
    expiry_date: DateTime<Utc>,
    terms_description: String,
    documents_required: Vec<String>,
    status: LCStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl LetterOfCredit {
    /// Create a new letter of credit. Enforces domain invariants.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        lc_type: LCType,
        applicant_id: CustomerId,
        beneficiary_name: &str,
        issuing_bank: &str,
        advising_bank: &str,
        amount: Money,
        issue_date: DateTime<Utc>,
        expiry_date: DateTime<Utc>,
        terms_description: &str,
        documents_required: Vec<String>,
    ) -> Result<Self, DomainError> {
        // Invariant: amount must be positive
        if amount.is_negative() || amount.is_zero() {
            return Err(DomainError::InvalidMoney(
                "LC amount must be positive".to_string(),
            ));
        }

        // Invariant: expiry date must be after issue date
        if expiry_date <= issue_date {
            return Err(DomainError::ValidationError(
                "LC expiry date must be after issue date".to_string(),
            ));
        }

        // Invariant: documents_required must not be empty
        if documents_required.is_empty() {
            return Err(DomainError::ValidationError(
                "LC must have at least one required document".to_string(),
            ));
        }

        // Invariant: beneficiary name must not be empty
        if beneficiary_name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Beneficiary name cannot be empty".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(LetterOfCredit {
            id: LetterOfCreditId::new(),
            lc_type,
            applicant_id,
            beneficiary_name: beneficiary_name.to_string(),
            issuing_bank: issuing_bank.to_string(),
            advising_bank: advising_bank.to_string(),
            amount,
            issue_date,
            expiry_date,
            terms_description: terms_description.to_string(),
            documents_required,
            status: LCStatus::Draft,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: LetterOfCreditId,
        lc_type: LCType,
        applicant_id: CustomerId,
        beneficiary_name: String,
        issuing_bank: String,
        advising_bank: String,
        amount: Money,
        issue_date: DateTime<Utc>,
        expiry_date: DateTime<Utc>,
        terms_description: String,
        documents_required: Vec<String>,
        status: LCStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        LetterOfCredit {
            id,
            lc_type,
            applicant_id,
            beneficiary_name,
            issuing_bank,
            advising_bank,
            amount,
            issue_date,
            expiry_date,
            terms_description,
            documents_required,
            status,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &LetterOfCreditId {
        &self.id
    }

    pub fn lc_type(&self) -> LCType {
        self.lc_type
    }

    pub fn applicant_id(&self) -> &CustomerId {
        &self.applicant_id
    }

    pub fn beneficiary_name(&self) -> &str {
        &self.beneficiary_name
    }

    pub fn issuing_bank(&self) -> &str {
        &self.issuing_bank
    }

    pub fn advising_bank(&self) -> &str {
        &self.advising_bank
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }

    pub fn issue_date(&self) -> DateTime<Utc> {
        self.issue_date
    }

    pub fn expiry_date(&self) -> DateTime<Utc> {
        self.expiry_date
    }

    pub fn terms_description(&self) -> &str {
        &self.terms_description
    }

    pub fn documents_required(&self) -> &[String] {
        &self.documents_required
    }

    pub fn status(&self) -> LCStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Commands ---

    pub fn issue(&mut self) -> Result<(), DomainError> {
        if self.status != LCStatus::Draft {
            return Err(DomainError::ValidationError(
                "Can only issue a draft LC".to_string(),
            ));
        }

        self.status = LCStatus::Issued;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn amend(&mut self) -> Result<(), DomainError> {
        if self.status != LCStatus::Issued {
            return Err(DomainError::ValidationError(
                "Can only amend an issued LC".to_string(),
            ));
        }

        self.status = LCStatus::Amended;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn utilize(&mut self) -> Result<(), DomainError> {
        if self.status != LCStatus::Issued && self.status != LCStatus::Amended {
            return Err(DomainError::ValidationError(
                "Can only utilize an issued or amended LC".to_string(),
            ));
        }

        self.status = LCStatus::Utilized;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), DomainError> {
        if self.status == LCStatus::Cancelled || self.status == LCStatus::Expired {
            return Err(DomainError::ValidationError(
                "Cannot cancel an already closed LC".to_string(),
            ));
        }

        self.status = LCStatus::Cancelled;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expiry_date
    }
}

// --- BankGuarantee aggregate root ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankGuarantee {
    id: BankGuaranteeId,
    guarantee_type: GuaranteeType,
    principal_id: CustomerId,
    beneficiary_name: String,
    amount: Money,
    issue_date: DateTime<Utc>,
    expiry_date: DateTime<Utc>,
    claim_conditions: String,
    status: GuaranteeStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BankGuarantee {
    /// Create a new bank guarantee. Enforces domain invariants.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        guarantee_type: GuaranteeType,
        principal_id: CustomerId,
        beneficiary_name: &str,
        amount: Money,
        issue_date: DateTime<Utc>,
        expiry_date: DateTime<Utc>,
        claim_conditions: &str,
    ) -> Result<Self, DomainError> {
        // Invariant: amount must be positive
        if amount.is_negative() || amount.is_zero() {
            return Err(DomainError::InvalidMoney(
                "Guarantee amount must be positive".to_string(),
            ));
        }

        // Invariant: expiry date must be after issue date
        if expiry_date <= issue_date {
            return Err(DomainError::ValidationError(
                "Guarantee expiry date must be after issue date".to_string(),
            ));
        }

        // Invariant: claim conditions must not be empty
        if claim_conditions.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Claim conditions must be specified".to_string(),
            ));
        }

        // Invariant: beneficiary name must not be empty
        if beneficiary_name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Beneficiary name cannot be empty".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(BankGuarantee {
            id: BankGuaranteeId::new(),
            guarantee_type,
            principal_id,
            beneficiary_name: beneficiary_name.to_string(),
            amount,
            issue_date,
            expiry_date,
            claim_conditions: claim_conditions.to_string(),
            status: GuaranteeStatus::Active,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: BankGuaranteeId,
        guarantee_type: GuaranteeType,
        principal_id: CustomerId,
        beneficiary_name: String,
        amount: Money,
        issue_date: DateTime<Utc>,
        expiry_date: DateTime<Utc>,
        claim_conditions: String,
        status: GuaranteeStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        BankGuarantee {
            id,
            guarantee_type,
            principal_id,
            beneficiary_name,
            amount,
            issue_date,
            expiry_date,
            claim_conditions,
            status,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &BankGuaranteeId {
        &self.id
    }

    pub fn guarantee_type(&self) -> GuaranteeType {
        self.guarantee_type
    }

    pub fn principal_id(&self) -> &CustomerId {
        &self.principal_id
    }

    pub fn beneficiary_name(&self) -> &str {
        &self.beneficiary_name
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }

    pub fn issue_date(&self) -> DateTime<Utc> {
        self.issue_date
    }

    pub fn expiry_date(&self) -> DateTime<Utc> {
        self.expiry_date
    }

    pub fn claim_conditions(&self) -> &str {
        &self.claim_conditions
    }

    pub fn status(&self) -> GuaranteeStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Commands ---

    pub fn call(&mut self) -> Result<(), DomainError> {
        if self.status != GuaranteeStatus::Active {
            return Err(DomainError::ValidationError(
                "Can only call an active guarantee".to_string(),
            ));
        }

        self.status = GuaranteeStatus::Called;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn release(&mut self) -> Result<(), DomainError> {
        if self.status == GuaranteeStatus::Released || self.status == GuaranteeStatus::Expired {
            return Err(DomainError::ValidationError(
                "Cannot release an already closed guarantee".to_string(),
            ));
        }

        self.status = GuaranteeStatus::Released;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expiry_date
    }
}

// --- DocumentaryCollection aggregate root ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentaryCollection {
    id: DocumentaryCollectionId,
    collection_type: CollectionType,
    exporter_id: CustomerId,
    importer_name: String,
    amount: Money,
    documents: Vec<String>,
    status: CollectionStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl DocumentaryCollection {
    /// Create a new documentary collection. Enforces domain invariants.
    pub fn new(
        collection_type: CollectionType,
        exporter_id: CustomerId,
        importer_name: &str,
        amount: Money,
        documents: Vec<String>,
    ) -> Result<Self, DomainError> {
        // Invariant: amount must be positive
        if amount.is_negative() || amount.is_zero() {
            return Err(DomainError::InvalidMoney(
                "Collection amount must be positive".to_string(),
            ));
        }

        // Invariant: documents must not be empty
        if documents.is_empty() {
            return Err(DomainError::ValidationError(
                "Collection must have at least one document".to_string(),
            ));
        }

        // Invariant: importer name must not be empty
        if importer_name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Importer name cannot be empty".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(DocumentaryCollection {
            id: DocumentaryCollectionId::new(),
            collection_type,
            exporter_id,
            importer_name: importer_name.to_string(),
            amount,
            documents,
            status: CollectionStatus::Received,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: DocumentaryCollectionId,
        collection_type: CollectionType,
        exporter_id: CustomerId,
        importer_name: String,
        amount: Money,
        documents: Vec<String>,
        status: CollectionStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        DocumentaryCollection {
            id,
            collection_type,
            exporter_id,
            importer_name,
            amount,
            documents,
            status,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &DocumentaryCollectionId {
        &self.id
    }

    pub fn collection_type(&self) -> CollectionType {
        self.collection_type
    }

    pub fn exporter_id(&self) -> &CustomerId {
        &self.exporter_id
    }

    pub fn importer_name(&self) -> &str {
        &self.importer_name
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }

    pub fn documents(&self) -> &[String] {
        &self.documents
    }

    pub fn status(&self) -> CollectionStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Commands ---

    pub fn present(&mut self) -> Result<(), DomainError> {
        if self.status != CollectionStatus::Received {
            return Err(DomainError::ValidationError(
                "Can only present a received collection".to_string(),
            ));
        }

        self.status = CollectionStatus::Presented;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn accept(&mut self) -> Result<(), DomainError> {
        if self.status != CollectionStatus::Presented {
            return Err(DomainError::ValidationError(
                "Can only accept a presented collection".to_string(),
            ));
        }

        self.status = CollectionStatus::Accepted;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn pay(&mut self) -> Result<(), DomainError> {
        if self.status != CollectionStatus::Accepted {
            return Err(DomainError::ValidationError(
                "Can only pay an accepted collection".to_string(),
            ));
        }

        self.status = CollectionStatus::Paid;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn protest(&mut self) -> Result<(), DomainError> {
        if self.status != CollectionStatus::Presented {
            return Err(DomainError::ValidationError(
                "Can only protest a presented collection".to_string(),
            ));
        }

        self.status = CollectionStatus::Protested;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// --- TradeFinanceLimit aggregate root ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeFinanceLimit {
    id: TradeFinanceLimitId,
    customer_id: CustomerId,
    limit_type: LimitType,
    total_limit: Money,
    utilized: Money,
    available: Money,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TradeFinanceLimit {
    /// Create a new trade finance limit. Enforces domain invariants.
    pub fn new(
        customer_id: CustomerId,
        limit_type: LimitType,
        total_limit: Money,
    ) -> Result<Self, DomainError> {
        // Invariant: total_limit must be positive
        if total_limit.is_negative() || total_limit.is_zero() {
            return Err(DomainError::InvalidMoney(
                "Total limit must be positive".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(TradeFinanceLimit {
            id: TradeFinanceLimitId::new(),
            customer_id,
            limit_type,
            total_limit: total_limit.clone(),
            utilized: Money::zero(total_limit.currency()),
            available: total_limit,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: TradeFinanceLimitId,
        customer_id: CustomerId,
        limit_type: LimitType,
        total_limit: Money,
        utilized: Money,
        available: Money,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        TradeFinanceLimit {
            id,
            customer_id,
            limit_type,
            total_limit,
            utilized,
            available,
            created_at,
            updated_at,
        }
    }

    // --- Getters ---

    pub fn id(&self) -> &TradeFinanceLimitId {
        &self.id
    }

    pub fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }

    pub fn limit_type(&self) -> LimitType {
        self.limit_type
    }

    pub fn total_limit(&self) -> &Money {
        &self.total_limit
    }

    pub fn utilized(&self) -> &Money {
        &self.utilized
    }

    pub fn available(&self) -> &Money {
        &self.available
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // --- Commands ---

    pub fn utilize(&mut self, amount: &Money) -> Result<(), DomainError> {
        // Check that utilization does not exceed available
        if amount.amount_cents() > self.available.amount_cents() {
            return Err(DomainError::ValidationError(
                "Utilization exceeds available limit".to_string(),
            ));
        }

        self.utilized = self.utilized.add(amount)?;
        self.available = self.available.subtract(amount)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn release(&mut self, amount: &Money) -> Result<(), DomainError> {
        // Check that release does not exceed utilized
        if amount.amount_cents() > self.utilized.amount_cents() {
            return Err(DomainError::ValidationError(
                "Release amount exceeds utilized amount".to_string(),
            ));
        }

        self.utilized = self.utilized.subtract(amount)?;
        self.available = self.available.add(amount)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn increase_limit(&mut self, increase: &Money) -> Result<(), DomainError> {
        if increase.is_negative() || increase.is_zero() {
            return Err(DomainError::InvalidMoney(
                "Increase amount must be positive".to_string(),
            ));
        }

        self.total_limit = self.total_limit.add(increase)?;
        self.available = self.available.add(increase)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn decrease_limit(&mut self, decrease: &Money) -> Result<(), DomainError> {
        if decrease.is_negative() || decrease.is_zero() {
            return Err(DomainError::InvalidMoney(
                "Decrease amount must be positive".to_string(),
            ));
        }

        // Cannot decrease below utilized amount
        if decrease.amount_cents() > self.available.amount_cents() {
            return Err(DomainError::ValidationError(
                "Cannot decrease limit below utilized amount".to_string(),
            ));
        }

        self.total_limit = self.total_limit.subtract(decrease)?;
        self.available = self.available.subtract(decrease)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn utilization_rate(&self) -> Result<f64, DomainError> {
        if self.total_limit.is_zero() {
            return Ok(0.0);
        }

        Ok(self.utilized.amount() / self.total_limit.amount())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::value_objects::Currency;

    #[test]
    fn test_letter_of_credit_new() {
        let applicant = CustomerId::from_uuid(uuid::Uuid::new_v4());
        let now = Utc::now();
        let expiry = now + chrono::Duration::days(180);
        let amount = Money::from_cents(1000000, Currency::try_from("TND").unwrap());

        let lc = LetterOfCredit::new(
            LCType::Import,
            applicant,
            "John Doe",
            "Bank A",
            "Bank B",
            amount,
            now,
            expiry,
            "Standard LC terms",
            vec!["Invoice".to_string(), "Bill of Lading".to_string()],
        );

        assert!(lc.is_ok());
        let l = lc.unwrap();
        assert_eq!(l.lc_type(), LCType::Import);
        assert_eq!(l.status(), LCStatus::Draft);
    }

    #[test]
    fn test_letter_of_credit_invalid_expiry() {
        let applicant = CustomerId::from_uuid(uuid::Uuid::new_v4());
        let now = Utc::now();
        let amount = Money::from_cents(1000000, Currency::try_from("TND").unwrap());

        let lc = LetterOfCredit::new(
            LCType::Export,
            applicant,
            "Jane Doe",
            "Bank A",
            "Bank B",
            amount,
            now,
            now,
            "Terms",
            vec!["Invoice".to_string()],
        );

        assert!(lc.is_err());
    }

    #[test]
    fn test_letter_of_credit_zero_amount() {
        let applicant = CustomerId::from_uuid(uuid::Uuid::new_v4());
        let now = Utc::now();
        let expiry = now + chrono::Duration::days(180);
        let amount = Money::from_cents(0, Currency::try_from("TND").unwrap());

        let lc = LetterOfCredit::new(
            LCType::Standby,
            applicant,
            "Ben Smith",
            "Bank A",
            "Bank B",
            amount,
            now,
            expiry,
            "Terms",
            vec!["Invoice".to_string()],
        );

        assert!(lc.is_err());
    }

    #[test]
    fn test_bank_guarantee_new() {
        let principal = CustomerId::from_uuid(uuid::Uuid::new_v4());
        let now = Utc::now();
        let expiry = now + chrono::Duration::days(365);
        let amount = Money::from_cents(500000, Currency::try_from("TND").unwrap());

        let guarantee = BankGuarantee::new(
            GuaranteeType::Performance,
            principal,
            "ABC Corp",
            amount,
            now,
            expiry,
            "Payment on first demand",
        );

        assert!(guarantee.is_ok());
        let g = guarantee.unwrap();
        assert_eq!(g.guarantee_type(), GuaranteeType::Performance);
        assert_eq!(g.status(), GuaranteeStatus::Active);
    }

    #[test]
    fn test_bank_guarantee_call() {
        let principal = CustomerId::from_uuid(uuid::Uuid::new_v4());
        let now = Utc::now();
        let expiry = now + chrono::Duration::days(365);
        let amount = Money::from_cents(500000, Currency::try_from("TND").unwrap());

        let mut guarantee = BankGuarantee::new(
            GuaranteeType::Payment,
            principal,
            "XYZ Ltd",
            amount,
            now,
            expiry,
            "Conditions apply",
        )
        .unwrap();

        guarantee.call().unwrap();
        assert_eq!(guarantee.status(), GuaranteeStatus::Called);
    }

    #[test]
    fn test_documentary_collection_new() {
        let exporter = CustomerId::from_uuid(uuid::Uuid::new_v4());
        let amount = Money::from_cents(750000, Currency::try_from("TND").unwrap());

        let collection = DocumentaryCollection::new(
            CollectionType::DocumentsAgainstPayment,
            exporter,
            "Importer LLC",
            amount,
            vec!["Invoice".to_string(), "Packing List".to_string()],
        );

        assert!(collection.is_ok());
        let c = collection.unwrap();
        assert_eq!(c.collection_type(), CollectionType::DocumentsAgainstPayment);
        assert_eq!(c.status(), CollectionStatus::Received);
    }

    #[test]
    fn test_documentary_collection_workflow() {
        let exporter = CustomerId::from_uuid(uuid::Uuid::new_v4());
        let amount = Money::from_cents(750000, Currency::try_from("TND").unwrap());

        let mut collection = DocumentaryCollection::new(
            CollectionType::DocumentsAgainstAcceptance,
            exporter,
            "Buyer Co",
            amount,
            vec!["Invoice".to_string()],
        )
        .unwrap();

        collection.present().unwrap();
        assert_eq!(collection.status(), CollectionStatus::Presented);

        collection.accept().unwrap();
        assert_eq!(collection.status(), CollectionStatus::Accepted);

        collection.pay().unwrap();
        assert_eq!(collection.status(), CollectionStatus::Paid);
    }

    #[test]
    fn test_trade_finance_limit_new() {
        let customer = CustomerId::from_uuid(uuid::Uuid::new_v4());
        let limit = Money::from_cents(5000000, Currency::try_from("TND").unwrap());

        let tfl = TradeFinanceLimit::new(customer, LimitType::LC, limit);

        assert!(tfl.is_ok());
        let t = tfl.unwrap();
        assert_eq!(t.limit_type(), LimitType::LC);
        assert!(t.utilized().is_zero());
        assert_eq!(t.total_limit(), t.available());
    }

    #[test]
    fn test_trade_finance_limit_utilize() {
        let customer = CustomerId::from_uuid(uuid::Uuid::new_v4());
        let limit = Money::from_cents(5000000, Currency::try_from("TND").unwrap());
        let utilize = Money::from_cents(2000000, Currency::try_from("TND").unwrap());

        let mut tfl =
            TradeFinanceLimit::new(customer, LimitType::Guarantee, limit.clone()).unwrap();

        tfl.utilize(&utilize).unwrap();

        assert_eq!(tfl.utilized(), &utilize);
        let remaining = limit.subtract(&utilize).unwrap();
        assert_eq!(tfl.available(), &remaining);
    }

    #[test]
    fn test_trade_finance_limit_exceed_fails() {
        let customer = CustomerId::from_uuid(uuid::Uuid::new_v4());
        let limit = Money::from_cents(5000000, Currency::try_from("TND").unwrap());
        let exceed = Money::from_cents(6000000, Currency::try_from("TND").unwrap());

        let mut tfl = TradeFinanceLimit::new(customer, LimitType::Collection, limit).unwrap();

        let result = tfl.utilize(&exceed);
        assert!(result.is_err());
    }

    #[test]
    fn test_trade_finance_limit_utilization_rate() {
        let customer = CustomerId::from_uuid(uuid::Uuid::new_v4());
        let limit = Money::from_cents(10000000, Currency::try_from("TND").unwrap());
        let utilize = Money::from_cents(5000000, Currency::try_from("TND").unwrap());

        let mut tfl = TradeFinanceLimit::new(customer, LimitType::LC, limit).unwrap();
        tfl.utilize(&utilize).unwrap();

        let rate = tfl.utilization_rate().unwrap();
        assert!((rate - 0.5).abs() < 0.001);
    }
}
