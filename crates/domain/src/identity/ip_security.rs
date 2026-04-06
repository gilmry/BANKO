use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::str::FromStr;
use uuid::Uuid;

use crate::shared::errors::DomainError;

/// Whitelist of allowed IP addresses for a customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpWhitelist {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub allowed_ips: Vec<String>,
    pub is_strict_mode: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IpWhitelist {
    pub fn new(customer_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            customer_id,
            allowed_ips: Vec::new(),
            is_strict_mode: false,
            created_at: now,
            updated_at: now,
        }
    }

    /// Add an IP address to the whitelist
    pub fn add_ip(&mut self, ip: &str) -> Result<(), DomainError> {
        // Validate IP format
        if !self.is_valid_ip(ip) {
            return Err(DomainError::InvalidInput(format!(
                "Invalid IP address format: {}",
                ip
            )));
        }

        // Check for duplicates
        if self.allowed_ips.contains(&ip.to_string()) {
            return Err(DomainError::InvalidInput(format!(
                "IP address already whitelisted: {}",
                ip
            )));
        }

        self.allowed_ips.push(ip.to_string());
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove an IP address from the whitelist
    pub fn remove_ip(&mut self, ip: &str) -> Result<(), DomainError> {
        if !self.allowed_ips.contains(&ip.to_string()) {
            return Err(DomainError::InvalidInput(format!(
                "IP address not in whitelist: {}",
                ip
            )));
        }

        self.allowed_ips.retain(|x| x != ip);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Check if an IP is in the whitelist
    pub fn is_allowed(&self, ip: &str) -> bool {
        self.allowed_ips
            .iter()
            .any(|whitelisted| self.ips_match(ip, whitelisted))
    }

    /// Check if whitelist is in strict mode (only whitelisted IPs allowed)
    pub fn is_strict(&self) -> bool {
        self.is_strict_mode
    }

    /// Set strict mode
    pub fn set_strict_mode(&mut self, strict: bool) {
        self.is_strict_mode = strict;
        self.updated_at = Utc::now();
    }

    /// Validate IP address format (both IPv4 and IPv6)
    fn is_valid_ip(&self, ip: &str) -> bool {
        IpAddr::from_str(ip).is_ok()
    }

    /// Compare two IPs (handles both string and binary comparisons)
    fn ips_match(&self, ip1: &str, ip2: &str) -> bool {
        // Direct string match
        if ip1 == ip2 {
            return true;
        }

        // Parse and compare as IP addresses
        if let (Ok(addr1), Ok(addr2)) = (IpAddr::from_str(ip1), IpAddr::from_str(ip2)) {
            return addr1 == addr2;
        }

        false
    }
}

/// Geolocation information for a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub country_code: String,
    pub country_name: String,
    pub city: Option<String>,
    pub is_vpn: bool,
}

impl GeoLocation {
    pub fn new(country_code: String, country_name: String) -> Self {
        Self {
            country_code,
            country_name,
            city: None,
            is_vpn: false,
        }
    }

    pub fn with_city(mut self, city: String) -> Self {
        self.city = Some(city);
        self
    }

    pub fn with_vpn(mut self, is_vpn: bool) -> Self {
        self.is_vpn = is_vpn;
        self
    }
}

/// Decision from geo-blocking evaluation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeoDecision {
    Allow,
    RequireMfa,
    Block,
    Alert,
}

/// Rules for geo-blocking and geo-fencing
#[derive(Debug, Clone)]
pub struct GeoBlockRule {
    pub blocked_countries: Vec<String>,
    pub alert_countries: Vec<String>,
}

impl GeoBlockRule {
    pub fn new() -> Self {
        Self {
            blocked_countries: vec![
                "IR".to_string(), // Iran
                "SY".to_string(), // Syria
                "KP".to_string(), // North Korea
                "CU".to_string(), // Cuba
            ],
            alert_countries: vec![
                "RU".to_string(), // Russia
                "BY".to_string(), // Belarus
            ],
        }
    }

    pub fn with_blocked_countries(mut self, countries: Vec<String>) -> Self {
        self.blocked_countries = countries;
        self
    }

    pub fn with_alert_countries(mut self, countries: Vec<String>) -> Self {
        self.alert_countries = countries;
        self
    }

