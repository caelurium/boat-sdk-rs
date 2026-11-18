use super::models::*;
use crate::{Client, Error};
use reqwest::Method;

impl Client {
    /// Get current Boat user.
    pub async fn me(&self) -> Result<MeResponse, Error> {
        Self::send(self.request(Method::GET, &["me"])?).await
    }

    /// List organizations.
    pub async fn list_organizations(&self) -> Result<OrgListResponse, Error> {
        Self::send(self.request(Method::GET, &["orgs"])?).await
    }

    /// Update organization settings. Renaming requires the owner; owners or admins may change sandbox
    /// visibility defaults.
    pub async fn update_organization(
        &self,
        org: &str,
        body: &UpdateOrganizationRequest,
    ) -> Result<UpdateOrganizationResponse, Error> {
        Self::send(self.request(Method::PATCH, &["orgs", org])?.json(body)).await
    }

    /// Set the active organization. Changes the default wallet across clients; send `org: null` to
    /// select personal billing.
    pub async fn set_active_organization(
        &self,
        body: &ActiveOrgUpdateRequest,
    ) -> Result<ActiveOrgResponse, Error> {
        Self::send(self.request(Method::PATCH, &["orgs", "active"])?.json(body)).await
    }
}
