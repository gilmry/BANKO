use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

/// Quick transfer request - simplified for mobile
#[derive(Debug, Clone, serde::Deserialize)]
pub struct QuickTransferRequest {
    pub from_account_id: Uuid,
    pub to_iban_or_phone: String,
    pub amount: Decimal,
    pub currency: String,
    pub note: Option<String>,
}

/// Quick transfer response
#[derive(Debug, Clone, serde::Serialize)]
pub struct QuickTransferResponse {
    pub transfer_id: Uuid,
    pub status: String,
    pub requires_2fa: bool,
}

/// Beneficiary for frequent transfers
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Beneficiary {
    pub id: String,
    pub name: String,
    pub iban: Option<String>,
    pub phone: Option<String>,
    pub transfer_count: u32,
}

/// QR code payment info
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QrPaymentInfo {
    pub beneficiary_name: String,
    pub iban: String,
    pub amount: Decimal,
    pub currency: String,
    pub reference: String,
}

/// Errors for mobile payment service
#[derive(Debug, thiserror::Error)]
pub enum MobilePaymentError {
    #[error("Account not found")]
    AccountNotFound,

    #[error("Invalid IBAN or phone number")]
    InvalidBeneficiary,

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Invalid QR data")]
    InvalidQrData,

    #[error("Invalid amount: {0}")]
    InvalidAmount(String),

