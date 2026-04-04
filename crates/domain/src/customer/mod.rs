mod entities;
mod value_objects;

pub use entities::Customer;
pub use value_objects::{
    Address, Beneficiary, Cin, ConsentStatus, CustomerStatus, CustomerType, KycProfile, PepStatus,
    RiskLevel, RiskScore, SourceOfFunds,
};
