const UPDATE_VERBS: &[&str] =
    &["INSERT", "DELETE", "LOAD", "CLEAR", "CREATE", "DROP", "MOVE", "COPY", "ADD"];

/// Returns true when the query appears to be a SPARQL update (not read-only).
pub fn is_sparql_update(sparql: &str) -> bool {
    let mut rest = sparql;
    loop {
        rest = rest.trim_start();
        if rest.is_empty() {
            return false;
        }
        if let Some(after_hash) = rest.strip_prefix('#') {
            rest = after_hash.split_once('\n').map_or("", |(_, tail)| tail);
            continue;
        }
        if rest.to_ascii_uppercase().starts_with("PREFIX") {
            rest = rest.split_once('\n').map_or("", |(_, tail)| tail);
            continue;
        }
        let upper = rest.to_ascii_uppercase();
        return UPDATE_VERBS.iter().any(|verb| upper.starts_with(verb));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_insert_after_prefix() {
        let q = "PREFIX ex: <http://example.org/>\nINSERT DATA { ex:s ex:p ex:o }";
        assert!(is_sparql_update(q));
    }

    #[test]
    fn allows_select_after_prefix() {
        let q = "PREFIX ex: <http://example.org/>\nSELECT ?s WHERE { ?s ?p ?o }";
        assert!(!is_sparql_update(q));
    }

    #[test]
    fn rejects_insert_after_comment() {
        let q = "# setup\nINSERT DATA { }";
        assert!(is_sparql_update(q));
    }
}