    #[error("Transfer failed: {0}")]
    TransferFailed(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Port for mobile payment operations
#[async_trait]
pub trait IMobilePaymentProvider: Send + Sync {
    async fn get_account_balance(&self, account_id: &Uuid) -> Result<Decimal, String>;
    async fn create_transfer(
        &self,
        from_account_id: Uuid,
        beneficiary_iban: &str,
        amount: Decimal,
        currency: &str,
        reference: &str,
    ) -> Result<Uuid, String>;
    async fn get_frequent_beneficiaries(
        &self,
        customer_id: &Uuid,
        limit: usize,
    ) -> Result<Vec<Beneficiary>, String>;
    async fn record_frequent_beneficiary(
        &self,
        customer_id: &Uuid,
        name: &str,
        iban: Option<&str>,
        phone: Option<&str>,
    ) -> Result<(), String>;
}

/// Mobile Payment Service
pub struct MobilePaymentService {
    payment_provider: Arc<dyn IMobilePaymentProvider>,
}

impl MobilePaymentService {
    pub fn new(payment_provider: Arc<dyn IMobilePaymentProvider>) -> Self {
        MobilePaymentService { payment_provider }
    }

    /// Validate IBAN format: 2 uppercase letters + 2 digits + alphanumeric
    fn is_valid_iban(iban: &str) -> bool {
        if iban.len() < 15 || iban.len() > 34 {
            return false;
        }
        let bytes = iban.as_bytes();
        // First 2 chars must be letters
        if !bytes[0].is_ascii_uppercase() || !bytes[1].is_ascii_uppercase() {
            return false;
        }
        // Next 2 chars must be digits
        if !bytes[2].is_ascii_digit() || !bytes[3].is_ascii_digit() {
            return false;
        }
        // Rest must be alphanumeric
        bytes[4..].iter().all(|b| b.is_ascii_alphanumeric())
    }

    /// Validate phone format: +216 or +X or starts with digit, 8-15 digits
    fn is_valid_phone(phone: &str) -> bool {
        if phone.len() < 8 || phone.len() > 17 {
            return false;
        }
        if phone.starts_with('+') {
            phone[1..].chars().all(|c| c.is_ascii_digit())
        } else {
            phone.chars().all(|c| c.is_ascii_digit())
        }
    }

    /// Simplified quick transfer for mobile
    pub async fn quick_transfer(
        &self,
        customer_id: Uuid,
        req: QuickTransferRequest,
    ) -> Result<QuickTransferResponse, MobilePaymentError> {
        // Validate amount
        if req.amount <= Decimal::ZERO {
            return Err(MobilePaymentError::InvalidAmount("Amount must be positive".to_string()));
        }

        // Check balance
        let balance: Decimal = self
            .payment_provider
            .get_account_balance(&req.from_account_id)
            .await
            .map_err(|e| MobilePaymentError::Internal(e))?;

        if balance < req.amount {
            return Err(MobilePaymentError::InsufficientBalance);
        }

        // Auto-detect type: IBAN or phone
        let (is_iban, beneficiary_id) = if Self::is_valid_iban(&req.to_iban_or_phone) {
            (true, req.to_iban_or_phone.clone())
        } else if Self::is_valid_phone(&req.to_iban_or_phone) {
            // Phone number - future: mobile money integration
            (false, req.to_iban_or_phone.clone())
        } else {
            return Err(MobilePaymentError::InvalidBeneficiary);
        };

        if !is_iban {
            // For now, only support IBAN transfers
            return Err(MobilePaymentError::InvalidBeneficiary);
        }

        // Create transfer
        let transfer_id = self
            .payment_provider
            .create_transfer(
                req.from_account_id,
                &beneficiary_id,
                req.amount,
                &req.currency,
                &req.note.unwrap_or_default(),
            )
            .await
            .map_err(|e| MobilePaymentError::TransferFailed(e))?;

        // Record beneficiary for quick access
        self.payment_provider
            .record_frequent_beneficiary(&customer_id, &beneficiary_id, Some(&beneficiary_id), None)
            .await
            .ok(); // Don't fail if recording fails

        // Check if 2FA required (amount > 1000 TND)
        let requires_2fa = req.amount > Decimal::new(1000, 0);

        Ok(QuickTransferResponse {
            transfer_id,
            status: "pending".to_string(),
            requires_2fa,
        })
    }

    /// Get top 5 frequent beneficiaries
    pub async fn get_frequent_beneficiaries(
        &self,
        customer_id: Uuid,
    ) -> Result<Vec<Beneficiary>, MobilePaymentError> {
        self.payment_provider
            .get_frequent_beneficiaries(&customer_id, 5)
            .await
            .map_err(|e| MobilePaymentError::Internal(e))
    }

    /// Scan and parse QR code for payment
    pub async fn scan_qr_payment(&self, qr_data: String) -> Result<QrPaymentInfo, MobilePaymentError> {
        // Simple QR format: beneficiary|iban|amount|currency|reference
        let parts: Vec<&str> = qr_data.split('|').collect();

        if parts.len() < 5 {
            return Err(MobilePaymentError::InvalidQrData);
        }

        let beneficiary_name = parts[0].to_string();
        let iban = parts[1].to_string();
        let amount_str = parts[2];
        let currency = parts[3].to_string();
        let reference = parts[4].to_string();

        // Validate IBAN
        if !Self::is_valid_iban(&iban) {
            return Err(MobilePaymentError::InvalidBeneficiary);
        }

        // Parse amount
        let amount = amount_str
            .parse::<Decimal>()
            .map_err(|_| MobilePaymentError::InvalidAmount("Invalid amount in QR".to_string()))?;

        if amount <= Decimal::ZERO {
            return Err(MobilePaymentError::InvalidAmount("Amount must be positive".to_string()));
        }

        Ok(QrPaymentInfo {
            beneficiary_name,
            iban,
            amount,
            currency,
            reference,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPaymentProvider {
        balance: Decimal,
    }

    #[async_trait]
    impl IMobilePaymentProvider for MockPaymentProvider {
        async fn get_account_balance(&self, _account_id: &Uuid) -> Result<Decimal, String> {
            Ok(self.balance)
        }

        async fn create_transfer(
            &self,
            _from_account_id: Uuid,
            _beneficiary_iban: &str,
            _amount: Decimal,
            _currency: &str,
            _reference: &str,
        ) -> Result<Uuid, String> {
            Ok(Uuid::new_v4())
        }

        async fn get_frequent_beneficiaries(
            &self,
            _customer_id: &Uuid,
            _limit: usize,
        ) -> Result<Vec<Beneficiary>, String> {
            Ok(vec![
                Beneficiary {
                    id: "1".to_string(),
                    name: "Ahmed".to_string(),
                    iban: Some("TN5910005355869143604711".to_string()),
                    phone: None,
                    transfer_count: 10,
                },
                Beneficiary {
                    id: "2".to_string(),
                    name: "Fatma".to_string(),
                    iban: Some("TN5910005355869143604712".to_string()),
                    phone: None,
                    transfer_count: 5,
                },
            ])
        }

        async fn record_frequent_beneficiary(
            &self,
            _customer_id: &Uuid,
            _name: &str,
            _iban: Option<&str>,
            _phone: Option<&str>,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_quick_transfer_valid_iban() {
        let provider = Arc::new(MockPaymentProvider {
            balance: Decimal::new(100000, 2),
        });
        let service = MobilePaymentService::new(provider);

        let req = QuickTransferRequest {
            from_account_id: Uuid::new_v4(),
            to_iban_or_phone: "TN5910005355869143604711".to_string(),
            amount: Decimal::new(50000, 2),
            currency: "TND".to_string(),
            note: Some("Payment".to_string()),
        };

        let result = service.quick_transfer(Uuid::new_v4(), req).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status, "pending");
        assert!(!response.requires_2fa);
    }

    #[tokio::test]
    async fn test_quick_transfer_requires_2fa() {
        let provider = Arc::new(MockPaymentProvider {
            balance: Decimal::new(200000, 2),
        });
        let service = MobilePaymentService::new(provider);

        let req = QuickTransferRequest {
            from_account_id: Uuid::new_v4(),
            to_iban_or_phone: "TN5910005355869143604711".to_string(),
            amount: Decimal::new(150000, 2), // > 1000 TND
            currency: "TND".to_string(),
            note: None,
        };

        let result = service.quick_transfer(Uuid::new_v4(), req).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.requires_2fa);
    }

    #[tokio::test]
    async fn test_quick_transfer_insufficient_balance() {
        let provider = Arc::new(MockPaymentProvider {
            balance: Decimal::new(10000, 2),
        });
        let service = MobilePaymentService::new(provider);

        let req = QuickTransferRequest {
            from_account_id: Uuid::new_v4(),
            to_iban_or_phone: "TN5910005355869143604711".to_string(),
            amount: Decimal::new(50000, 2),
            currency: "TND".to_string(),
            note: None,
        };

        let result = service.quick_transfer(Uuid::new_v4(), req).await;
        assert!(matches!(result, Err(MobilePaymentError::InsufficientBalance)));
    }

    #[tokio::test]
    async fn test_quick_transfer_invalid_iban() {
        let provider = Arc::new(MockPaymentProvider {
            balance: Decimal::new(100000, 2),
        });
        let service = MobilePaymentService::new(provider);

        let req = QuickTransferRequest {
            from_account_id: Uuid::new_v4(),
            to_iban_or_phone: "INVALID123".to_string(),
            amount: Decimal::new(50000, 2),
            currency: "TND".to_string(),
            note: None,
        };

        let result = service.quick_transfer(Uuid::new_v4(), req).await;
        assert!(matches!(result, Err(MobilePaymentError::InvalidBeneficiary)));
    }

    #[tokio::test]
    async fn test_get_frequent_beneficiaries() {
        let provider = Arc::new(MockPaymentProvider {
            balance: Decimal::new(100000, 2),
        });
        let service = MobilePaymentService::new(provider);

        let result = service.get_frequent_beneficiaries(Uuid::new_v4()).await;
        assert!(result.is_ok());

        let beneficiaries = result.unwrap();
        assert_eq!(beneficiaries.len(), 2);
        assert_eq!(beneficiaries[0].name, "Ahmed");
    }

    #[tokio::test]
    async fn test_scan_qr_payment_valid() {
        let provider = Arc::new(MockPaymentProvider {
            balance: Decimal::new(100000, 2),
        });
        let service = MobilePaymentService::new(provider);

        let qr_data = "Ahmed|TN5910005355869143604711|500.50|TND|INV001".to_string();
        let result = service.scan_qr_payment(qr_data).await;

        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.beneficiary_name, "Ahmed");
        assert_eq!(info.iban, "TN5910005355869143604711");
        assert_eq!(info.amount, Decimal::new(50050, 2));
    }

    #[tokio::test]
    async fn test_scan_qr_payment_invalid_format() {
        let provider = Arc::new(MockPaymentProvider {
            balance: Decimal::new(100000, 2),
        });
        let service = MobilePaymentService::new(provider);

        let qr_data = "Ahmed|Invalid".to_string();
        let result = service.scan_qr_payment(qr_data).await;

        assert!(matches!(result, Err(MobilePaymentError::InvalidQrData)));
    }
}
