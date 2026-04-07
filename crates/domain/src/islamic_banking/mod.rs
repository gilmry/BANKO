/// Islamic Banking Bounded Context
///
/// Implements Sharia-compliant banking products and governance per:
/// - BMAD v4.0.1 (Islamic Banking Standard)
/// - Loi 2016-33 (Tunisia Islamic Banking Regulation)
/// - Quranic principles of profit-sharing and asset-backed financing
///
/// Products implemented:
/// - Murabaha: Cost-plus-profit sale financing
/// - Ijara: Leasing with purchase option
/// - Musharaka: Partnership financing (diminishing)
/// - Mudaraba: Profit-sharing investment
/// - Sukuk: Islamic bonds
/// - Waqf: Endowment management
/// - Zakat: 2.5% wealth tax calculation
/// - Takaful: Islamic insurance reference
/// - Sharia Board: Governance and compliance
/// - Profit Distribution: Sharia-compliant returns
pub mod entities;
pub mod value_objects;
pub mod invariants;
pub mod errors;

pub use entities::*;
pub use value_objects::*;
pub use errors::IslamicBankingError;
