//! Request and response types for Boat account.

/// Account User.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountUser {
    /// Your account id.
    pub id: Option<String>,
    /// Boat `login` field.
    pub login: Option<String>,
    /// Boat `email` field.
    pub email: Option<String>,
    /// Boat `displayName` field.
    pub display_name: Option<String>,
    /// The login method the account was created with.
    pub connection_method: Option<String>,
    /// Every login method linked to the account.
    pub connection_methods: Option<Vec<String>>,
    /// Boat `accountPlan` field.
    pub account_plan: Option<String>,
    /// Boat `serviceAccount` field.
    pub service_account: Option<bool>,
    /// Boat `accountStatus` field.
    pub account_status: Option<String>,
    /// Boat `suspended` field.
    pub suspended: Option<bool>,
    /// Boat `suspendedAt` field.
    pub suspended_at: Option<String>,
    /// Boat `suspendedReason` field.
    pub suspended_reason: Option<String>,
    /// Boat `closed` field.
    pub closed: Option<bool>,
    /// Boat `closedAt` field.
    pub closed_at: Option<String>,
    /// When a closed account's data is scheduled for deletion.
    pub purge_at: Option<String>,
    /// When a closed account's data was deleted.
    pub purged_at: Option<String>,
    /// Address to contact about a suspended or closed account.
    pub closure_contact_email: Option<String>,
    /// Whether archived sandbox data is configured for deletion instead of retention.
    pub zero_data_retention: Option<bool>,
    /// Boat `zeroDataRetentionEnabledAt` field.
    pub zero_data_retention_enabled_at: Option<String>,
}

/// Active Org Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrgResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `active` field.
    pub active: ActiveOrgResponseActive,
}

/// Active Org Response Active.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrgResponseActive {
    /// Boat `id` field.
    pub id: String,
    /// Boat `name` field.
    pub name: String,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
}

/// Active Org Update Request.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrgUpdateRequest {
    /// Organization id or name you belong to; `personal`, your own id or `null` for personal.
    pub org: Option<String>,
}

/// Me Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `user` field.
    pub user: AccountUser,
}

/// Org.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Org {
    /// Your account id for the personal row, `team_…` for an organization.
    pub id: String,
    /// Accepted wherever an org is passed, like the id.
    pub name: String,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Boat `role` field.
    pub role: String,
    /// Boat `memberCount` field.
    pub member_count: i64,
    /// Stripe status of the organization's plan; absent on the personal row.
    pub subscription_status: Option<String>,
    /// The wallet new sandboxes bill when a request names none.
    pub active: bool,
}

/// Org List Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrgListResponse {
    /// Boat `ok` field.
    pub ok: bool,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Whether this account ever set its active wallet.
    pub active_wallet_chosen: bool,
    /// Boat `orgs` field.
    pub orgs: Vec<Org>,
}

/// Resource Access.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceAccess {
    /// Boat `restricted` field.
    pub restricted: bool,
    /// Member ids.
    pub members: Vec<String>,
}

/// Resource Access Update.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceAccessUpdate {
    /// False opens it to every member.
    pub restricted: bool,
    /// Member ids or emails of active members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

/// Update Organization Request.
#[derive(Clone, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrganizationRequest {
    /// New name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// True = new sandboxes start private.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sandboxes_private: Option<bool>,
}

/// Update Organization Response.
#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrganizationResponse {
    /// Boat `ok` field.
    pub ok: Option<bool>,
    /// Boat `type` field.
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// Boat `success` field.
    pub success: Option<bool>,
    /// Boat `name` field.
    pub name: Option<String>,
    /// Boat `newSandboxesPrivate` field.
    pub new_sandboxes_private: Option<bool>,
}
