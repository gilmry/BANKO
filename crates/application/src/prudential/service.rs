use std::sync::Arc;

use uuid::Uuid;

use banko_domain::prudential::*;

use super::dto::*;
use super::errors::PrudentialServiceError;
use super::ports::{IBreachAlertRepository, IPrudentialRepository};

pub struct RatioCalculationService {
    ratio_repo: Arc<dyn IPrudentialRepository>,
    alert_repo: Arc<dyn IBreachAlertRepository>,
}

impl RatioCalculationService {
    pub fn new(
        ratio_repo: Arc<dyn IPrudentialRepository>,
        alert_repo: Arc<dyn IBreachAlertRepository>,
    ) -> Self {
        RatioCalculationService {
            ratio_repo,
            alert_repo,
        }
    }

    pub async fn calculate_and_save(
        &self,
        request: CalculateRatiosRequest,
    ) -> Result<PrudentialRatioResponse, PrudentialServiceError> {
        let institution_id = Uuid::parse_str(&request.institution_id).map_err(|e| {
            PrudentialServiceError::InvalidInput(format!("Invalid institution_id: {e}"))
        })?;

        let exposures: Vec<Exposure> = request
            .exposures
            .unwrap_or_default()
            .into_iter()
            .map(|e| {
                let ben_id = Uuid::parse_str(&e.beneficiary_id).map_err(|err| {
                    PrudentialServiceError::InvalidInput(format!("Invalid beneficiary_id: {err}"))
                })?;
                Ok(Exposure::new(ben_id, e.amount, e.description))
            })
            .collect::<Result<Vec<_>, PrudentialServiceError>>()?;

        let ratio = PrudentialRatio::new(
            InstitutionId::from_uuid(institution_id),
            request.capital_tier1,
            request.capital_tier2,
            request.risk_weighted_assets,
            request.total_credits,
            request.total_deposits,
            exposures,
        )
        .map_err(|e| PrudentialServiceError::DomainError(e.to_string()))?;

        // Save ratio
        self.ratio_repo
            .save(&ratio)
            .await
            .map_err(PrudentialServiceError::Internal)?;

        // Save exposures
        for exp in ratio.exposures() {
            self.ratio_repo
                .save_exposure(ratio.ratio_id(), exp)
                .await
                .map_err(PrudentialServiceError::Internal)?;
        }

        // Check for breaches and save alerts
        let alerts = ratio.generate_alerts();
        for alert in &alerts {
            self.alert_repo
                .save(alert)
                .await
                .map_err(PrudentialServiceError::Internal)?;
        }

        let breaches: Vec<String> = ratio
            .check_all_ratios()
            .iter()
            .map(|b| b.as_str().to_string())
            .collect();

        Ok(PrudentialRatioResponse {
            id: ratio.ratio_id().to_string(),
            institution_id: ratio.institution_id().to_string(),
            solvency_ratio: ratio.solvency_ratio(),
            tier1_ratio: ratio.tier1_ratio(),
            credit_deposit_ratio: ratio.credit_deposit_ratio(),
            capital_tier1: ratio.capital_tier1(),
            capital_tier2: ratio.capital_tier2(),
            fonds_propres_nets: ratio.fonds_propres_nets(),
            risk_weighted_assets: ratio.risk_weighted_assets(),
            total_credits: ratio.total_credits(),
            total_deposits: ratio.total_deposits(),
            breaches,
            calculated_at: ratio.calculated_at(),
        })
    }

    pub async fn get_current_ratios(
        &self,
        institution_id: Uuid,
    ) -> Result<PrudentialRatioResponse, PrudentialServiceError> {
        let ratio = self
            .ratio_repo
            .find_latest(institution_id)
            .await
            .map_err(PrudentialServiceError::Internal)?
            .ok_or(PrudentialServiceError::RatioNotFound)?;

        let breaches: Vec<String> = ratio
            .check_all_ratios()
            .iter()
            .map(|b| b.as_str().to_string())
            .collect();

        Ok(PrudentialRatioResponse {
            id: ratio.ratio_id().to_string(),
            institution_id: ratio.institution_id().to_string(),
            solvency_ratio: ratio.solvency_ratio(),
            tier1_ratio: ratio.tier1_ratio(),
            credit_deposit_ratio: ratio.credit_deposit_ratio(),
            capital_tier1: ratio.capital_tier1(),
            capital_tier2: ratio.capital_tier2(),
            fonds_propres_nets: ratio.fonds_propres_nets(),
            risk_weighted_assets: ratio.risk_weighted_assets(),
            total_credits: ratio.total_credits(),
            total_deposits: ratio.total_deposits(),
            breaches,
            calculated_at: ratio.calculated_at(),
        })
    }

    pub async fn check_solvency(
        &self,
        institution_id: Uuid,
    ) -> Result<SolvencyCheckResponse, PrudentialServiceError> {
        let ratio = self
            .ratio_repo
            .find_latest(institution_id)
            .await
            .map_err(PrudentialServiceError::Internal)?
            .ok_or(PrudentialServiceError::RatioNotFound)?;

        let solvency = ratio.solvency_ratio();
        let compliant = solvency >= SOLVENCY_MINIMUM;
        Ok(SolvencyCheckResponse {
            ratio: solvency,
            minimum: SOLVENCY_MINIMUM,
            compliant,
            status: if compliant {
                "Clear".into()
            } else {
                "Breach".into()
            },
        })
    }

