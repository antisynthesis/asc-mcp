//! Query option helpers for App Store Connect list endpoints.

use crate::go::url::Values;
use crate::go::{GoMap, GoSlice};

/// The JSON:API query parameters every list endpoint accepts:
///
/// * `limit`  → `?limit=N`
/// * `filter` → `?filter[<k>]=v1,v2,...`
/// * `sort`   → `?sort=field1,-field2`
/// * `fields` → `?fields[<k>]=f1,f2` (sparse fieldsets)
/// * `include`→ `?include=rel1,rel2`
///
/// All fields are optional; the default produces an empty query.
#[derive(Debug, Clone, Default)]
pub struct ListOptions {
    /// Page size.
    pub limit: i64,
    /// `filter[...]` parameters.
    pub filter: GoMap<GoSlice<String>>,
    /// Sort fields.
    pub sort: GoSlice<String>,
    /// Sparse fieldsets.
    pub fields: GoMap<GoSlice<String>>,
    /// Related resources to include.
    pub include: GoSlice<String>,
}

impl ListOptions {
    /// Writes the options into `q`. Keys with no non-empty values are
    /// skipped so a malformed `filter[x]=` is never emitted; map keys are
    /// written in sorted order.
    pub fn apply(&self, q: &mut Values) {
        if self.limit > 0 {
            q.set("limit", self.limit.to_string());
        }
        write_bracketed_map(q, "filter", &self.filter);
        let sort = strip_empty(&self.sort).join(",");
        if !sort.is_empty() {
            q.set("sort", sort);
        }
        write_bracketed_map(q, "fields", &self.fields);
        let include = strip_empty(&self.include).join(",");
        if !include.is_empty() {
            q.set("include", include);
        }
    }
}

fn write_bracketed_map(q: &mut Values, prefix: &str, m: &GoMap<GoSlice<String>>) {
    for (k, vs) in m.iter() {
        let vs = strip_empty(vs);
        if vs.is_empty() {
            continue;
        }
        q.set(format!("{prefix}[{k}]"), vs.join(","));
    }
}

fn strip_empty(items: &[String]) -> Vec<String> {
    items
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_matches_go() {
        let mut o = ListOptions {
            limit: 25,
            ..Default::default()
        };
        o.filter.insert(
            "platform",
            GoSlice::from(vec!["IOS".into(), " ".into(), "MAC_OS".into()]),
        );
        o.filter
            .insert("empty", GoSlice::from(vec!["".to_string()]));
        o.sort = vec!["-createdDate".to_string(), "".into()].into();
        o.include = vec!["app".to_string()].into();
        let mut q = Values::new();
        o.apply(&mut q);
        assert_eq!(
            q.encode(),
            "filter%5Bplatform%5D=IOS%2CMAC_OS&include=app&limit=25&sort=-createdDate"
        );
    }

    fn strings(v: &[&str]) -> GoSlice<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    // Go: TestListOptions_Apply_Empty (Go's nil receiver is Option::None
    // at the call sites, which skip apply).
    #[test]
    fn apply_empty() {
        let mut q = Values::new();
        ListOptions::default().apply(&mut q);
        assert!(
            q.is_empty(),
            "empty opts should write nothing, got {}",
            q.encode()
        );
    }

    // Go: TestListOptions_Apply_AllFields
    #[test]
    fn apply_all_fields() {
        let mut o = ListOptions {
            limit: 25,
            ..Default::default()
        };
        o.filter.insert("platform", strings(&["IOS", "MAC_OS"]));
        o.filter
            .insert("appStoreState", strings(&["READY_FOR_SALE"]));
        o.sort = strings(&["-uploadedDate", "name"]);
        o.fields.insert("apps", strings(&["name", "bundleId"]));
        o.include = strings(&["builds", "appStoreVersions"]);
        let mut q = Values::new();
        o.apply(&mut q);
        assert_eq!(q.get("limit"), "25");
        assert_eq!(q.get("filter[platform]"), "IOS,MAC_OS");
        assert_eq!(q.get("filter[appStoreState]"), "READY_FOR_SALE");
        assert_eq!(q.get("sort"), "-uploadedDate,name");
        assert_eq!(q.get("fields[apps]"), "name,bundleId");
        assert_eq!(q.get("include"), "builds,appStoreVersions");
    }

    // Go: TestListOptions_StripsEmptyValues
    #[test]
    fn strips_empty_values() {
        let mut o = ListOptions::default();
        o.filter.insert("x", strings(&["", "  ", "real"]));
        o.filter.insert("y", strings(&[""]));
        o.sort = strings(&["", " ", "field"]);
        o.include = strings(&[""]);
        let mut q = Values::new();
        o.apply(&mut q);
        assert_eq!(q.get("filter[x]"), "real");
        assert!(
            !q.has("filter[y]"),
            "filter[y] should be absent when all values empty"
        );
        assert_eq!(q.get("sort"), "field");
        assert!(
            !q.has("include"),
            "include should be absent when all values empty"
        );
    }
}
