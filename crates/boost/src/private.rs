// Copyright 2025 rustgram-client
//
// Licensed under MIT License

//! Internal implementation details for boost module.

use crate::error::{BoostError, Result};
use crate::types::DialogBoostLinkInfo;
use rustgram_types::ChannelId;

/// Parse a boost link URL to extract boost link info.
///
/// # Arguments
/// * `url` - The boost link URL (e.g., "https://t.me/boost/mychannel" or "https://t.me/boost?c=123")
///
/// # Returns
/// Parsed `DialogBoostLinkInfo`
pub fn parse_boost_link(url: &str) -> Result<DialogBoostLinkInfo> {
    let url = url.trim();

    // Validate URL format
    if !url.starts_with("https://t.me/boost") && !url.starts_with("t.me/boost") {
        return Err(BoostError::InvalidBoostLink(
            "URL must be a t.me boost link".to_string(),
        ));
    }

    // Extract path after /boost
    let path = if url.contains("t.me/boost/") {
        url.split("t.me/boost/").nth(1)
    } else if url.contains("?c=") {
        // Private channel link: t.me/boost?c=123
        let part = url.split("?c=").nth(1);
        return match part {
            Some(channel_id_str) => {
                let channel_id = channel_id_str
                    .parse::<i64>()
                    .map_err(|_| BoostError::InvalidBoostLink("Invalid channel ID".to_string()))?;
                let channel_id = ChannelId::try_from(channel_id).map_err(|_| {
                    BoostError::InvalidBoostLink("Invalid channel ID range".to_string())
                })?;
                Ok(DialogBoostLinkInfo::private(channel_id))
            }
            None => Err(BoostError::InvalidBoostLink(
                "Missing channel ID".to_string(),
            )),
        };
    } else {
        // Public link without username (e.g., t.me/boost)
        return Ok(DialogBoostLinkInfo {
            username: None,
            channel_id: None,
        });
    };

    match path {
        Some(username) => {
            // Extract username (remove any query parameters)
            let username = username
                .split('?')
                .next()
                .ok_or_else(|| BoostError::InvalidBoostLink("Invalid username".to_string()))?;
            if username.is_empty() {
                Err(BoostError::InvalidBoostLink("Empty username".to_string()))
            } else {
                Ok(DialogBoostLinkInfo::public(username.to_string()))
            }
        }
        None => Err(BoostError::InvalidBoostLink("Missing username".to_string())),
    }
}

/// Format a boost link for a dialog.
///
/// # Arguments
/// * `username` - Optional username for public channels
/// * `channel_id` - Channel ID for private channels
///
/// # Returns
/// Formatted boost link URL and a boolean indicating if it's public
pub fn format_boost_link(username: Option<&str>, channel_id: ChannelId) -> (String, bool) {
    match username {
        Some(un) if !un.is_empty() => {
            // Public link
            (format!("https://t.me/boost/{}", un), true)
        }
        _ => {
            // Private link
            (
                format!("https://t.me/boost?c={}", i64::from(channel_id)),
                false,
            )
        }
    }
}

/// Validate slot IDs.
///
/// # Arguments
/// * `slot_ids` - List of slot IDs to validate
///
/// # Returns
/// Ok if valid, Err otherwise
pub fn validate_slot_ids(slot_ids: &[i32]) -> Result<()> {
    for &slot_id in slot_ids {
        if slot_id < 0 {
            return Err(BoostError::InvalidSlotId(format!(
                "Slot ID must be non-negative, got {}",
                slot_id
            )));
        }
    }
    Ok(())
}

/// Validate limit parameter.
///
/// # Arguments
/// * `limit` - The limit to validate
///
/// # Returns
/// Ok if valid, Err otherwise
pub fn validate_limit(limit: i32) -> Result<()> {
    if limit <= 0 {
        return Err(BoostError::InvalidLimit(limit));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_boost_link_public() {
        let info = parse_boost_link("https://t.me/boost/mychannel").unwrap();
        assert!(info.is_public());
        assert_eq!(info.username, Some("mychannel".to_string()));
        assert!(info.channel_id.is_none());
    }

    #[test]
    fn test_parse_boost_link_public_short() {
        let info = parse_boost_link("t.me/boost/mychannel").unwrap();
        assert!(info.is_public());
        assert_eq!(info.username, Some("mychannel".to_string()));
    }

    #[test]
    fn test_parse_boost_link_private() {
        let info = parse_boost_link("https://t.me/boost?c=12345").unwrap();
        assert!(!info.is_public());
        assert!(info.username.is_none());
        assert_eq!(info.channel_id, ChannelId::try_from(12345).ok());
    }

    #[test]
    fn test_parse_boost_link_invalid() {
        let result = parse_boost_link("https://example.com/boost");
        assert!(result.is_err());
    }

    #[test]
    fn test_format_boost_link_public() {
        let channel_id = ChannelId::try_from(12345).unwrap();
        let (url, is_public) = format_boost_link(Some("mychannel"), channel_id);
        assert!(is_public);
        assert_eq!(url, "https://t.me/boost/mychannel");
    }

    #[test]
    fn test_format_boost_link_private() {
        let channel_id = ChannelId::try_from(12345).unwrap();
        let (url, is_public) = format_boost_link(None, channel_id);
        assert!(!is_public);
        assert_eq!(url, "https://t.me/boost?c=12345");
    }

    #[test]
    fn test_validate_slot_ids_valid() {
        assert!(validate_slot_ids(&[0, 1, 2]).is_ok());
        assert!(validate_slot_ids(&[5]).is_ok());
    }

    #[test]
    fn test_validate_slot_ids_invalid() {
        assert!(validate_slot_ids(&[-1]).is_err());
        assert!(validate_slot_ids(&[1, -2, 3]).is_err());
    }

    #[test]
    fn test_validate_limit() {
        assert!(validate_limit(1).is_ok());
        assert!(validate_limit(100).is_ok());
        assert!(validate_limit(0).is_err());
        assert!(validate_limit(-1).is_err());
    }

    #[test]
    fn test_std_clamp() {
        // Test clamp behavior using manual implementation
        let clamp_fn = |value: i32, min: i32, max: i32| -> i32 {
            if value < min {
                min
            } else if value > max {
                max
            } else {
                value
            }
        };
        assert_eq!(clamp_fn(5, 0, 10), 5);
        assert_eq!(clamp_fn(-1, 0, 10), 0);
        assert_eq!(clamp_fn(15, 0, 10), 10);
    }

    #[test]
    fn test_std_max() {
        use std::cmp::max;
        assert_eq!(max(1, 2), 2);
        assert_eq!(max(10, 5), 10);
        assert_eq!(max(5, 5), 5);
    }
}