    pub async fn check_tier1(
        &self,
        institution_id: Uuid,
    ) -> Result<Tier1CheckResponse, PrudentialServiceError> {
        let ratio = self
            .ratio_repo
            .find_latest(institution_id)
            .await
            .map_err(PrudentialServiceError::Internal)?
            .ok_or(PrudentialServiceError::RatioNotFound)?;

        let tier1 = ratio.tier1_ratio();
        let compliant = tier1 >= TIER1_MINIMUM;
        Ok(Tier1CheckResponse {
            ratio: tier1,
            minimum: TIER1_MINIMUM,
            compliant,
            status: if compliant {
                "Clear".into()
            } else {
                "Breach".into()
            },
        })
    }

    pub async fn check_credit_deposit(
        &self,
        institution_id: Uuid,
    ) -> Result<CreditDepositCheckResponse, PrudentialServiceError> {
        let ratio = self
            .ratio_repo
            .find_latest(institution_id)
            .await
            .map_err(PrudentialServiceError::Internal)?
            .ok_or(PrudentialServiceError::RatioNotFound)?;

        let cd = ratio.credit_deposit_ratio();
        let compliant = cd <= CREDIT_DEPOSIT_MAXIMUM;
        Ok(CreditDepositCheckResponse {
            ratio: cd,
            maximum: CREDIT_DEPOSIT_MAXIMUM,
            compliant,
            status: if compliant {
                "Clear".into()
            } else {
                "Breach".into()
            },
        })
    }

    pub async fn check_concentration(
        &self,
        institution_id: Uuid,
        beneficiary_id: Uuid,
    ) -> Result<ConcentrationCheckResponse, PrudentialServiceError> {
        let ratio = self
            .ratio_repo
            .find_latest(institution_id)
            .await
            .map_err(PrudentialServiceError::Internal)?
            .ok_or(PrudentialServiceError::RatioNotFound)?;

        match ratio.check_concentration(beneficiary_id) {
            Ok(concentration) => Ok(ConcentrationCheckResponse {
                beneficiary_id: beneficiary_id.to_string(),
                ratio: concentration,
                maximum: CONCENTRATION_MAXIMUM,
                compliant: true,
                status: "Clear".into(),
            }),
            Err(_) => {
                let fpn = ratio.fonds_propres_nets();
                let total_exposure: i64 = ratio
                    .exposures()
                    .iter()
                    .filter(|e| e.beneficiary_id() == beneficiary_id)
                    .map(|e| e.amount())
                    .sum();
                let concentration = if fpn > 0 {
                    (total_exposure as f64 / fpn as f64) * 100.0
                } else {
                    0.0
                };
                Ok(ConcentrationCheckResponse {
                    beneficiary_id: beneficiary_id.to_string(),
                    ratio: concentration,
                    maximum: CONCENTRATION_MAXIMUM,
                    compliant: false,
                    status: "Breach".into(),
                })
            }
        }
    }

