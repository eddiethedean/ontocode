//! Oasis / Protégé `catalog-v001.xml` IRI → local path redirects.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XmlCatalogError {
    #[error("io: {0}")]
    Io(String),
    #[error("parse: {0}")]
    Parse(String),
}

/// Parsed XML Catalog redirect table (Protégé `catalog-v001.xml` subset).
#[derive(Debug, Clone, Default)]
pub struct XmlCatalog {
    /// Absolute path of the catalog file (used to resolve relative uris).
    pub catalog_path: PathBuf,
    /// Exact IRI (`name`) → redirect URI string (may be relative).
    pub uri_entries: BTreeMap<String, String>,
    /// Prefix rewrite rules: start string → replacement prefix.
    pub rewrite_entries: Vec<(String, String)>,
}

impl XmlCatalog {
    /// Resolve an ontology import IRI to a local filesystem path when possible.
    pub fn resolve(&self, iri: &str) -> Option<PathBuf> {
        let redirect = self.resolve_uri(iri)?;
        Some(self.absolutize(&redirect))
    }

    /// Resolve to the redirect URI string (relative or absolute).
    pub fn resolve_uri(&self, iri: &str) -> Option<String> {
        if let Some(u) = self.uri_entries.get(iri) {
            return Some(u.clone());
        }
        // Longest rewriteURI prefix wins
        let mut best: Option<&(String, String)> = None;
        for rule in &self.rewrite_entries {
            if iri.starts_with(&rule.0) && best.as_ref().is_none_or(|b| rule.0.len() > b.0.len()) {
                best = Some(rule);
            }
        }
        best.map(|(start, replace)| format!("{}{}", replace, &iri[start.len()..]))
    }

    fn absolutize(&self, redirect: &str) -> PathBuf {
        let p = Path::new(redirect);
        if p.is_absolute() {
            return p.to_path_buf();
        }
        if let Some(rest) = redirect.strip_prefix("file://") {
            return PathBuf::from(rest);
        }
        let parent = self.catalog_path.parent().unwrap_or_else(|| Path::new("."));
        parent.join(redirect)
    }
}

/// Parse a Protégé/Oasis catalog XML document.
pub fn parse_xml_catalog(path: &Path, xml: &str) -> Result<XmlCatalog, XmlCatalogError> {
    let mut catalog = XmlCatalog { catalog_path: path.to_path_buf(), ..Default::default() };
    // Prefer xml:base on catalog/group when present
    let mut active_base: Option<String> = None;

    // Walk uri / rewriteURI tags with a lightweight scanner
    for (tag, attrs) in iter_empty_elements(xml) {
        let local = local_name(&tag);
        if local == "catalog" || local == "group" {
            if let Some(b) = attrs.get("xml:base").or_else(|| attrs.get("base")) {
                if !b.is_empty() {
                    active_base = Some(b.clone());
                }
            }
            continue;
        }
        if local == "uri" {
            let Some(name) = attrs.get("name").cloned() else {
                continue;
            };
            let Some(uri) = attrs.get("uri").cloned() else {
                continue;
            };
            let uri = apply_xml_base(&uri, active_base.as_deref());
            catalog.uri_entries.insert(name, uri);
        } else if local == "rewriteURI" {
            let Some(start) = attrs.get("uriStartString").cloned() else {
                continue;
            };
            let Some(rewrite) = attrs.get("rewritePrefix").cloned() else {
                continue;
            };
            catalog.rewrite_entries.push((start, rewrite));
        } else if local == "nextCatalog" {
            // Nested catalogs: optional; load relative catalog if present adjacent
            if let Some(next) = attrs.get("catalog") {
                let next_path = catalog.absolutize(next);
                if next_path.is_file() {
                    let text = std::fs::read_to_string(&next_path)
                        .map_err(|e| XmlCatalogError::Io(e.to_string()))?;
                    let nested = parse_xml_catalog(&next_path, &text)?;
                    for (k, v) in nested.uri_entries {
                        catalog.uri_entries.entry(k).or_insert(v);
                    }
                    catalog.rewrite_entries.extend(nested.rewrite_entries);
                }
            }
        }
    }
    Ok(catalog)
}

/// Load and parse a catalog file from disk.
pub fn load_xml_catalog(path: &Path) -> Result<XmlCatalog, XmlCatalogError> {
    let text = std::fs::read_to_string(path).map_err(|e| XmlCatalogError::Io(e.to_string()))?;
    parse_xml_catalog(path, &text)
}

/// Discover `catalog-v001.xml` / `catalog-*.xml` under `root` (non-recursive first, then depth 1).
pub fn discover_workspace_catalogs(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let candidates = [root.join("catalog-v001.xml"), root.join("catalog.xml")];
    for c in candidates {
        if c.is_file() {
            out.push(c);
        }
    }
    if let Ok(rd) = std::fs::read_dir(root) {
        for entry in rd.flatten() {
            let p = entry.path();
            if p.is_file() {
                if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("catalog-") && name.ends_with(".xml") && !out.contains(&p) {
                        out.push(p);
                    }
                }
            }
        }
    }
    out
}

