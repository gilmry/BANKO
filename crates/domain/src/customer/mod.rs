mod consent;
mod data_rights;
mod entities;
mod grouping;  // FR-007: Economic grouping
mod value_objects;

pub use consent::*;
pub use data_rights::*;
pub use entities::Customer;
pub use grouping::{CustomerGroup, GroupId, GroupType};  // FR-007
pub use value_objects::{
    Address, Beneficiary, Cin, ConsentStatus, CustomerSegment, CustomerStatus, CustomerType,
    Document, DocumentType, KycProfile, PepStatus, RiskLevel, RiskScore, SourceOfFunds,  // FR-006, FR-008
};