    pub async fn get_breach_alerts(
        &self,
        institution_id: Option<Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<BreachAlertListResponse, PrudentialServiceError> {
        let alerts = self
            .alert_repo
            .find_all(institution_id, limit, offset)
            .await
            .map_err(PrudentialServiceError::Internal)?;

        let total = self
            .alert_repo
            .count_active(institution_id)
            .await
            .map_err(PrudentialServiceError::Internal)?;

        let data = alerts
            .iter()
            .map(|a| BreachAlertResponse {
                id: a.id().to_string(),
                breach_type: a.breach_type().as_str().to_string(),
                current_value: a.current_value(),
                threshold: a.threshold(),
                severity: a.severity().as_str().to_string(),
                status: a.status().as_str().to_string(),
                created_at: a.created_at(),
            })
            .collect();

        Ok(BreachAlertListResponse { data, total })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prudential::ports::*;
    use async_trait::async_trait;
    use chrono::NaiveDate;
    use std::sync::Mutex;

    struct MockPrudentialRepo {
        ratios: Mutex<Vec<PrudentialRatio>>,
    }

    impl MockPrudentialRepo {
        fn new() -> Self {
            MockPrudentialRepo {
                ratios: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IPrudentialRepository for MockPrudentialRepo {
        async fn save(&self, ratio: &PrudentialRatio) -> Result<(), String> {
            self.ratios.lock().unwrap().push(ratio.clone());
            Ok(())
        }
        async fn find_by_id(&self, id: &RatioId) -> Result<Option<PrudentialRatio>, String> {
            Ok(self
                .ratios
                .lock()
                .unwrap()
                .iter()
                .find(|r| r.ratio_id() == id)
                .cloned())
        }
        async fn find_by_institution(&self, id: Uuid) -> Result<Option<PrudentialRatio>, String> {
            Ok(self
                .ratios
                .lock()
                .unwrap()
                .iter()
                .find(|r| *r.institution_id().as_uuid() == id)
                .cloned())
        }
        async fn find_latest(&self, id: Uuid) -> Result<Option<PrudentialRatio>, String> {
            Ok(self
                .ratios
                .lock()
                .unwrap()
                .iter()
                .rev()
                .find(|r| *r.institution_id().as_uuid() == id)
                .cloned())
        }
        async fn save_snapshot(&self, _snapshot: &RatioSnapshot) -> Result<(), String> {
            Ok(())
        }
        async fn find_snapshots(
            &self,
            _id: Uuid,
            _from: NaiveDate,
            _to: NaiveDate,
        ) -> Result<Vec<RatioSnapshot>, String> {
            Ok(vec![])
        }
        async fn save_exposure(
            &self,
            _ratio_id: &RatioId,
            _exposure: &Exposure,
        ) -> Result<(), String> {
            Ok(())
        }
        async fn find_exposures(&self, _ratio_id: &RatioId) -> Result<Vec<Exposure>, String> {
            Ok(vec![])
        }
    }

    struct MockBreachAlertRepo {
        alerts: Mutex<Vec<BreachAlert>>,
    }

    impl MockBreachAlertRepo {
        fn new() -> Self {
            MockBreachAlertRepo {
                alerts: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl IBreachAlertRepository for MockBreachAlertRepo {
        async fn save(&self, alert: &BreachAlert) -> Result<(), String> {
            self.alerts.lock().unwrap().push(alert.clone());
            Ok(())
        }
        async fn find_active(&self, _id: Uuid) -> Result<Vec<BreachAlert>, String> {
            Ok(vec![])
        }
        async fn find_all(
            &self,
            _id: Option<Uuid>,
            _limit: i64,
            _offset: i64,
        ) -> Result<Vec<BreachAlert>, String> {
            Ok(self.alerts.lock().unwrap().clone())
        }
        async fn count_active(&self, _id: Option<Uuid>) -> Result<i64, String> {
            Ok(self.alerts.lock().unwrap().len() as i64)
        }
    }

    #[tokio::test]
    async fn test_calculate_and_save_valid() {
        let service = RatioCalculationService::new(
            Arc::new(MockPrudentialRepo::new()),
            Arc::new(MockBreachAlertRepo::new()),
        );

        let inst_id = Uuid::new_v4();
        let result = service
            .calculate_and_save(CalculateRatiosRequest {
                institution_id: inst_id.to_string(),
                capital_tier1: 150_000,
                capital_tier2: 50_000,
                risk_weighted_assets: 1_000_000,
                total_credits: 500_000,
                total_deposits: 800_000,
                exposures: None,
            })
            .await
            .unwrap();

        assert!(result.solvency_ratio >= 10.0);
        assert!(result.tier1_ratio >= 7.0);
        assert!(result.credit_deposit_ratio <= 120.0);
        assert!(result.breaches.is_empty());
    }

    #[tokio::test]
    async fn test_calculate_solvency_breach_rejected() {
        let service = RatioCalculationService::new(
            Arc::new(MockPrudentialRepo::new()),
            Arc::new(MockBreachAlertRepo::new()),
        );

        let result = service
            .calculate_and_save(CalculateRatiosRequest {
                institution_id: Uuid::new_v4().to_string(),
                capital_tier1: 50_000,
                capital_tier2: 20_000,
                risk_weighted_assets: 1_000_000,
                total_credits: 500_000,
                total_deposits: 800_000,
                exposures: None,
            })
            .await;

        assert!(matches!(
            result,
            Err(PrudentialServiceError::DomainError(_))
        ));
    }

    #[tokio::test]
    async fn test_get_current_ratios() {
        let repo = Arc::new(MockPrudentialRepo::new());
        let service =
            RatioCalculationService::new(repo.clone(), Arc::new(MockBreachAlertRepo::new()));

        let inst_id = Uuid::new_v4();
        service
            .calculate_and_save(CalculateRatiosRequest {
                institution_id: inst_id.to_string(),
                capital_tier1: 150_000,
                capital_tier2: 50_000,
                risk_weighted_assets: 1_000_000,
                total_credits: 500_000,
                total_deposits: 800_000,
                exposures: None,
            })
            .await
            .unwrap();

        let result = service.get_current_ratios(inst_id).await.unwrap();
        assert!(result.solvency_ratio >= 10.0);
    }

    #[tokio::test]
    async fn test_check_solvency_compliant() {
        let repo = Arc::new(MockPrudentialRepo::new());
        let service =
            RatioCalculationService::new(repo.clone(), Arc::new(MockBreachAlertRepo::new()));

        let inst_id = Uuid::new_v4();
        service
            .calculate_and_save(CalculateRatiosRequest {
                institution_id: inst_id.to_string(),
                capital_tier1: 150_000,
                capital_tier2: 50_000,
                risk_weighted_assets: 1_000_000,
                total_credits: 500_000,
                total_deposits: 800_000,
                exposures: None,
            })
            .await
            .unwrap();

        let check = service.check_solvency(inst_id).await.unwrap();
        assert!(check.compliant);
        assert_eq!(check.status, "Clear");
    }
}