/// Merge all discovered catalogs under a workspace root.
pub fn load_workspace_xml_catalogs(root: &Path) -> Result<XmlCatalog, XmlCatalogError> {
    let paths = discover_workspace_catalogs(root);
    let mut merged =
        XmlCatalog { catalog_path: root.join("catalog-v001.xml"), ..Default::default() };
    for path in paths {
        let cat = load_xml_catalog(&path)?;
        merged.catalog_path = path;
        for (k, v) in cat.uri_entries {
            merged.uri_entries.insert(k, v);
        }
        merged.rewrite_entries.extend(cat.rewrite_entries);
    }
    Ok(merged)
}

fn apply_xml_base(uri: &str, base: Option<&str>) -> String {
    let p = Path::new(uri);
    if p.is_absolute() || uri.contains("://") {
        return uri.to_string();
    }
    if let Some(b) = base {
        if b.is_empty() {
            return uri.to_string();
        }
        if b.ends_with('/') {
            format!("{b}{uri}")
        } else {
            format!("{b}/{uri}")
        }
    } else {
        uri.to_string()
    }
}

fn local_name(tag: &str) -> &str {
    tag.rsplit([':', '}']).next().unwrap_or(tag)
}

/// Yield (tag_name, attrs) for empty-element or start tags.
fn iter_empty_elements(xml: &str) -> Vec<(String, BTreeMap<String, String>)> {
    let mut out = Vec::new();
    let bytes = xml.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'<' {
            i += 1;
            continue;
        }
        if i + 1 < bytes.len()
            && (bytes[i + 1] == b'!' || bytes[i + 1] == b'?' || bytes[i + 1] == b'/')
        {
            // skip comments / decls / end tags — advance to '>'
            if let Some(rel) = xml[i..].find('>') {
                i += rel + 1;
            } else {
                break;
            }
            continue;
        }
        let start = i + 1;
        let Some(end_rel) = xml[start..].find('>') else {
            break;
        };
        let end = start + end_rel;
        let mut inner = &xml[start..end];
        inner = inner.trim_end_matches('/').trim();
        let (name, rest) = match inner.split_once(|c: char| c.is_whitespace()) {
            Some((n, r)) => (n, r),
            None => (inner, ""),
        };
        let attrs = parse_attrs(rest);
        out.push((name.to_string(), attrs));
        i = end + 1;
    }
    out
}

fn parse_attrs(s: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let mut rest = s;
    while let Some(eq) = rest.find('=') {
        let key = rest[..eq]
            .trim()
            .rsplit_once(char::is_whitespace)
            .map(|(_, k)| k)
            .unwrap_or(rest[..eq].trim());
        let after = rest[eq + 1..].trim_start();
        let (val, next) = if let Some(stripped) = after.strip_prefix('"') {
            if let Some(e) = stripped.find('"') {
                (&stripped[..e], &stripped[e + 1..])
            } else {
                break;
            }
        } else if let Some(stripped) = after.strip_prefix('\'') {
            if let Some(e) = stripped.find('\'') {
                (&stripped[..e], &stripped[e + 1..])
            } else {
                break;
            }
        } else {
            break;
        };
        if !key.is_empty() {
            map.insert(key.to_string(), val.to_string());
        }
        rest = next;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_uri_redirect() {
        let xml = r#"<?xml version="1.0"?>
<catalog xmlns="urn:oasis:names:tc:entity:xmlns:xml:catalog" prefer="public">
  <group id="Folder" prefer="public" xml:base="">
    <uri name="http://example.org/ont/pizza" uri="pizza.ttl"/>
  </group>
</catalog>"#;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("catalog-v001.xml");
        std::fs::write(&path, xml).unwrap();
        let cat = parse_xml_catalog(&path, xml).unwrap();
        assert_eq!(cat.resolve_uri("http://example.org/ont/pizza").as_deref(), Some("pizza.ttl"));
        let resolved = cat.resolve("http://example.org/ont/pizza").unwrap();
        assert_eq!(resolved, dir.path().join("pizza.ttl"));
    }

    #[test]
    fn xml_base_prefixes_relative_uri() {
        let xml = r#"<catalog xmlns="urn:oasis:names:tc:entity:xmlns:xml:catalog">
  <group xml:base="lib/">
    <uri name="http://example.org/a" uri="a.ttl"/>
  </group>
</catalog>"#;
        let path = Path::new("/tmp/catalog-v001.xml");
        let cat = parse_xml_catalog(path, xml).unwrap();
        assert_eq!(cat.resolve_uri("http://example.org/a").as_deref(), Some("lib/a.ttl"));
    }
}
