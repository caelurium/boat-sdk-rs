/// Capacity Limits in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapacityLimits {
    /// Boat `activeSandboxes` field.
    pub active_sandboxes: Option<i64>,
    /// Boat `creationRatePerMinute` field.
    pub creation_rate_per_minute: Option<i64>,
    /// Boat `creationRequestsPerHour` field.
    pub creation_requests_per_hour: Option<i64>,
    /// Boat `creationRequestsPerDay` field.
    pub creation_requests_per_day: Option<i64>,
}

/// Start Limits in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartLimits {
    /// Boat `perMinute` field.
    pub per_minute: Option<i64>,
    /// Boat `perHour` field.
    pub per_hour: Option<i64>,
    /// Boat `perDay` field.
    pub per_day: Option<i64>,
}

/// Wallet in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    /// Money in the wallet.
    pub balance_dollars: Option<f64>,
}

/// Plan Included in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanIncluded {
    /// VM time included this period, in hours of a `default` machine.
    pub vm_hours: Option<f64>,
    /// Included VM time left this period, in hours of a `default` machine.
    pub vm_hours_remaining: Option<f64>,
    /// When the included amounts reset.
    pub period_end: Option<String>,
}

/// Overage Prices Start Brackets Item in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OveragePricesStartBracketsItem {
    /// The bracket starts above this multiple of the limit.
    pub above_multiple: Option<f64>,
    /// The bracket ends at this multiple of the limit (included).
    pub up_to_multiple: Option<f64>,
    /// Dollars per start in this bracket.
    pub dollars: Option<f64>,
}

/// Overage Prices in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OveragePrices {
    /// Dollars per hour of a `default` machine.
    pub vm_hour_dollars: Option<f64>,
    /// Dollars per day for each saved snapshot above the included count.
    pub saved_snapshot_day_dollars: Option<f64>,
    /// Price of a start above a start limit, by how far past the limit it falls, in each window (minute,
    /// hour, day).
    pub start_brackets: Option<Vec<OveragePricesStartBracketsItem>>,
    /// Extra starts stop at this multiple of the hour and day limits (the last bracket's upToMultiple).
    pub start_limit_multiplier: Option<f64>,
    /// Extra starts stop at this multiple of the minute limit.
    pub start_minute_limit_multiplier: Option<f64>,
}

/// Starts in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Starts {
    /// Boat `unlimited` field.
    pub unlimited: Option<bool>,
    /// Boat `minute` field.
    pub minute: Option<StartWindowUsage>,
    /// Boat `hour` field.
    pub hour: Option<StartWindowUsage>,
    /// Boat `day` field.
    pub day: Option<StartWindowUsage>,
}

/// Sandbox Plan Tier in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxPlanTier {
    /// Boat `key` field.
    pub key: Option<String>,
    /// Boat `dollars` field.
    pub dollars: Option<f64>,
    /// Boat `displayPrice` field.
    pub display_price: Option<String>,
    /// Boat `maxActiveSandboxes` field.
    pub max_active_sandboxes: Option<i64>,
    /// Boat `startsPerMinute` field.
    pub starts_per_minute: Option<i64>,
    /// Boat `startsPerHour` field.
    pub starts_per_hour: Option<i64>,
    /// Boat `startsPerDay` field.
    pub starts_per_day: Option<i64>,
    /// Machine time included each month.
    pub included_seconds: Option<i64>,
    /// Whether this tier can be bought right now.
    pub purchasable: Option<bool>,
}

/// Machine Availability in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MachineAvailability {
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub machine_type: String,
    /// Boat `available` field.
    pub available: bool,
    /// Why the type is blocked, for example `not during the free trial; the first payment unlocks it`.
    pub reason: Option<String>,
}

/// Gift Limit in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftLimit {
    /// Boat `maxActiveSandboxes` field.
    pub max_active_sandboxes: Option<i64>,
    /// Boat `maxTtlSeconds` field.
    pub max_ttl_seconds: Option<i64>,
    /// The caps in one readable line.
    pub line: Option<String>,
    /// The same text shown when a cap is hit.
    pub message: Option<String>,
}

