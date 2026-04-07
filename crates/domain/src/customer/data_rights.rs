use std::fmt;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

// --- DataRequestId ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DataRequestId(Uuid);

impl Default for DataRequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl DataRequestId {
    pub fn new() -> Self {
        DataRequestId(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        DataRequestId(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl fmt::Display for DataRequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- DataRequestType ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataRequestType {
    Access,
    Rectification,
    Opposition,
}

impl DataRequestType {
    pub fn from_str_type(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "access" => Ok(DataRequestType::Access),
            "rectification" => Ok(DataRequestType::Rectification),
            "opposition" => Ok(DataRequestType::Opposition),
            _ => Err(DomainError::InvalidDataRequest(format!(
                "Unknown data request type: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DataRequestType::Access => "Access",
            DataRequestType::Rectification => "Rectification",
            DataRequestType::Opposition => "Opposition",
        }
    }
}

impl fmt::Display for DataRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- DataRequestStatus ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataRequestStatus {
    Pending,
    Processing,
    Completed,
    Rejected,
}

impl DataRequestStatus {
    pub fn from_str_status(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(DataRequestStatus::Pending),
            "processing" => Ok(DataRequestStatus::Processing),
            "completed" => Ok(DataRequestStatus::Completed),
            "rejected" => Ok(DataRequestStatus::Rejected),
            _ => Err(DomainError::InvalidDataRequest(format!(
                "Unknown data request status: {s}"
            ))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DataRequestStatus::Pending => "Pending",
            DataRequestStatus::Processing => "Processing",
            DataRequestStatus::Completed => "Completed",
            DataRequestStatus::Rejected => "Rejected",
        }
    }
}

impl fmt::Display for DataRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// --- DataRightsRequest ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRightsRequest {
    request_id: DataRequestId,
    customer_id: Uuid,
    request_type: DataRequestType,
    status: DataRequestStatus,
    details: Option<String>,
    response: Option<String>,
    requested_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    deadline: DateTime<Utc>,
}

impl DataRightsRequest {
    /// Create a new data rights request. Deadline is set to 30 days from now.
    pub fn new(customer_id: Uuid, request_type: DataRequestType, details: Option<String>) -> Self {
        let now = Utc::now();
        DataRightsRequest {
            request_id: DataRequestId::new(),
            customer_id,
            request_type,
            status: DataRequestStatus::Pending,
            details,
            response: None,
            requested_at: now,
            completed_at: None,
            deadline: now + Duration::days(30),
        }
    }

    /// Reconstitute from persistence (no validation).
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        request_id: DataRequestId,
        customer_id: Uuid,
        request_type: DataRequestType,
        status: DataRequestStatus,
        details: Option<String>,
        response: Option<String>,
        requested_at: DateTime<Utc>,
        completed_at: Option<DateTime<Utc>>,
        deadline: DateTime<Utc>,
    ) -> Self {
        DataRightsRequest {
            request_id,
            customer_id,
            request_type,
            status,
            details,
            response,
            requested_at,
            completed_at,
            deadline,
        }
    }

    /// Move to Processing status. Only Pending requests can be processed.
    pub fn process(&mut self) -> Result<(), DomainError> {
        if self.status != DataRequestStatus::Pending {
            return Err(DomainError::InvalidDataRequest(format!(
                "Cannot process request in status: {}",
                self.status
            )));
        }
        self.status = DataRequestStatus::Processing;
        Ok(())
    }

    /// Complete the request with a response. Only Processing requests can be completed.
    pub fn complete(&mut self, response: String) -> Result<(), DomainError> {
        if self.status != DataRequestStatus::Processing {
            return Err(DomainError::DataRequestAlreadyCompleted);
        }
        self.status = DataRequestStatus::Completed;
        self.response = Some(response);
        self.completed_at = Some(Utc::now());
        Ok(())
    }

    /// Reject the request with a reason.
    pub fn reject(&mut self, reason: String) -> Result<(), DomainError> {
        if self.status == DataRequestStatus::Completed || self.status == DataRequestStatus::Rejected
        {
            return Err(DomainError::DataRequestAlreadyCompleted);
        }
        self.status = DataRequestStatus::Rejected;
        self.response = Some(reason);
        self.completed_at = Some(Utc::now());
        Ok(())
    }

    /// Check if the request is overdue (deadline passed and not completed/rejected).
    pub fn is_overdue(&self) -> bool {
        Utc::now() > self.deadline
            && self.status != DataRequestStatus::Completed
            && self.status != DataRequestStatus::Rejected
    }

    // --- Getters ---

    pub fn request_id(&self) -> &DataRequestId {
        &self.request_id
    }

    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }

    pub fn request_type(&self) -> DataRequestType {
        self.request_type
    }

    pub fn status(&self) -> DataRequestStatus {
        self.status
    }

    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }

    pub fn response(&self) -> Option<&str> {
        self.response.as_deref()
    }

    pub fn requested_at(&self) -> DateTime<Utc> {
        self.requested_at
    }

    pub fn completed_at(&self) -> Option<DateTime<Utc>> {
        self.completed_at
    }

    pub fn deadline(&self) -> DateTime<Utc> {
        self.deadline
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_customer_id() -> Uuid {
        Uuid::new_v4()
    }

    #[test]
    fn test_data_rights_request_new() {
        let cid = sample_customer_id();
        let req = DataRightsRequest::new(cid, DataRequestType::Access, None);
        assert_eq!(req.customer_id(), cid);
        assert_eq!(req.request_type(), DataRequestType::Access);
        assert_eq!(req.status(), DataRequestStatus::Pending);
        assert!(req.details().is_none());
        assert!(req.response().is_none());
        assert!(req.completed_at().is_none());
        // Deadline should be ~30 days from now
        let diff = req.deadline() - req.requested_at();
        assert_eq!(diff.num_days(), 30);
    }

    #[test]
    fn test_data_rights_request_with_details() {
        let cid = sample_customer_id();
        let req = DataRightsRequest::new(
            cid,
            DataRequestType::Rectification,
            Some("Please update my address".to_string()),
        );
        assert_eq!(req.details(), Some("Please update my address"));
    }

    #[test]
    fn test_data_rights_request_lifecycle() {
        let cid = sample_customer_id();
        let mut req = DataRightsRequest::new(cid, DataRequestType::Access, None);

        // Pending -> Processing
        assert!(req.process().is_ok());
        assert_eq!(req.status(), DataRequestStatus::Processing);

        // Processing -> Completed
        assert!(req.complete("Here is your data export".to_string()).is_ok());
        assert_eq!(req.status(), DataRequestStatus::Completed);
        assert_eq!(req.response(), Some("Here is your data export"));
        assert!(req.completed_at().is_some());
    }

    #[test]
    fn test_data_rights_request_reject() {
        let cid = sample_customer_id();
        let mut req = DataRightsRequest::new(cid, DataRequestType::Opposition, None);

        assert!(req.reject("Invalid request".to_string()).is_ok());
        assert_eq!(req.status(), DataRequestStatus::Rejected);
        assert_eq!(req.response(), Some("Invalid request"));
    }

    #[test]
    fn test_cannot_process_non_pending() {
        let cid = sample_customer_id();
        let mut req = DataRightsRequest::new(cid, DataRequestType::Access, None);
        req.process().unwrap();

        // Cannot process again
        assert!(req.process().is_err());
    }

    #[test]
    fn test_cannot_complete_non_processing() {
        let cid = sample_customer_id();
        let mut req = DataRightsRequest::new(cid, DataRequestType::Access, None);

        // Cannot complete from Pending
        assert!(req.complete("data".to_string()).is_err());
    }

    #[test]
    fn test_cannot_reject_completed() {
        let cid = sample_customer_id();
        let mut req = DataRightsRequest::new(cid, DataRequestType::Access, None);
        req.process().unwrap();
        req.complete("data".to_string()).unwrap();

        assert!(req.reject("nope".to_string()).is_err());
    }

    #[test]
    fn test_is_overdue_not_overdue_when_new() {
        let cid = sample_customer_id();
        let req = DataRightsRequest::new(cid, DataRequestType::Access, None);
        assert!(!req.is_overdue());
    }

    #[test]
    fn test_is_overdue_when_deadline_passed() {
        let cid = sample_customer_id();
        let now = Utc::now();
        let req = DataRightsRequest::reconstitute(
            DataRequestId::new(),
            cid,
            DataRequestType::Access,
            DataRequestStatus::Pending,
            None,
            None,
            now - Duration::days(31),
            None,
            now - Duration::days(1), // deadline was yesterday
        );
        assert!(req.is_overdue());
    }

    #[test]
    fn test_is_not_overdue_when_completed() {
        let cid = sample_customer_id();
        let now = Utc::now();
        let req = DataRightsRequest::reconstitute(
            DataRequestId::new(),
            cid,
            DataRequestType::Access,
            DataRequestStatus::Completed,
            None,
            Some("done".to_string()),
            now - Duration::days(31),
            Some(now),
            now - Duration::days(1),
        );
        assert!(!req.is_overdue());
    }

    #[test]
    fn test_data_request_type_from_str() {
        assert_eq!(
            DataRequestType::from_str_type("Access").unwrap(),
            DataRequestType::Access
        );
        assert_eq!(
            DataRequestType::from_str_type("rectification").unwrap(),
            DataRequestType::Rectification
        );
        assert_eq!(
            DataRequestType::from_str_type("opposition").unwrap(),
            DataRequestType::Opposition
        );
        assert!(DataRequestType::from_str_type("invalid").is_err());
    }

    #[test]
    fn test_data_request_status_from_str() {
        assert_eq!(
            DataRequestStatus::from_str_status("Pending").unwrap(),
            DataRequestStatus::Pending
        );
        assert_eq!(
            DataRequestStatus::from_str_status("processing").unwrap(),
            DataRequestStatus::Processing
        );
        assert_eq!(
            DataRequestStatus::from_str_status("completed").unwrap(),
            DataRequestStatus::Completed
        );
        assert_eq!(
            DataRequestStatus::from_str_status("rejected").unwrap(),
            DataRequestStatus::Rejected
        );
        assert!(DataRequestStatus::from_str_status("unknown").is_err());
    }

    #[test]
    fn test_data_request_id_display() {
        let id = DataRequestId::new();
        let display = format!("{id}");
        assert!(!display.is_empty());
    }
}
