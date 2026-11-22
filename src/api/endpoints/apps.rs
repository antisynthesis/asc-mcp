// App Store Connect API endpoints for the "apps" area.
//
// Signatures were generated from the Go declarations; bodies are ported by
// hand from the Go file named in each method's todo!/comment and must
// preserve the exact request path, query and body.

#[allow(unused_imports)]
use crate::api::{Client, Error, ListOptions, Result, decode, model::*};
#[allow(unused_imports)]
use crate::go::{
    GoBytes, GoMap, GoSlice, GoTime, RawMessage, json,
    url::{self, Values},
};
#[allow(unused_imports)]
use serde_json::Value;

impl Client {
    /// GetApp returns a single app by ID.
    pub async fn get_app(&self, app_id: &str) -> Result<AppResponse> {
        let data = self
            .get(&format!("/v1/apps/{}", url::path_escape(app_id)), None)
            .await?;
        decode(&data)
    }

    /// GetAppVersions returns versions for an app.
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

    /// ListApps returns a list of apps.
    pub async fn list_apps(&self, opts: Option<&ListOptions>) -> Result<AppsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get("/v1/apps", Some(&query)).await?;
        decode(&data)
    }
}