/// Limits Response in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitsResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Stable success envelope discriminator added by v1.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `accessTier` field.
    pub access_tier: Option<String>,
    /// Boat `blockedReason` field.
    pub blocked_reason: Option<String>,
    /// Boat `currentLimits` field.
    pub current_limits: Option<CapacityLimits>,
    /// Boat `standardLimits` field.
    pub standard_limits: Option<CapacityLimits>,
    /// Boat `trialLimits` field.
    pub trial_limits: Option<CapacityLimits>,
    /// Boat `upgradeEffects` field.
    pub upgrade_effects: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Whether the authenticated account can create or operate sandboxes right now.
    pub can_start: bool,
    /// Boat `checkoutRequired` field.
    pub checkout_required: Option<bool>,
    /// Boat `startBlockedReason` field.
    pub start_blocked_reason: Option<String>,
    /// Boat `contactMessage` field.
    pub contact_message: Option<String>,
    /// Boat `activeSandboxes` field.
    pub active_sandboxes: i64,
    /// Boat `activeStates` field.
    pub active_states: Option<Vec<String>>,
    /// Boat `maxActiveSandboxes` field.
    pub max_active_sandboxes: i64,
    /// Boat `maxCreationRequestsPerMinute` field.
    pub max_creation_requests_per_minute: Option<i64>,
    /// Boat `maxCreationRequestsPerDay` field.
    pub max_creation_requests_per_day: Option<i64>,
    /// Plan caps for machine starts.
    pub start_limits: Option<StartLimits>,
    /// The one wallet of this account or organization, in US dollars.
    pub wallet: Option<Wallet>,
    /// What the plan includes this billing period.
    pub plan_included: Option<PlanIncluded>,
    /// What the wallet pays past the plan.
    pub overage_prices: Option<OveragePrices>,
    /// What the next start costs now.
    pub next_start_dollars: Option<f64>,
    /// Start ceilings with extra starts: the hour and day limits times
    /// `overagePrices.startLimitMultiplier`, and the minute limit times
    /// `overagePrices.startMinuteLimitMultiplier`.
    pub start_limits_with_wallet: Option<StartLimits>,
    /// Remaining machine starts in the rolling minute, hour and day windows.
    pub starts: Option<Starts>,
    /// Deprecated.
    pub credit_balance_hours: Option<f64>,
    /// Deprecated.
    pub pack_balance_hours: Option<f64>,
    /// Deprecated.
    pub pack_balance_dollars: Option<f64>,
    /// Boat `hasPaymentHistory` field.
    pub has_payment_history: Option<bool>,
    /// Boat `subscriptionQuotaSeconds` field.
    pub subscription_quota_seconds: Option<i64>,
    /// Boat `subscriptionRemainingSeconds` field.
    pub subscription_remaining_seconds: Option<i64>,
    /// Deprecated.
    pub pack_balance_seconds: Option<i64>,
    /// Deprecated.
    pub credit_purchased_seconds: Option<i64>,
    /// Boat `creditUsedSeconds` field.
    pub credit_used_seconds: Option<i64>,
    /// Boat `liveUsageSeconds` field.
    pub live_usage_seconds: Option<i64>,
    /// Boat `creditSecondsPerDollar` field.
    pub credit_seconds_per_dollar: Option<i64>,
    /// Current plan key, or `trial` before any payment.
    pub sandbox_plan_key: Option<String>,
    /// Monthly price of the current plan in dollars.
    pub sandbox_plan_dollars: Option<f64>,
    /// Every plan tier and its limits.
    pub sandbox_plan_tiers: Option<Vec<SandboxPlanTier>>,
    /// Boat `accountPlan` field.
    pub account_plan: Option<String>,
    /// `service` for service accounts, otherwise null.
    pub plan: Option<String>,
    /// `Service` for service accounts, otherwise null.
    pub plan_name: Option<String>,
    /// Boat `serviceAccount` field.
    pub service_account: Option<bool>,
    /// True when the account has no machine-time or start limits.
    pub unlimited: Option<bool>,
    /// True when an organization has an active per-seat plan.
    pub has_seat_plan: Option<bool>,
    /// Machine time used in the last 24 hours, in seconds.
    pub last24h_usage_seconds: Option<i64>,
    /// Lifetime VM-time allowance for new sandboxes while the account has not paid, a plan trial included.
    pub trial_compute_cap_seconds: Option<i64>,
    /// VM time already used against trialComputeCapSeconds.
    pub trial_compute_used_seconds: Option<i64>,
    /// The trial limits in one readable line.
    pub trial_line: Option<String>,
    /// Each sandbox type, and whether this account can create it now.
    pub machine_types: Option<Vec<MachineAvailability>>,
    /// Fair-use caps on a gifted account, or null when there are none.
    pub gift_limit: Option<GiftLimit>,
    /// Account access state returned by the current backend.
    pub billing_status: String,
    /// Boat `subscriptionStatus` field.
    pub subscription_status: Option<String>,
    /// Boat `subscriptionCancelAtPeriodEnd` field.
    pub subscription_cancel_at_period_end: Option<bool>,
    /// Boat `hasSubscription` field.
    pub has_subscription: Option<bool>,
    /// Boat `subscriptionTrialEndsAt` field.
    pub subscription_trial_ends_at: Option<String>,
    /// Boat `subscriptionCurrentPeriodEnd` field.
    pub subscription_current_period_end: Option<String>,
    /// Deprecated.
    pub credit_balance_seconds: Option<i64>,
    /// Present when limits were read for a team wallet (`?teamId=`, `?org=`, or `X-Boat-Org`).
    pub team_id: Option<String>,
    /// Caller's role on that team when `teamId` is present.
    pub team_role: Option<String>,
}

/// Start Window Usage in the Boat API.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartWindowUsage {
    /// Boat `limit` field.
    pub limit: Option<i64>,
    /// Boat `used` field.
    pub used: Option<i64>,
    /// Boat `remaining` field.
    pub remaining: Option<i64>,
}