    /// Evaluate geolocation against blocking rules
    pub fn evaluate(&self, location: &GeoLocation) -> GeoDecision {
        // VPN usage is suspicious
        if location.is_vpn {
            return GeoDecision::RequireMfa;
        }

        // Blocked countries are immediately rejected
        if self.blocked_countries.contains(&location.country_code) {
            return GeoDecision::Block;
        }

        // Alert countries require MFA
        if self.alert_countries.contains(&location.country_code) {
            return GeoDecision::RequireMfa;
        }

        GeoDecision::Allow
    }
}

impl Default for GeoBlockRule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_valid_ipv4_address() {
        let mut whitelist = IpWhitelist::new(Uuid::new_v4());
        let result = whitelist.add_ip("192.168.1.1");
        assert!(result.is_ok());
        assert!(whitelist.is_allowed("192.168.1.1"));
    }

    #[test]
    fn test_add_valid_ipv6_address() {
        let mut whitelist = IpWhitelist::new(Uuid::new_v4());
        let result = whitelist.add_ip("2001:0db8:85a3:0000:0000:8a2e:0370:7334");
        assert!(result.is_ok());
        assert!(whitelist.is_allowed("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
    }

    #[test]
    fn test_add_invalid_ip_address() {
        let mut whitelist = IpWhitelist::new(Uuid::new_v4());
        let result = whitelist.add_ip("invalid-ip");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_duplicate_ip_address() {
        let mut whitelist = IpWhitelist::new(Uuid::new_v4());
        assert!(whitelist.add_ip("10.0.0.1").is_ok());
        let result = whitelist.add_ip("10.0.0.1");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_ip_address() {
        let mut whitelist = IpWhitelist::new(Uuid::new_v4());
        assert!(whitelist.add_ip("172.16.0.1").is_ok());
        assert!(whitelist.is_allowed("172.16.0.1"));

        let result = whitelist.remove_ip("172.16.0.1");
        assert!(result.is_ok());
        assert!(!whitelist.is_allowed("172.16.0.1"));
    }

    #[test]
    fn test_remove_nonexistent_ip() {
        let mut whitelist = IpWhitelist::new(Uuid::new_v4());
        let result = whitelist.remove_ip("1.1.1.1");
        assert!(result.is_err());
    }

    #[test]
    fn test_is_allowed_empty_whitelist() {
        let whitelist = IpWhitelist::new(Uuid::new_v4());
        assert!(!whitelist.is_allowed("8.8.8.8"));
    }

    #[test]
    fn test_strict_mode_toggle() {
        let mut whitelist = IpWhitelist::new(Uuid::new_v4());
        assert!(!whitelist.is_strict());

        whitelist.set_strict_mode(true);
        assert!(whitelist.is_strict());

        whitelist.set_strict_mode(false);
        assert!(!whitelist.is_strict());
    }

    #[test]
    fn test_geolocation_creation() {
        let geo = GeoLocation::new("US".to_string(), "United States".to_string());
        assert_eq!(geo.country_code, "US");
        assert_eq!(geo.country_name, "United States");
        assert!(!geo.is_vpn);
    }

    #[test]
    fn test_geolocation_with_city() {
        let geo = GeoLocation::new("US".to_string(), "United States".to_string())
            .with_city("New York".to_string());
        assert_eq!(geo.city, Some("New York".to_string()));
    }

    #[test]
    fn test_geolocation_with_vpn() {
        let geo = GeoLocation::new("US".to_string(), "United States".to_string()).with_vpn(true);
        assert!(geo.is_vpn);
    }

    #[test]
    fn test_geo_block_rule_allows_safe_country() {
        let rule = GeoBlockRule::new();
        let geo = GeoLocation::new("US".to_string(), "United States".to_string());
        assert_eq!(rule.evaluate(&geo), GeoDecision::Allow);
    }

    #[test]
    fn test_geo_block_rule_blocks_iran() {
        let rule = GeoBlockRule::new();
        let geo = GeoLocation::new("IR".to_string(), "Iran".to_string());
        assert_eq!(rule.evaluate(&geo), GeoDecision::Block);
    }

    #[test]
    fn test_geo_block_rule_requires_mfa_for_alert_countries() {
        let rule = GeoBlockRule::new();
        let geo = GeoLocation::new("RU".to_string(), "Russia".to_string());
        assert_eq!(rule.evaluate(&geo), GeoDecision::RequireMfa);
    }

    #[test]
    fn test_geo_block_rule_requires_mfa_for_vpn() {
        let rule = GeoBlockRule::new();
        let geo = GeoLocation::new("US".to_string(), "United States".to_string()).with_vpn(true);
        assert_eq!(rule.evaluate(&geo), GeoDecision::RequireMfa);
    }
}
