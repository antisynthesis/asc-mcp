// App Store Connect API endpoints for the "apps" area.

#[allow(unused_imports)]
use crate::api::{Client, Error, ListOptions, Result, decode, model::*};
#[allow(unused_imports)]
use crate::wire::{
    Base64Bytes, JsonMap, JsonSlice, RawMessage, Timestamp, json,
    url::{self, Values},
};
#[allow(unused_imports)]
use serde_json::Value;

impl Client {
    /// Returns a single app by ID.
    pub async fn get_app(&self, app_id: &str) -> Result<AppResponse> {
        let data = self
            .get(&format!("/v1/apps/{}", url::path_escape(app_id)), None)
            .await?;
        decode(&data)
    }

    /// Returns versions for an app.
    pub async fn get_app_versions(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppStoreVersionsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/apps/{}/appStoreVersions", url::path_escape(app_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns a list of apps.
    pub async fn list_apps(&self, opts: Option<&ListOptions>) -> Result<AppsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get("/v1/apps", Some(&query)).await?;
        decode(&data)
    }
}
