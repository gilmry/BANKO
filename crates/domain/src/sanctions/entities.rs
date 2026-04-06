use std::fmt;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::errors::DomainError;

use super::fuzzy_matcher;

// --- Newtypes ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SanctionListId(Uuid);

impl SanctionListId {
    pub fn new() -> Self {
        SanctionListId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        SanctionListId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for SanctionListId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SanctionListId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SanctionEntryId(Uuid);

impl SanctionEntryId {
    pub fn new() -> Self {
        SanctionEntryId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        SanctionEntryId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for SanctionEntryId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SanctionEntryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScreeningResultId(Uuid);

impl ScreeningResultId {
    pub fn new() -> Self {
        ScreeningResultId(Uuid::new_v4())
    }
    pub fn from_uuid(id: Uuid) -> Self {
        ScreeningResultId(id)
    }
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ScreeningResultId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ScreeningResultId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- Enums ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ListSource {
    UN,
    EU,
    OFAC,
    National,
}

impl ListSource {
    pub fn as_str(&self) -> &str {
        match self {
            ListSource::UN => "UN",
            ListSource::EU => "EU",
            ListSource::OFAC => "OFAC",
            ListSource::National => "National",
        }
    }

    pub fn from_str_source(s: &str) -> Result<Self, DomainError> {
        match s {
            "UN" => Ok(ListSource::UN),
            "EU" => Ok(ListSource::EU),
            "OFAC" => Ok(ListSource::OFAC),
            "National" => Ok(ListSource::National),
            _ => Err(DomainError::InvalidSanctionList(format!(
                "Unknown list source: {s}"
            ))),
        }
    }
}

impl fmt::Display for ListSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScreeningStatus {
    Clear,
    PotentialMatch,
    Hit,
}

impl ScreeningStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ScreeningStatus::Clear => "Clear",
            ScreeningStatus::PotentialMatch => "PotentialMatch",
            ScreeningStatus::Hit => "Hit",
        }
    }

    pub fn from_str_status(s: &str) -> Result<Self, DomainError> {
        match s {
            "Clear" => Ok(ScreeningStatus::Clear),
            "PotentialMatch" => Ok(ScreeningStatus::PotentialMatch),
            "Hit" => Ok(ScreeningStatus::Hit),
            _ => Err(DomainError::InvalidScreeningResult(format!(
                "Unknown screening status: {s}"
            ))),
        }
    }
}

// --- Sanction Entry ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SanctionEntry {
    id: SanctionEntryId,
    list_source: ListSource,
    full_name: String,
    aliases: Vec<String>,
    country: Option<String>,
    listing_date: Option<NaiveDate>,
    delisting_date: Option<NaiveDate>,
    additional_info: Option<String>,
    active: bool,
}

impl SanctionEntry {
    pub fn new(
        list_source: ListSource,
        full_name: String,
        aliases: Vec<String>,
        country: Option<String>,
        listing_date: Option<NaiveDate>,
    ) -> Result<Self, DomainError> {
        if full_name.trim().is_empty() {
            return Err(DomainError::InvalidSanctionEntry(
                "Name cannot be empty".to_string(),
            ));
        }
        Ok(SanctionEntry {
            id: SanctionEntryId::new(),
            list_source,
            full_name: full_name.trim().to_string(),
            aliases,
            country,
            listing_date,
            delisting_date: None,
            additional_info: None,
            active: true,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        id: SanctionEntryId,
        list_source: ListSource,
        full_name: String,
        aliases: Vec<String>,
        country: Option<String>,
        listing_date: Option<NaiveDate>,
        delisting_date: Option<NaiveDate>,
        additional_info: Option<String>,
        active: bool,
    ) -> Self {
        SanctionEntry {
            id,
            list_source,
            full_name,
            aliases,
            country,
            listing_date,
            delisting_date,
            additional_info,
            active,
        }
    }

    pub fn deactivate(&mut self, delisting_date: NaiveDate) {
        self.active = false;
        self.delisting_date = Some(delisting_date);
    }

    // Accessors
    pub fn id(&self) -> &SanctionEntryId {
        &self.id
    }
    pub fn list_source(&self) -> ListSource {
        self.list_source
    }
    pub fn full_name(&self) -> &str {
        &self.full_name
    }
    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }
    pub fn country(&self) -> Option<&str> {
        self.country.as_deref()
    }
    pub fn listing_date(&self) -> Option<NaiveDate> {
        self.listing_date
    }
    pub fn delisting_date(&self) -> Option<NaiveDate> {
        self.delisting_date
    }
    pub fn additional_info(&self) -> Option<&str> {
        self.additional_info.as_deref()
    }
    pub fn active(&self) -> bool {
        self.active
    }
}

// --- Sanction List (Aggregate Root) ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SanctionList {
    id: SanctionListId,
    source: ListSource,
    version: String,
    entries: Vec<SanctionEntry>,
    last_updated: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl SanctionList {
    pub fn new(source: ListSource, version: String) -> Self {
        let now = Utc::now();
        SanctionList {
            id: SanctionListId::new(),
            source,
            version,
            entries: Vec::new(),
            last_updated: now,
            created_at: now,
        }
    }

    pub fn from_parts(
        id: SanctionListId,
        source: ListSource,
        version: String,
        entries: Vec<SanctionEntry>,
        last_updated: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Self {
        SanctionList {
            id,
            source,
            version,
            entries,
            last_updated,
            created_at,
        }
    }

    pub fn add_entry(&mut self, entry: SanctionEntry) {
        self.entries.push(entry);
        self.last_updated = Utc::now();
    }

    pub fn remove_entry(&mut self, entry_id: &SanctionEntryId, delisting_date: NaiveDate) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id() == entry_id) {
            entry.deactivate(delisting_date);
        }
        self.last_updated = Utc::now();
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn active_entries(&self) -> Vec<&SanctionEntry> {
        self.entries.iter().filter(|e| e.active()).collect()
    }

    // Accessors
    pub fn id(&self) -> &SanctionListId {
        &self.id
    }
    pub fn source(&self) -> ListSource {
        self.source
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn entries(&self) -> &[SanctionEntry] {
        &self.entries
    }
    pub fn last_updated(&self) -> DateTime<Utc> {
        self.last_updated
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

// --- Match Detail ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchDetail {
    pub entry_id: SanctionEntryId,
    pub entry_name: String,
    pub list_source: ListSource,
    pub score: u8,
    pub matched_fields: Vec<String>,
}

// --- Screening Result ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreeningResult {
    id: ScreeningResultId,
    screened_name: String,
    matched_entries: Vec<MatchDetail>,
    status: ScreeningStatus,
    screened_at: DateTime<Utc>,
}

/// Default thresholds
pub const HIT_THRESHOLD: u8 = 80;
pub const POTENTIAL_MATCH_THRESHOLD: u8 = 60;

impl ScreeningResult {
    pub fn new(screened_name: String, matched_entries: Vec<MatchDetail>) -> Self {
        let highest = matched_entries.iter().map(|m| m.score).max().unwrap_or(0);
        let status = if highest >= HIT_THRESHOLD {
            ScreeningStatus::Hit
        } else if highest >= POTENTIAL_MATCH_THRESHOLD {
            ScreeningStatus::PotentialMatch
        } else {
            ScreeningStatus::Clear
        };

        ScreeningResult {
            id: ScreeningResultId::new(),
            screened_name,
            matched_entries,
            status,
            screened_at: Utc::now(),
        }
    }

    pub fn from_parts(
        id: ScreeningResultId,
        screened_name: String,
        matched_entries: Vec<MatchDetail>,
        status: ScreeningStatus,
        screened_at: DateTime<Utc>,
    ) -> Self {
        ScreeningResult {
            id,
            screened_name,
            matched_entries,
            status,
            screened_at,
        }
    }

    pub fn highest_score(&self) -> u8 {
        self.matched_entries
            .iter()
            .map(|m| m.score)
            .max()
            .unwrap_or(0)
    }

    pub fn is_hit(&self) -> bool {
        self.status == ScreeningStatus::Hit
    }

    pub fn is_clear(&self) -> bool {
        self.status == ScreeningStatus::Clear
    }

    // Accessors
    pub fn id(&self) -> &ScreeningResultId {
        &self.id
    }
    pub fn screened_name(&self) -> &str {
        &self.screened_name
    }
    pub fn matched_entries(&self) -> &[MatchDetail] {
        &self.matched_entries
    }
    pub fn status(&self) -> ScreeningStatus {
        self.status
    }
    pub fn screened_at(&self) -> DateTime<Utc> {
        self.screened_at
    }
}

/// High-level screening function: screen a name against all entries in a list.
pub fn screen_name(name: &str, entries: &[SanctionEntry], threshold: u8) -> ScreeningResult {
    let active_entries: Vec<_> = entries.iter().filter(|e| e.active()).collect();

    let entry_data: Vec<(String, Vec<String>)> = active_entries
        .iter()
        .map(|e| (e.full_name().to_string(), e.aliases().to_vec()))
        .collect();

    let raw_matches = fuzzy_matcher::screen_name_against_entries(name, &entry_data, threshold);

    let matched_entries: Vec<MatchDetail> = raw_matches
        .into_iter()
        .map(|(idx, matched_name, score)| {
            let entry = active_entries[idx];
            MatchDetail {
                entry_id: entry.id().clone(),
                entry_name: matched_name,
                list_source: entry.list_source(),
                score,
                matched_fields: vec!["name".to_string()],
            }
        })
        .collect();

    ScreeningResult::new(name.to_string(), matched_entries)
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(name: &str, source: ListSource) -> SanctionEntry {
        SanctionEntry::new(source, name.to_string(), vec![], None, None).unwrap()
    }

    fn make_entry_with_aliases(
        name: &str,
        aliases: Vec<&str>,
        source: ListSource,
    ) -> SanctionEntry {
        SanctionEntry::new(
            source,
            name.to_string(),
            aliases.into_iter().map(|s| s.to_string()).collect(),
            None,
            None,
        )
        .unwrap()
    }

    // --- SanctionEntry tests ---

    #[test]
    fn test_create_entry() {
        let entry = make_entry("Jean Alaoui", ListSource::UN);
        assert_eq!(entry.full_name(), "Jean Alaoui");
        assert_eq!(entry.list_source(), ListSource::UN);
        assert!(entry.active());
    }

    #[test]
    fn test_entry_empty_name_rejected() {
        let result = SanctionEntry::new(ListSource::UN, "".to_string(), vec![], None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_entry_deactivate() {
        let mut entry = make_entry("Test", ListSource::EU);
        let date = NaiveDate::from_ymd_opt(2026, 4, 1).unwrap();
        entry.deactivate(date);
        assert!(!entry.active());
        assert_eq!(entry.delisting_date(), Some(date));
    }

    // --- SanctionList tests ---

    #[test]
    fn test_create_list() {
        let list = SanctionList::new(ListSource::UN, "2026-04".to_string());
        assert_eq!(list.source(), ListSource::UN);
        assert_eq!(list.entry_count(), 0);
    }

    #[test]
    fn test_add_entry_to_list() {
        let mut list = SanctionList::new(ListSource::UN, "2026-04".to_string());
        let entry = make_entry("Jean Alaoui", ListSource::UN);
        list.add_entry(entry);
        assert_eq!(list.entry_count(), 1);
        assert_eq!(list.active_entries().len(), 1);
    }

    #[test]
    fn test_remove_entry_marks_inactive() {
        let mut list = SanctionList::new(ListSource::UN, "2026-04".to_string());
        let entry = make_entry("Jean Alaoui", ListSource::UN);
        let entry_id = entry.id().clone();
        list.add_entry(entry);

        let date = NaiveDate::from_ymd_opt(2026, 4, 1).unwrap();
        list.remove_entry(&entry_id, date);

        assert_eq!(list.entry_count(), 1); // still in list
        assert_eq!(list.active_entries().len(), 0); // but inactive
    }

    // --- ScreeningResult tests ---

    #[test]
    fn test_screening_clear() {
        let result = ScreeningResult::new("John Smith".to_string(), vec![]);
        assert_eq!(result.status(), ScreeningStatus::Clear);
        assert_eq!(result.highest_score(), 0);
        assert!(result.is_clear());
    }

    #[test]
    fn test_screening_hit() {
        let matches = vec![MatchDetail {
            entry_id: SanctionEntryId::new(),
            entry_name: "Jean Alaoui".to_string(),
            list_source: ListSource::UN,
            score: 95,
            matched_fields: vec!["name".to_string()],
        }];
        let result = ScreeningResult::new("Jean Alaoui".to_string(), matches);
        assert_eq!(result.status(), ScreeningStatus::Hit);
        assert!(result.is_hit());
    }

    #[test]
    fn test_screening_potential_match() {
        let matches = vec![MatchDetail {
            entry_id: SanctionEntryId::new(),
            entry_name: "Jean Alaoui".to_string(),
            list_source: ListSource::UN,
            score: 70,
            matched_fields: vec!["name".to_string()],
        }];
        let result = ScreeningResult::new("Jean Alo".to_string(), matches);
        assert_eq!(result.status(), ScreeningStatus::PotentialMatch);
    }

    // --- screen_name integration ---

    #[test]
    fn test_screen_name_exact_match() {
        let entries = vec![make_entry("Jean Alaoui", ListSource::UN)];
        let result = screen_name("Jean Alaoui", &entries, 80);
        assert!(result.is_hit());
        assert_eq!(result.highest_score(), 100);
    }

    #[test]
    fn test_screen_name_typo_match() {
        let entries = vec![make_entry("Jean Alaoui", ListSource::UN)];
        let result = screen_name("Jean Alaouie", &entries, 80);
        assert!(result.is_hit());
        assert!(result.highest_score() > 80);
    }

    #[test]
    fn test_screen_name_case_insensitive() {
        let entries = vec![make_entry("Jean Alaoui", ListSource::UN)];
        let result = screen_name("JEAN ALAOUI", &entries, 80);
        assert!(result.is_hit());
    }

    #[test]
    fn test_screen_name_no_match() {
        let entries = vec![make_entry("Mohammed Ben Ali", ListSource::UN)];
        let result = screen_name("Pierre Dupont", &entries, 80);
        assert!(result.is_clear());
    }

    #[test]
    fn test_screen_name_alias_match() {
        let entries = vec![make_entry_with_aliases(
            "Full Official Name",
            vec!["Jean Alaoui"],
            ListSource::UN,
        )];
        let result = screen_name("Jean Alaoui", &entries, 80);
        assert!(result.is_hit());
    }

    #[test]
    fn test_screen_name_inactive_entry_ignored() {
        let mut entry = make_entry("Jean Alaoui", ListSource::UN);
        entry.deactivate(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
        let entries = vec![entry];
        let result = screen_name("Jean Alaoui", &entries, 80);
        assert!(result.is_clear());
    }

    #[test]
    fn test_list_source_display() {
        assert_eq!(ListSource::UN.as_str(), "UN");
        assert_eq!(ListSource::EU.as_str(), "EU");
        assert_eq!(ListSource::OFAC.as_str(), "OFAC");
        assert_eq!(ListSource::National.as_str(), "National");
    }

    #[test]
    fn test_list_source_from_str() {
        assert_eq!(ListSource::from_str_source("UN").unwrap(), ListSource::UN);
        assert!(ListSource::from_str_source("INVALID").is_err());
    }

    #[test]
    fn test_screening_status_from_str() {
        assert_eq!(
            ScreeningStatus::from_str_status("Clear").unwrap(),
            ScreeningStatus::Clear
        );
        assert_eq!(
            ScreeningStatus::from_str_status("Hit").unwrap(),
            ScreeningStatus::Hit
        );
        assert!(ScreeningStatus::from_str_status("Invalid").is_err());
    }
}
