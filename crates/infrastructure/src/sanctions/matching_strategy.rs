use banko_application::sanctions::IMatchingStrategy;
use banko_domain::sanctions::{screen_name, MatchDetail, SanctionEntry};

/// Default matching strategy: delegates to domain fuzzy_matcher.
pub struct DefaultMatchingStrategy;

impl IMatchingStrategy for DefaultMatchingStrategy {
    fn screen(&self, name: &str, entries: &[SanctionEntry], threshold: u8) -> Vec<MatchDetail> {
        screen_name(name, entries, threshold)
            .matched_entries()
            .to_vec()
    }
}
