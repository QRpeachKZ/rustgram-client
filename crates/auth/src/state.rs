//! Authentication state machine

/// Current state of the authentication flow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuthState {
    /// Initial state - no authentication started
    #[default]
    Idle,

    /// Waiting for phone number input
    WaitingForPhone,

    /// Waiting for authentication code
    WaitingForCode,

    /// Waiting for password (2FA)
    WaitingForPassword,

    /// Successfully authenticated
    Authenticated,

    /// Authentication failed
    Failed,
}

impl AuthState {
    /// Check if authentication is complete
    pub const fn is_authenticated(&self) -> bool {
        matches!(self, Self::Authenticated)
    }

    /// Check if waiting for user input
    pub const fn is_waiting_input(&self) -> bool {
        matches!(
            self,
            Self::WaitingForPhone | Self::WaitingForCode | Self::WaitingForPassword
        )
    }

    /// Check if state is terminal (no further transitions)
    pub const fn is_terminal(&self) -> bool {
        matches!(self, Self::Authenticated | Self::Failed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        assert_eq!(AuthState::default(), AuthState::Idle);
    }

    #[test]
    fn test_state_checks() {
        assert!(!AuthState::Idle.is_authenticated());
        assert!(AuthState::Authenticated.is_authenticated());
        assert!(AuthState::WaitingForCode.is_waiting_input());
        assert!(AuthState::Authenticated.is_terminal());
    }
}
