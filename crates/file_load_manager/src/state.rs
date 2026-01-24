/// Load state for file operations.
///
/// Matches TDLib's node state classification for tracking file load progress.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum LoadState {
    /// Queued, waiting for resources.
    #[default]
    Pending = 0,
    /// Resources allocated, processing.
    Active = 1,
    /// Finished successfully.
    Complete = 2,
}

impl PartialEq for LoadState {
    fn eq(&self, other: &Self) -> bool {
        (*self as u8) == (*other as u8)
    }
}

impl Eq for LoadState {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let state = LoadState::default();
        assert_eq!(state, LoadState::Pending);
    }

    #[test]
    fn test_state_equality() {
        assert_eq!(LoadState::Pending, LoadState::Pending);
        assert_eq!(LoadState::Active, LoadState::Active);
        assert_eq!(LoadState::Complete, LoadState::Complete);
    }

    #[test]
    fn test_state_inequality() {
        assert_ne!(LoadState::Pending, LoadState::Active);
        assert_ne!(LoadState::Active, LoadState::Complete);
        assert_ne!(LoadState::Pending, LoadState::Complete);
    }

    #[test]
    fn test_state_copy() {
        let state = LoadState::Active;
        let copied = state;
        assert_eq!(state, LoadState::Active);
        assert_eq!(copied, LoadState::Active);
    }
}
