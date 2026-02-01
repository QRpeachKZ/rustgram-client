//! TL (Type Language) serialization support for birthdate.

use crate::birthdate::{Birthdate, BIRTHDAY_MAGIC, YEAR_FLAG_MASK};

/// Telegram API birthday object.
///
/// From TL schema: `birthday#6c8e1e06 flags:# day:int month:int year:flags.0?int = Birthday`
///
/// # Examples
///
/// ```
/// use rustgram_birthdate::tl::TelegramApiBirthday;
/// use rustgram_birthdate::Birthdate;
///
/// let telegram_bd = TelegramApiBirthday {
///     flags: 1,
///     day: 15,
///     month: 6,
///     year: Some(1990),
/// };
/// let bd = Birthdate::from_telegram_api(&telegram_bd).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramApiBirthday {
    /// Flags field indicating which optional fields are present.
    /// Bit 0: year is present
    pub flags: u32,

    /// Day of month (1-31).
    pub day: i32,

    /// Month (1-12).
    pub month: i32,

    /// Year (1800-3000), present only when flag bit 0 is set.
    pub year: Option<i32>,
}

impl TelegramApiBirthday {
    /// Returns the constructor ID for this type.
    ///
    /// From TL schema: `birthday#6c8e1e06`
    #[inline]
    pub const fn constructor_id(&self) -> u32 {
        BIRTHDAY_MAGIC
    }

    /// Returns `true` if the year field is present.
    #[inline]
    pub fn has_year(&self) -> bool {
        (self.flags & YEAR_FLAG_MASK) != 0
    }

    /// Creates a new Telegram API birthday with year.
    #[inline]
    pub fn with_year(day: i32, month: i32, year: i32) -> Self {
        Self {
            flags: YEAR_FLAG_MASK,
            day,
            month,
            year: Some(year),
        }
    }

    /// Creates a new Telegram API birthday without year.
    #[inline]
    pub fn without_year(day: i32, month: i32) -> Self {
        Self {
            flags: 0,
            day,
            month,
            year: None,
        }
    }
}

impl From<Birthdate> for TelegramApiBirthday {
    fn from(bd: Birthdate) -> Self {
        bd.to_telegram_api()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor_id() {
        let bd = TelegramApiBirthday::with_year(15, 6, 1990);
        assert_eq!(bd.constructor_id(), BIRTHDAY_MAGIC);
        assert_eq!(bd.constructor_id(), 0x6c8e1e06);
    }

    #[test]
    fn test_has_year() {
        let bd_with = TelegramApiBirthday::with_year(15, 6, 1990);
        assert!(bd_with.has_year());

        let bd_without = TelegramApiBirthday::without_year(15, 6);
        assert!(!bd_without.has_year());

        let mut custom = TelegramApiBirthday {
            flags: 0,
            day: 1,
            month: 1,
            year: None,
        };
        assert!(!custom.has_year());

        custom.flags = YEAR_FLAG_MASK;
        custom.year = Some(2000);
        assert!(custom.has_year());
    }

    #[test]
    fn test_with_year() {
        let bd = TelegramApiBirthday::with_year(15, 6, 1990);
        assert_eq!(bd.flags, YEAR_FLAG_MASK);
        assert_eq!(bd.day, 15);
        assert_eq!(bd.month, 6);
        assert_eq!(bd.year, Some(1990));
    }

    #[test]
    fn test_without_year() {
        let bd = TelegramApiBirthday::without_year(15, 6);
        assert_eq!(bd.flags, 0);
        assert_eq!(bd.day, 15);
        assert_eq!(bd.month, 6);
        assert_eq!(bd.year, None);
    }

    #[test]
    fn test_from_birthdate() {
        let bd = Birthdate::new(15, 6, 1990).unwrap();
        let telegram_bd: TelegramApiBirthday = bd.into();

        assert_eq!(telegram_bd.flags, YEAR_FLAG_MASK);
        assert_eq!(telegram_bd.day, 15);
        assert_eq!(telegram_bd.month, 6);
        assert_eq!(telegram_bd.year, Some(1990));

        let bd2 = Birthdate::new(20, 12, 0).unwrap();
        let telegram_bd2: TelegramApiBirthday = bd2.into();

        assert_eq!(telegram_bd2.flags, 0);
        assert_eq!(telegram_bd2.day, 20);
        assert_eq!(telegram_bd2.month, 12);
        assert_eq!(telegram_bd2.year, None);
    }

    #[test]
    fn test_equality() {
        let bd1 = TelegramApiBirthday::with_year(15, 6, 1990);
        let bd2 = TelegramApiBirthday::with_year(15, 6, 1990);
        let bd3 = TelegramApiBirthday::with_year(15, 6, 1991);

        assert_eq!(bd1, bd2);
        assert_ne!(bd1, bd3);
    }
}
