use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignIn {
    pub email: String,
    pub password: String,
    pub device_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CryptoExchangeRequest {
    crypto_exchange_slug: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Devices {
    device_id: String,
    device_push_token: String,
    expo_push_token: String,
    is_notification_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct InvestorProfile {
    annual_salary: f64,
    has_clicked_opportunities: bool,
    income_tax_rate: f32,
    monthly_expenses: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OnboardingStep {
    state: String, // "done"
    step: String,  // "create_account" | "add_first_asset" | "connect_bank"
}

#[derive(Debug, Serialize, Deserialize)]
struct DisplayCurrency {
    code: String,
    correlation_id: String,
    id: u8,
    logo_url: String,
    name: String,
    symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FinancialProjectionParameters {
    duration: String,
    monthly_contribution: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UiConfiguration {
    display_currency: DisplayCurrency,
    display_language: String, // "fr"
    display_new_webapp_modal: bool,
    financial_projection_parameters: FinancialProjectionParameters,
    has_filled_invest_form: bool,
    has_seen_download_app_modal: bool,
    has_seen_free_referral_modal: bool,
    information_banner_seen_version: u8,
    is_crypto_currency_display_enabled: bool,
    konami_code: Option<String>,
    period_display_mode: String, // "1d"
    wealth_display_mode: String, // "gross"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    age: Option<u8>,
    #[serde(rename = "birthdate")]
    birth_date: Option<String>,
    created_at: String,
    crypto_exchange_requests: Vec<CryptoExchangeRequest>,
    devices: Vec<Devices>,
    email: String,
    finary_investor_status: Option<String>,
    #[serde(rename = "firstname")]
    first_name: String,
    has_bi_account: bool,
    investor_profile: InvestorProfile,
    is_active: bool,
    is_beta_tester: bool,
    is_crowdfunding_open: bool,
    is_crowdfunding_preregistered: bool,
    is_free_trial_available: bool,
    is_onboarding_completed: bool,
    is_otp_enabled: bool,
    last_asset_updated_at: String,
    last_user_sync_at: String,
    last_user_sync_triggered_at: String,
    #[serde(rename = "lastname")]
    last_name: String,
    net_monthly_salary: Option<f64>,
    newsletter_subscribed: bool,
    onboarding_steps: Vec<OnboardingStep>,
    push_token: Option<String>,
    referral_id: String,
    referral_status: Option<String>,
    registration_platform: String, // "ios"
    slug: String,
    subscription_cancel_at: Option<String>,
    subscription_current_period_end_at: Option<String>,
    subscription_price_without_tax: f32,
    subscription_renewal_period: String, // "month"
    subscription_status: String,         // "premium"
    ui_configuration: UiConfiguration,
    webapp_emails_subscribed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInResponse {
    access_token_expiry: u64,
    refresh_token_expiry: u64,
    user: User,
}
