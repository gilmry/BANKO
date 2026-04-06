use async_trait::async_trait;

use banko_domain::account::AccountId;
use banko_domain::credit::{AssetClass, Installment, Loan, LoanId, LoanStatus};

/// Port for loan persistence — implemented by infrastructure layer.
#[async_trait]
pub trait ILoanRepository: Send + Sync {
    async fn save(&self, loan: &Loan) -> Result<(), String>;
    async fn find_by_id(&self, id: &LoanId) -> Result<Option<Loan>, String>;
    async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Vec<Loan>, String>;
    async fn find_all(
        &self,
        status: Option<LoanStatus>,
        asset_class: Option<AssetClass>,
        account_id: Option<&AccountId>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Loan>, String>;
    async fn count_all(
        &self,
        status: Option<LoanStatus>,
        asset_class: Option<AssetClass>,
        account_id: Option<&AccountId>,
    ) -> Result<i64, String>;
    async fn find_active_loans(&self) -> Result<Vec<Loan>, String>;
    async fn delete(&self, id: &LoanId) -> Result<(), String>;
}

/// Port for loan schedule persistence — separate per ISP.
#[async_trait]
pub trait IScheduleRepository: Send + Sync {
    async fn save_installments(&self, installments: &[Installment]) -> Result<(), String>;
    async fn find_by_loan_id(&self, loan_id: &LoanId) -> Result<Vec<Installment>, String>;
    async fn update_installment(&self, installment: &Installment) -> Result<(), String>;
}
