use async_trait::async_trait;

use banko_application::customer::IPepCheckService;

/// In-memory PEP checker with a small hardcoded list.
/// In production, this would be backed by an external sanctions/PEP database.
pub struct InMemoryPepChecker {
    pep_names: Vec<String>,
}

impl InMemoryPepChecker {
    /// Create a PEP checker with a default list of known PEP names.
    pub fn new() -> Self {
        InMemoryPepChecker {
            pep_names: vec![
                "politically exposed person".to_string(),
                "john pep doe".to_string(),
            ],
        }
    }

    /// Create a PEP checker with a custom list.
    pub fn with_names(names: Vec<String>) -> Self {
        InMemoryPepChecker { pep_names: names }
    }
}

impl Default for InMemoryPepChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl IPepCheckService for InMemoryPepChecker {
    async fn is_pep(&self, full_name: &str) -> Result<bool, String> {
        let name_lower = full_name.to_lowercase();
        Ok(self
            .pep_names
            .iter()
            .any(|n| name_lower.contains(&n.to_lowercase())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pep_checker_default_not_pep() {
        let checker = InMemoryPepChecker::new();
        assert!(!checker.is_pep("Ahmed Ben Ayed").await.unwrap());
    }

    #[tokio::test]
    async fn test_pep_checker_detects_pep() {
        let checker = InMemoryPepChecker::new();
        assert!(checker
            .is_pep("Politically Exposed Person")
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_pep_checker_custom_list() {
        let checker =
            InMemoryPepChecker::with_names(vec!["suspicious name".to_string()]);
        assert!(checker.is_pep("A Suspicious Name Here").await.unwrap());
        assert!(!checker.is_pep("Normal Person").await.unwrap());
    }
}
