//! Authentication manager
//!
//! Main authentication manager that handles all authentication flows.
//! Based on TDLib's `AuthManager` class.

use crate::{
    code::SentCode,
    email::{EmailSettings, EmailVerification, SentEmailCode},
    error::{AuthError, AuthResult},
    password::{InputCheckPasswordSrp, PasswordInfo},
    qr::{ImportQrCodeToken, QrCodeLogin},
    state::{AuthQueryType, AuthState, StateTransition},
    tl::Authorization,
    MAX_NAME_LENGTH,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Query ID for tracking authentication requests
pub type QueryId = u64;

/// Authentication manager
///
/// Main handler for Telegram authentication flows.
///
/// # Example
///
/// ```no_run
/// use rustgram_auth::{AuthManager, AuthState};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let manager = AuthManager::new(12345, "api_hash".to_string());
///
/// // Start phone authentication
/// manager.set_phone_number("+1234567890".to_string()).await?;
///
/// // Check state
/// assert!(matches!(manager.state().await, AuthState::WaitingForCode));
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct AuthManager {
    /// API ID from Telegram
    api_id: i32,

    /// API hash from Telegram
    api_hash: Arc<String>,

    /// Current authentication state
    state: Arc<RwLock<AuthState>>,

    /// Current query type
    query_type: Arc<RwLock<AuthQueryType>>,

    /// Active queries
    #[allow(dead_code)]
    queries: Arc<RwLock<HashMap<QueryId, AuthQueryType>>>,

    /// Phone number (for phone auth)
    phone_number: Arc<RwLock<Option<String>>>,

    /// Bot token (for bot auth)
    bot_token: Arc<RwLock<Option<String>>>,

    /// Sent code info
    sent_code: Arc<RwLock<Option<SentCode>>>,

    /// Email code info
    #[allow(dead_code)]
    email_code_info: Arc<RwLock<Option<SentEmailCode>>>,

    /// Password info
    password_info: Arc<RwLock<Option<PasswordInfo>>>,

    /// QR code login
    qr_login: Arc<RwLock<Option<QrCodeLogin>>>,

    /// Email settings
    email_settings: Arc<RwLock<EmailSettings>>,

    /// Is bot flag
    is_bot: Arc<RwLock<bool>>,

    /// Is authorized flag
    was_authorized: Arc<RwLock<bool>>,

    /// Query counter
    #[allow(dead_code)]
    query_counter: Arc<RwLock<u64>>,
}

impl AuthManager {
    /// Create a new authentication manager
    ///
    /// # Arguments
    ///
    /// * `api_id` - API ID from https://my.telegram.org
    /// * `api_hash` - API hash from https://my.telegram.org
    pub fn new(api_id: i32, api_hash: String) -> Self {
        Self {
            api_id,
            api_hash: Arc::new(api_hash),
            state: Arc::new(RwLock::new(AuthState::Idle)),
            query_type: Arc::new(RwLock::new(AuthQueryType::None)),
            queries: Arc::new(RwLock::new(HashMap::new())),
            phone_number: Arc::new(RwLock::new(None)),
            bot_token: Arc::new(RwLock::new(None)),
            sent_code: Arc::new(RwLock::new(None)),
            email_code_info: Arc::new(RwLock::new(None)),
            password_info: Arc::new(RwLock::new(None)),
            qr_login: Arc::new(RwLock::new(None)),
            email_settings: Arc::new(RwLock::new(EmailSettings::default())),
            is_bot: Arc::new(RwLock::new(false)),
            was_authorized: Arc::new(RwLock::new(false)),
            query_counter: Arc::new(RwLock::new(0)),
        }
    }

    /// Get current authentication state
    pub async fn state(&self) -> AuthState {
        *self.state.read().await
    }

    /// Check if currently authenticated
    pub async fn is_authorized(&self) -> bool {
        self.state.read().await.is_authenticated()
    }

    /// Check if was previously authenticated
    pub async fn was_authorized(&self) -> bool {
        *self.was_authorized.read().await
    }

    /// Check if this is a bot
    pub async fn is_bot(&self) -> bool {
        *self.is_bot.read().await
    }

    /// Get API ID
    pub const fn api_id(&self) -> i32 {
        self.api_id
    }

    /// Get API hash
    pub async fn api_hash(&self) -> String {
        self.api_hash.as_ref().clone()
    }

    /// Get current phone number
    pub async fn phone_number(&self) -> Option<String> {
        self.phone_number.read().await.clone()
    }

    /// Get sent code info
    pub async fn sent_code(&self) -> Option<SentCode> {
        self.sent_code.read().await.clone()
    }

    /// Get password info
    pub async fn password_info(&self) -> Option<PasswordInfo> {
        self.password_info.read().await.clone()
    }

    /// Get QR code login
    pub async fn qr_login(&self) -> Option<QrCodeLogin> {
        self.qr_login.read().await.clone()
    }

    /// Get email settings
    pub async fn email_settings(&self) -> EmailSettings {
        self.email_settings.read().await.clone()
    }

    /// Set phone number (start phone authentication)
    ///
    /// Begins the phone number authentication flow.
    pub async fn set_phone_number(&self, phone_number: String) -> AuthResult<()> {
        let mut state = self.state.write().await;

        if !matches!(*state, AuthState::Idle) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        // Validate phone number format
        self.validate_phone_number(&phone_number)?;

        *self.phone_number.write().await = Some(phone_number.clone());
        *self.is_bot.write().await = false;

        // Transition to waiting for code
        self.update_state(
            &mut state,
            AuthState::WaitingForPhone,
            AuthQueryType::SendCode,
        )
        .await?;

        Ok(())
    }

    /// Check bot token (bot authentication)
    ///
    /// Authenticates using a bot token.
    pub async fn check_bot_token(&self, bot_token: String) -> AuthResult<()> {
        let mut state = self.state.write().await;

        if !matches!(*state, AuthState::Idle) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        // Validate bot token format (format: NUMBER:RANDOM_STRING)
        self.validate_bot_token(&bot_token)?;

        *self.bot_token.write().await = Some(bot_token);
        *self.is_bot.write().await = true;

        // In real implementation, this would send auth.importBotToken query
        self.update_state(
            &mut state,
            AuthState::Authenticated,
            AuthQueryType::BotAuthentication,
        )
        .await?;

        Ok(())
    }

    /// Request QR code authentication
    ///
    /// Begins QR code authentication flow.
    pub async fn request_qr_code(&self, other_user_ids: Vec<i64>) -> AuthResult<QrCodeLogin> {
        let mut state = self.state.write().await;

        if !matches!(*state, AuthState::Idle) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        // In real implementation, this would send auth.exportLoginToken query
        let login_token = vec![1, 2, 3, 4]; // Placeholder
        let dc_id = 2;

        let mut qr_login = QrCodeLogin::new(login_token, dc_id, 300);
        for user_id in other_user_ids {
            qr_login = qr_login.add_other_user_id(user_id);
        }

        *self.qr_login.write().await = Some(qr_login.clone());

        self.update_state(
            &mut state,
            AuthState::WaitingForQrCode,
            AuthQueryType::RequestQrCode,
        )
        .await?;

        Ok(qr_login)
    }

    /// Import QR code token (scan another device's QR code)
    ///
    /// Imports a login token from another device's QR code.
    pub async fn import_qr_code(&self, token: ImportQrCodeToken) -> AuthResult<Authorization> {
        let mut state = self.state.write().await;

        if !matches!(*state, AuthState::Idle) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        if !token.is_valid() {
            return Err(AuthError::QrCodeError("Invalid token".to_string()));
        }

        // In real implementation, this would send auth.importLoginToken query
        self.update_state(
            &mut state,
            AuthState::Authenticated,
            AuthQueryType::ImportQrCode,
        )
        .await?;

        Ok(Authorization::new(false, None, false))
    }

    /// Check authentication code
    ///
    /// Submits the authentication code received via SMS/email/app.
    pub async fn check_code(&self, code: String) -> AuthResult<()> {
        let mut state = self.state.write().await;

        if !matches!(*state, AuthState::WaitingForCode) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        // Validate code
        if code.is_empty() || code.len() > 16 {
            return Err(AuthError::InvalidCode);
        }

        // Check if code expired
        if let Some(sent_code) = self.sent_code.read().await.as_ref() {
            if sent_code.is_expired() {
                return Err(AuthError::CodeExpired);
            }
        }

        // In real implementation, this would send auth.signIn query
        // For now, transition to authenticated
        self.update_state(&mut state, AuthState::Authenticated, AuthQueryType::SignIn)
            .await?;

        Ok(())
    }

    /// Check password (2FA)
    ///
    /// Submits the 2FA password.
    pub async fn check_password(&self, password: String) -> AuthResult<()> {
        let mut state = self.state.write().await;

        if !matches!(*state, AuthState::WaitingForPassword) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        if password.is_empty() {
            return Err(AuthError::InvalidPassword);
        }

        // In real implementation, this would compute SRP and send auth.checkPassword query
        self.update_state(
            &mut state,
            AuthState::Authenticated,
            AuthQueryType::CheckPassword,
        )
        .await?;

        Ok(())
    }

    /// Check password using SRP
    ///
    /// Submits the 2FA password using SRP verification.
    pub async fn check_password_srp(&self, srp_password: InputCheckPasswordSrp) -> AuthResult<()> {
        let mut state = self.state.write().await;

        if !matches!(*state, AuthState::WaitingForPassword) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        if !srp_password.is_valid() {
            return Err(AuthError::InvalidPassword);
        }

        // In real implementation, this would send auth.checkPassword query with SRP data
        self.update_state(
            &mut state,
            AuthState::Authenticated,
            AuthQueryType::CheckPassword,
        )
        .await?;

        Ok(())
    }

    /// Register user (sign up)
    ///
    /// Completes registration after receiving code.
    pub async fn register_user(
        &self,
        first_name: String,
        last_name: Option<String>,
    ) -> AuthResult<()> {
        let mut state = self.state.write().await;

        if !matches!(*state, AuthState::WaitingForRegistration) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        // Validate name lengths
        if first_name.len() > MAX_NAME_LENGTH {
            return Err(AuthError::InvalidPhone("First name too long".to_string()));
        }

        if let Some(last) = &last_name {
            if last.len() > MAX_NAME_LENGTH {
                return Err(AuthError::InvalidPhone("Last name too long".to_string()));
            }
        }

        // In real implementation, this would send auth.signUp query
        self.update_state(&mut state, AuthState::Authenticated, AuthQueryType::SignUp)
            .await?;

        Ok(())
    }

    /// Set email address
    ///
    /// Sets email address for authentication.
    pub async fn set_email_address(&self, email_address: String) -> AuthResult<()> {
        let mut state = self.state.write().await;

        if !matches!(*state, AuthState::WaitingForEmailAddress) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        // Validate email format
        self.validate_email_address(&email_address)?;

        // Update email settings
        let mut settings = self.email_settings.write().await;
        settings.email_address = Some(email_address);
        drop(settings);

        // In real implementation, this would send auth.sendCode query with email
        self.update_state(
            &mut state,
            AuthState::WaitingForEmailCode,
            AuthQueryType::SendEmailCode,
        )
        .await?;

        Ok(())
    }

    /// Check email code
    ///
    /// Verifies email authentication code.
    pub async fn check_email_code(&self, verification: EmailVerification) -> AuthResult<()> {
        let mut state = self.state.write().await;

        if !matches!(*state, AuthState::WaitingForEmailCode) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        if !verification.is_valid() {
            return Err(AuthError::EmailVerificationFailed(
                "Invalid verification".to_string(),
            ));
        }

        // In real implementation, this would verify email code
        self.update_state(
            &mut state,
            AuthState::WaitingForCode,
            AuthQueryType::VerifyEmailAddress,
        )
        .await?;

        Ok(())
    }

    /// Request password recovery
    ///
    /// Initiates password recovery flow.
    pub async fn request_password_recovery(&self) -> AuthResult<String> {
        let state = self.state.read().await;

        if !matches!(*state, AuthState::WaitingForPassword) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        // Check if recovery is available
        let password_info = self.password_info.read().await;
        let info = password_info
            .as_ref()
            .ok_or_else(|| AuthError::InternalError("Password info not available".to_string()))?;

        if !info.has_recovery() {
            return Err(AuthError::InternalError(
                "Password recovery not available".to_string(),
            ));
        }

        let email_pattern = info
            .recovery_email()
            .ok_or_else(|| AuthError::InternalError("Recovery email not set".to_string()))?
            .to_string();

        // In real implementation, this would send auth.requestPasswordRecovery query
        Ok(email_pattern)
    }

    /// Recover password
    ///
    /// Completes password recovery with recovery code.
    pub async fn recover_password(
        &self,
        recovery_code: String,
        new_password: String,
        _new_hint: String,
    ) -> AuthResult<()> {
        let mut state = self.state.write().await;

        if !matches!(*state, AuthState::WaitingForPassword) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        if recovery_code.is_empty() || new_password.is_empty() {
            return Err(AuthError::InvalidPassword);
        }

        // In real implementation, this would send auth.recoverPassword query
        self.update_state(
            &mut state,
            AuthState::Authenticated,
            AuthQueryType::RecoverPassword,
        )
        .await?;

        Ok(())
    }

    /// Log out
    ///
    /// Logs out from current session.
    pub async fn log_out(&self) -> AuthResult<()> {
        let mut state = self.state.write().await;

        if !matches!(*state, AuthState::Authenticated) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        // In real implementation, this would send auth.logOut query
        self.update_state(&mut state, AuthState::LoggingOut, AuthQueryType::LogOut)
            .await?;

        // Then transition to destroying keys
        self.update_state(&mut state, AuthState::DestroyingKeys, AuthQueryType::None)
            .await?;

        Ok(())
    }

    /// Delete account
    ///
    /// Deletes the Telegram account.
    pub async fn delete_account(
        &self,
        _reason: String,
        _password: Option<String>,
    ) -> AuthResult<()> {
        let state = self.state.read().await;

        if !matches!(*state, AuthState::Authenticated) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        // In real implementation, this would send account.deleteAccount query
        Err(AuthError::InternalError(
            "Account deletion not implemented".to_string(),
        ))
    }

    /// Resend authentication code
    ///
    /// Requests to resend the authentication code.
    pub async fn resend_code(&self) -> AuthResult<SentCode> {
        let state = self.state.read().await;

        if !matches!(*state, AuthState::WaitingForCode) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        // In real implementation, this would resend the code
        Err(AuthError::InternalError(
            "Resend code not implemented".to_string(),
        ))
    }

    /// Reset email address
    ///
    /// Resets the email address for authentication.
    pub async fn reset_email_address(&self) -> AuthResult<()> {
        let mut state = self.state.write().await;

        if !matches!(*state, AuthState::WaitingForEmailCode) {
            return Err(AuthError::InvalidState {
                state: state.to_string(),
            });
        }

        // In real implementation, this would reset email
        self.update_state(
            &mut state,
            AuthState::WaitingForEmailAddress,
            AuthQueryType::ResetEmailAddress,
        )
        .await?;

        Ok(())
    }

    /// Generate next query ID
    #[allow(dead_code)]
    async fn next_query_id(&self) -> QueryId {
        let mut counter = self.query_counter.write().await;
        *counter += 1;
        *counter
    }

    /// Update state with validation
    async fn update_state(
        &self,
        state: &mut AuthState,
        new_state: AuthState,
        query_type: AuthQueryType,
    ) -> AuthResult<()> {
        let transition = StateTransition::new(*state, new_state, query_type);

        if !transition.is_valid() && new_state != AuthState::Failed {
            return Err(AuthError::InternalError(format!(
                "Invalid state transition: {:?} -> {:?}",
                transition.from, transition.to
            )));
        }

        *self.query_type.write().await = query_type;
        *state = new_state;

        tracing::debug!("Auth state transition: {:?}", transition);

        Ok(())
    }

    /// Validate phone number format
    fn validate_phone_number(&self, phone: &str) -> AuthResult<()> {
        if phone.is_empty() {
            return Err(AuthError::InvalidPhone("Phone number is empty".to_string()));
        }

        // Basic validation: should start with + and have 8-15 digits
        if !phone.starts_with('+') {
            return Err(AuthError::InvalidPhone(
                "Phone number must start with +".to_string(),
            ));
        }

        let digits: Vec<char> = phone.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() < 8 || digits.len() > 15 {
            return Err(AuthError::InvalidPhone(
                "Phone number must have 8-15 digits".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate bot token format
    fn validate_bot_token(&self, token: &str) -> AuthResult<()> {
        // Bot token format: NUMBER:RANDOM_STRING (e.g., "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11")
        let parts: Vec<&str> = token.split(':').collect();
        if parts.len() != 2 {
            return Err(AuthError::InvalidBotToken(
                "Invalid bot token format".to_string(),
            ));
        }

        // Validate first part is a number
        parts[0]
            .parse::<u64>()
            .map_err(|_| AuthError::InvalidBotToken("Invalid bot token ID".to_string()))?;

        // Validate second part is not empty
        if parts[1].is_empty() || parts[1].len() < 35 {
            return Err(AuthError::InvalidBotToken(
                "Invalid bot token hash".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate email address format
    fn validate_email_address(&self, email: &str) -> AuthResult<()> {
        if email.is_empty() {
            return Err(AuthError::InvalidEmail("Email is empty".to_string()));
        }

        // Basic email validation
        if !email.contains('@') || !email.contains('.') {
            return Err(AuthError::InvalidEmail("Invalid email format".to_string()));
        }

        Ok(())
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new(0, String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_auth_manager_creation() {
        let manager = AuthManager::new(12345, "test_hash".to_string());
        assert_eq!(manager.api_id(), 12345);
        assert_eq!(manager.api_hash().await, "test_hash");
        assert_eq!(manager.state().await, AuthState::Idle);
    }

    #[tokio::test]
    async fn test_set_phone_number() {
        let manager = AuthManager::new(12345, "test_hash".to_string());
        manager
            .set_phone_number("+1234567890".to_string())
            .await
            .unwrap();

        assert_eq!(manager.state().await, AuthState::WaitingForPhone);
        assert_eq!(
            manager.phone_number().await,
            Some("+1234567890".to_string())
        );
    }

    #[tokio::test]
    async fn test_invalid_phone_number() {
        let manager = AuthManager::new(12345, "test_hash".to_string());

        let result = manager.set_phone_number("invalid".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_bot_token() {
        let manager = AuthManager::new(12345, "test_hash".to_string());
        manager
            .check_bot_token("123456:ABCDEF1234567890ghIkl-zyx57W2v1u123ew11".to_string())
            .await
            .unwrap();

        assert_eq!(manager.state().await, AuthState::Authenticated);
        assert!(manager.is_bot().await);
    }

    #[tokio::test]
    async fn test_invalid_bot_token() {
        let manager = AuthManager::new(12345, "test_hash".to_string());

        let result = manager.check_bot_token("invalid".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_state_transition() {
        let manager = AuthManager::new(12345, "test_hash".to_string());

        // Try to check code without setting phone number first
        let result = manager.check_code("12345".to_string()).await;
        assert!(result.is_err());
    }
}
