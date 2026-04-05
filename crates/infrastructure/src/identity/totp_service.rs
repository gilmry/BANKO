use banko_application::identity::ITotpService;
use totp_lite::{totp_custom, Sha1, DEFAULT_STEP};

pub struct TotpServiceImpl;

impl TotpServiceImpl {
    pub fn new() -> Self {
        TotpServiceImpl
    }
}

impl Default for TotpServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ITotpService for TotpServiceImpl {
    fn generate_secret(&self) -> String {
        let secret: Vec<u8> = (0..20).map(|_| rand::random::<u8>()).collect();
        base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &secret)
    }

    fn generate_totp_uri(&self, secret: &str, email: &str) -> String {
        format!(
            "otpauth://totp/BANKO:{}?secret={}&issuer=BANKO&algorithm=SHA1&digits=6&period=30",
            email, secret
        )
    }

    fn verify_code(&self, secret: &str, code: &str) -> bool {
        let secret_bytes =
            match base32::decode(base32::Alphabet::Rfc4648 { padding: false }, secret) {
                Some(b) => b,
                None => return false,
            };

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Check current and adjacent time steps (30s window each side)
        for offset in [-1i64, 0, 1] {
            let time = (now as i64 + offset * DEFAULT_STEP as i64) as u64;
            let generated = totp_custom::<Sha1>(DEFAULT_STEP, 6, &secret_bytes, time);
            if generated == code {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret() {
        let service = TotpServiceImpl::new();
        let secret = service.generate_secret();
        assert!(!secret.is_empty());
        // Base32 encoded 20 bytes = 32 chars
        assert_eq!(secret.len(), 32);
    }

    #[test]
    fn test_generate_totp_uri() {
        let service = TotpServiceImpl::new();
        let uri = service.generate_totp_uri("JBSWY3DPEHPK3PXP", "test@banko.tn");
        assert!(uri.starts_with("otpauth://totp/BANKO:test@banko.tn"));
        assert!(uri.contains("secret=JBSWY3DPEHPK3PXP"));
        assert!(uri.contains("issuer=BANKO"));
    }

    #[test]
    fn test_verify_code_with_current_totp() {
        let service = TotpServiceImpl::new();
        let secret = service.generate_secret();

        // Generate a valid code for current time
        let secret_bytes =
            base32::decode(base32::Alphabet::Rfc4648 { padding: false }, &secret).unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let valid_code = totp_custom::<Sha1>(DEFAULT_STEP, 6, &secret_bytes, now);

        assert!(service.verify_code(&secret, &valid_code));
    }

    #[test]
    fn test_verify_invalid_code() {
        let service = TotpServiceImpl::new();
        let secret = service.generate_secret();
        // Very unlikely to be a valid code
        assert!(!service.verify_code(&secret, "000000"));
    }

    #[test]
    fn test_verify_invalid_secret() {
        let service = TotpServiceImpl::new();
        assert!(!service.verify_code("!!!INVALID!!!", "123456"));
    }
}
