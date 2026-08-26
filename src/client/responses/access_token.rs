use crate::client::responses::token_response::TokenResponse;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessToken {
    pub token_type: String,
    pub value: String,
    pub issued_at: String,
}

impl AccessToken {

    pub fn new(access_token: String, issued_at: String, token_type: String) -> Self {
        Self {
            token_type,
            value: access_token,
            issued_at,
        }
    }

    pub fn from_token_response(token_response: TokenResponse) -> Self {
        Self {
            token_type: token_response.token_type.unwrap_or_default(),
            value: token_response.access_token,
            issued_at: token_response.issued_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        let timestamp_ms = match self.issued_at.parse::<u64>() {
            Ok(ts) => ts,
            Err(_) => {
                // SOAP login returns ISO timestamp; can't compare, attempt refresh
                log::debug!("Could not parse issued_at as timestamp, attempting refresh.");
                return true;
            }
        };
        let seconds = timestamp_ms / 1000;
        let nanos = (timestamp_ms % 1000) * 1_000_000;

        let given_time = UNIX_EPOCH + Duration::new(seconds, nanos as u32);

        let two_hours = Duration::from_secs(2 * 60 * 60); // 2 hours in seconds
        let modified_time = given_time + two_hours;

        let current_time = SystemTime::now();

        if current_time > modified_time {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let token = AccessToken::default();
        assert_eq!(token.token_type, "");
        assert_eq!(token.value, "");
        assert_eq!(token.issued_at, "");
    }

    #[test]
    fn test_clone() {
        let token = AccessToken {
            token_type: "Bearer".to_string(),
            value: "abc123".to_string(),
            issued_at: "1234567890".to_string(),
        };
        let cloned = token.clone();
        assert_eq!(cloned.token_type, "Bearer");
        assert_eq!(cloned.value, "abc123");
        assert_eq!(cloned.issued_at, "1234567890");
    }

    #[test]
    fn test_debug() {
        let token = AccessToken {
            token_type: "Bearer".to_string(),
            value: "tok".to_string(),
            issued_at: "123".to_string(),
        };
        let debug = format!("{:?}", token);
        assert!(debug.contains("Bearer"));
        assert!(debug.contains("tok"));
        assert!(debug.contains("123"));
    }
}
