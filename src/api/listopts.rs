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
    items.iter().map(|s| s.trim()).filter(|s| !s.is_empty()).map(str::to_string).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_matches_go() {
        let mut o = ListOptions { limit: 25, ..Default::default() };
        o.filter.insert("platform", GoSlice::from(vec!["IOS".into(), " ".into(), "MAC_OS".into()]));
        o.filter.insert("empty", GoSlice::from(vec!["".to_string()]));
        o.sort = vec!["-createdDate".to_string(), "".into()].into();
        o.include = vec!["app".to_string()].into();
        let mut q = Values::new();
        o.apply(&mut q);
        assert_eq!(q.encode(), "filter%5Bplatform%5D=IOS%2CMAC_OS&include=app&limit=25&sort=-createdDate");
    }
}
