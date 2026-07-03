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
        // Skip /* block comments */
        if rest.starts_with("/*") {
            rest = rest.split_once("*/").map_or("", |(_, tail)| tail);
            continue;
        }
        let upper = rest.to_ascii_uppercase();
        // Skip prologue declarations (PREFIX / BASE), including same-line forms.
        if upper.starts_with("PREFIX") || upper.starts_with("BASE") {
            rest = skip_sparql_prologue_decl(rest);
            continue;
        }
        return UPDATE_VERBS.iter().any(|verb| {
            upper.starts_with(verb)
                && upper
                    .as_bytes()
                    .get(verb.len())
                    .is_none_or(|b| b.is_ascii_whitespace() || *b == b'{')
        });
    }
}

/// Advance past a PREFIX or BASE declaration (line- or token-oriented).
fn skip_sparql_prologue_decl(rest: &str) -> &str {
    let upper = rest.to_ascii_uppercase();
    if upper.starts_with("PREFIX") {
        // PREFIX name: <iri>  — skip through the closing '>' if present, else end of line.
        if let Some(gt) = rest.find('>') {
            return rest[gt + 1..].trim_start();
        }
        return rest.split_once('\n').map_or("", |(_, tail)| tail);
    }
    if upper.starts_with("BASE") {
        if let Some(gt) = rest.find('>') {
            return rest[gt + 1..].trim_start();
        }
        return rest.split_once('\n').map_or("", |(_, tail)| tail);
    }
    rest
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

    #[test]
    fn rejects_insert_after_base() {
        let q = "BASE <http://example.org/>\nINSERT DATA { <s> <p> <o> }";
        assert!(is_sparql_update(q));
    }

    #[test]
    fn rejects_insert_on_same_line_as_prefix() {
        let q = "PREFIX ex: <http://example.org/> INSERT DATA { ex:s ex:p ex:o }";
        assert!(is_sparql_update(q));
    }

    #[test]
    fn allows_select_after_base() {
        let q = "BASE <http://example.org/>\nSELECT ?s WHERE { ?s ?p ?o }";
        assert!(!is_sparql_update(q));
    }
}
