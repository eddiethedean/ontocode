use crate::error::{OwlError, Result};
use horned_owl::model::{Build, ClassExpression, ObjectPropertyExpression, RcStr};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Write as _;

#[derive(Debug, Clone, Serialize)]
pub struct ManchesterDiagnostic {
    pub message: String,
    pub offset: usize,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct ManchesterParseOutput {
    pub normalized: String,
    pub expression: ClassExpression<RcStr>,
    pub tree: serde_json::Value,
    pub diagnostics: Vec<ManchesterDiagnostic>,
}

pub fn parse_class_expression(
    input: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<ManchesterParseOutput> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(OwlError::ManchesterInvalid("empty expression".to_string()));
    }

    let tokens = tokenize(trimmed).map_err(OwlError::ManchesterInvalid)?;
    let mut parser = ManchesterParser { tokens, pos: 0 };
    let ast = parser.parse_expression().map_err(OwlError::ManchesterInvalid)?;
    if !parser.is_at_end() {
        return Err(OwlError::ManchesterInvalid(format!("unexpected token: {:?}", parser.peek())));
    }

    let build = Build::default();
    let expression = ast_to_class_expression(&ast, &build, namespaces)?;
    let normalized = class_expression_to_manchester(&expression, namespaces);
    let tree = expression_tree_json(&expression, namespaces);
    Ok(ManchesterParseOutput { normalized, expression, tree, diagnostics: Vec::new() })
}

pub fn class_expression_to_turtle_fragment(
    expr: &ClassExpression<RcStr>,
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<String> {
    let value = class_expression_to_turtle_value(expr, namespaces, 0)?;
    Ok(format!("    {predicate} {value} ;\n"))
}

pub fn class_expression_to_manchester(
    expr: &ClassExpression<RcStr>,
    namespaces: &BTreeMap<String, String>,
) -> String {
    match expr {
        ClassExpression::Class(c) => iri_to_manchester_term(&c.to_string(), namespaces),
        ClassExpression::ObjectIntersectionOf(v) => {
            let parts: Vec<String> =
                v.iter().map(|e| class_expression_to_manchester(e, namespaces)).collect();
            if parts.len() == 1 {
                parts[0].clone()
            } else {
                parts.join(" and ")
            }
        }
        ClassExpression::ObjectUnionOf(v) => {
            let parts: Vec<String> =
                v.iter().map(|e| class_expression_to_manchester(e, namespaces)).collect();
            if parts.len() == 1 {
                parts[0].clone()
            } else {
                format!("({})", parts.join(" or "))
            }
        }
        ClassExpression::ObjectSomeValuesFrom { ope, bce } => {
            let prop = ope_to_iri(ope);
            let filler = class_expression_to_manchester(bce, namespaces);
            format!("{} some {}", iri_to_manchester_term(&prop, namespaces), filler)
        }
        ClassExpression::ObjectAllValuesFrom { ope, bce } => {
            let prop = ope_to_iri(ope);
            let filler = class_expression_to_manchester(bce, namespaces);
            format!("{} only {}", iri_to_manchester_term(&prop, namespaces), filler)
        }
        ClassExpression::ObjectMinCardinality { n, ope, bce } => {
            let prop = iri_to_manchester_term(&ope_to_iri(ope), namespaces);
            let filler = class_expression_to_manchester(bce, namespaces);
            format!("min {n} {prop} some {filler}")
        }
        ClassExpression::ObjectMaxCardinality { n, ope, bce } => {
            let prop = iri_to_manchester_term(&ope_to_iri(ope), namespaces);
            let filler = class_expression_to_manchester(bce, namespaces);
            format!("max {n} {prop} some {filler}")
        }
        ClassExpression::ObjectExactCardinality { n, ope, bce } => {
            let prop = iri_to_manchester_term(&ope_to_iri(ope), namespaces);
            let filler = class_expression_to_manchester(bce, namespaces);
            format!("exactly {n} {prop} some {filler}")
        }
        other => format!("({other:?})"),
    }
}

pub fn expression_tree_json(
    expr: &ClassExpression<RcStr>,
    namespaces: &BTreeMap<String, String>,
) -> serde_json::Value {
    match expr {
        ClassExpression::Class(c) => serde_json::json!({
            "kind": "Class",
            "label": iri_to_manchester_term(&c.to_string(), namespaces),
        }),
        ClassExpression::ObjectIntersectionOf(v) => serde_json::json!({
            "kind": "ObjectIntersectionOf",
            "children": v.iter().map(|e| expression_tree_json(e, namespaces)).collect::<Vec<_>>(),
        }),
        ClassExpression::ObjectUnionOf(v) => serde_json::json!({
            "kind": "ObjectUnionOf",
            "children": v.iter().map(|e| expression_tree_json(e, namespaces)).collect::<Vec<_>>(),
        }),
        ClassExpression::ObjectSomeValuesFrom { ope, bce } => serde_json::json!({
            "kind": "ObjectSomeValuesFrom",
            "property": iri_to_manchester_term(&ope_to_iri(ope), namespaces),
            "filler": expression_tree_json(bce, namespaces),
        }),
        ClassExpression::ObjectAllValuesFrom { ope, bce } => serde_json::json!({
            "kind": "ObjectAllValuesFrom",
            "property": iri_to_manchester_term(&ope_to_iri(ope), namespaces),
            "filler": expression_tree_json(bce, namespaces),
        }),
        ClassExpression::ObjectMinCardinality { n, ope, bce } => serde_json::json!({
            "kind": "ObjectMinCardinality",
            "cardinality": n,
            "property": iri_to_manchester_term(&ope_to_iri(ope), namespaces),
            "filler": expression_tree_json(bce, namespaces),
        }),
        ClassExpression::ObjectMaxCardinality { n, ope, bce } => serde_json::json!({
            "kind": "ObjectMaxCardinality",
            "cardinality": n,
            "property": iri_to_manchester_term(&ope_to_iri(ope), namespaces),
            "filler": expression_tree_json(bce, namespaces),
        }),
        ClassExpression::ObjectExactCardinality { n, ope, bce } => serde_json::json!({
            "kind": "ObjectExactCardinality",
            "cardinality": n,
            "property": iri_to_manchester_term(&ope_to_iri(ope), namespaces),
            "filler": expression_tree_json(bce, namespaces),
        }),
        other => serde_json::json!({ "kind": format!("{other:?}") }),
    }
}

#[derive(Debug, Clone)]
enum ManchesterAst {
    Class(String),
    Some { property: String, filler: Box<ManchesterAst> },
    Only { property: String, filler: Box<ManchesterAst> },
    And(Vec<ManchesterAst>),
    Or(Vec<ManchesterAst>),
    Min { n: u32, property: String, filler: Box<ManchesterAst> },
    Max { n: u32, property: String, filler: Box<ManchesterAst> },
    Exactly { n: u32, property: String, filler: Box<ManchesterAst> },
}

fn ast_to_class_expression(
    ast: &ManchesterAst,
    build: &Build<RcStr>,
    namespaces: &BTreeMap<String, String>,
) -> Result<ClassExpression<RcStr>> {
    match ast {
        ManchesterAst::Class(iri) => {
            let resolved = resolve_term_iri(iri, namespaces)?;
            Ok(ClassExpression::Class(build.class(resolved)))
        }
        ManchesterAst::Some { property, filler } => {
            let prop = build.object_property(resolve_term_iri(property, namespaces)?);
            let bce = Box::new(ast_to_class_expression(filler, build, namespaces)?);
            Ok(ClassExpression::ObjectSomeValuesFrom {
                ope: ObjectPropertyExpression::ObjectProperty(prop),
                bce,
            })
        }
        ManchesterAst::Only { property, filler } => {
            let prop = build.object_property(resolve_term_iri(property, namespaces)?);
            let bce = Box::new(ast_to_class_expression(filler, build, namespaces)?);
            Ok(ClassExpression::ObjectAllValuesFrom {
                ope: ObjectPropertyExpression::ObjectProperty(prop),
                bce,
            })
        }
        ManchesterAst::And(items) => {
            let exprs: Result<Vec<_>> =
                items.iter().map(|i| ast_to_class_expression(i, build, namespaces)).collect();
            Ok(ClassExpression::ObjectIntersectionOf(exprs?))
        }
        ManchesterAst::Or(items) => {
            let exprs: Result<Vec<_>> =
                items.iter().map(|i| ast_to_class_expression(i, build, namespaces)).collect();
            Ok(ClassExpression::ObjectUnionOf(exprs?))
        }
        ManchesterAst::Min { n, property, filler } => {
            let prop = build.object_property(resolve_term_iri(property, namespaces)?);
            let bce = Box::new(ast_to_class_expression(filler, build, namespaces)?);
            Ok(ClassExpression::ObjectMinCardinality {
                n: *n,
                ope: ObjectPropertyExpression::ObjectProperty(prop),
                bce,
            })
        }
        ManchesterAst::Max { n, property, filler } => {
            let prop = build.object_property(resolve_term_iri(property, namespaces)?);
            let bce = Box::new(ast_to_class_expression(filler, build, namespaces)?);
            Ok(ClassExpression::ObjectMaxCardinality {
                n: *n,
                ope: ObjectPropertyExpression::ObjectProperty(prop),
                bce,
            })
        }
        ManchesterAst::Exactly { n, property, filler } => {
            let prop = build.object_property(resolve_term_iri(property, namespaces)?);
            let bce = Box::new(ast_to_class_expression(filler, build, namespaces)?);
            Ok(ClassExpression::ObjectExactCardinality {
                n: *n,
                ope: ObjectPropertyExpression::ObjectProperty(prop),
                bce,
            })
        }
    }
}

fn resolve_term_iri(term: &str, namespaces: &BTreeMap<String, String>) -> Result<String> {
    if term.starts_with("http://") || term.starts_with("https://") {
        return Ok(term.to_string());
    }
    if let Some(stripped) = term.strip_prefix('<').and_then(|s| s.strip_suffix('>')) {
        return Ok(stripped.to_string());
    }
    if let Some((prefix, local)) = term.split_once(':') {
        if prefix.is_empty() {
            return Err(OwlError::ManchesterInvalid(format!("empty prefix in QName '{term}'")));
        }
        if let Some(ns) = namespaces.get(prefix) {
            return Ok(format!("{ns}{local}"));
        }
        return Err(OwlError::ManchesterInvalid(format!("unknown prefix '{prefix}' in '{term}'")));
    }
    Ok(term.to_string())
}

fn tokenize(input: &str) -> std::result::Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();
    while let Some((start, ch)) = chars.next() {
        if ch.is_whitespace() {
            continue;
        }
        match ch {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '<' => {
                let mut iri = String::new();
                let mut closed = false;
                for (_, c) in chars.by_ref() {
                    if c == '>' {
                        closed = true;
                        break;
                    }
                    iri.push(c);
                }
                if !closed {
                    return Err(format!("unclosed IRI starting at {start}"));
                }
                tokens.push(Token::Iri(iri));
            }
            '0'..='9' => {
                let mut num = ch.to_string();
                while chars.peek().is_some_and(|(_, c)| c.is_ascii_digit()) {
                    num.push(chars.next().unwrap().1);
                }
                let n: u32 = num.parse().map_err(|_| format!("invalid number at {start}"))?;
                tokens.push(Token::Number(n));
            }
            _ if ch.is_alphabetic() || ch == '_' || ch == ':' => {
                let mut ident = ch.to_string();
                while chars.peek().is_some_and(|(_, c)| {
                    c.is_alphanumeric() || *c == '_' || *c == ':' || *c == '-'
                }) {
                    ident.push(chars.next().unwrap().1);
                }
                let lower = ident.to_ascii_lowercase();
                let kw = match lower.as_str() {
                    "and" => Some(Keyword::And),
                    "or" => Some(Keyword::Or),
                    "some" => Some(Keyword::Some),
                    "only" => Some(Keyword::Only),
                    "min" => Some(Keyword::Min),
                    "max" => Some(Keyword::Max),
                    "exactly" => Some(Keyword::Exactly),
                    "not" => Some(Keyword::Not),
                    _ => None,
                };
                if let Some(k) = kw {
                    tokens.push(Token::Keyword(k));
                } else {
                    tokens.push(Token::Ident(ident));
                }
            }
            _ => return Err(format!("unexpected character '{ch}' at {start}")),
        }
    }
    tokens.push(Token::Eof);
    Ok(tokens)
}

#[derive(Debug, Clone)]
enum Token {
    Ident(String),
    Iri(String),
    Keyword(Keyword),
    Number(u32),
    LParen,
    RParen,
    Eof,
}

#[derive(Debug, Clone, Copy)]
enum Keyword {
    And,
    Or,
    Some,
    Only,
    Min,
    Max,
    Exactly,
    Not,
}

struct ManchesterParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl ManchesterParser {
    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if !matches!(tok, Token::Eof) {
            self.pos += 1;
        }
        tok
    }

    fn parse_expression(&mut self) -> std::result::Result<ManchesterAst, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> std::result::Result<ManchesterAst, String> {
        let mut parts = vec![self.parse_and()?];
        while matches!(self.peek(), Token::Keyword(Keyword::Or)) {
            self.advance();
            parts.push(self.parse_and()?);
        }
        if parts.len() == 1 {
            Ok(parts.into_iter().next().unwrap())
        } else {
            Ok(ManchesterAst::Or(parts))
        }
    }

    fn parse_and(&mut self) -> std::result::Result<ManchesterAst, String> {
        let mut parts = vec![self.parse_unary()?];
        while matches!(self.peek(), Token::Keyword(Keyword::And)) {
            self.advance();
            parts.push(self.parse_unary()?);
        }
        if parts.len() == 1 {
            Ok(parts.into_iter().next().unwrap())
        } else {
            Ok(ManchesterAst::And(parts))
        }
    }

    fn parse_unary(&mut self) -> std::result::Result<ManchesterAst, String> {
        if matches!(self.peek(), Token::Keyword(Keyword::Not)) {
            return Err("not expressions are not supported in v0.5 Manchester MVP".to_string());
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> std::result::Result<ManchesterAst, String> {
        if matches!(self.peek(), Token::Keyword(Keyword::Min | Keyword::Max | Keyword::Exactly)) {
            return self.parse_cardinality();
        }
        if matches!(self.peek(), Token::LParen) {
            self.advance();
            let inner = self.parse_expression()?;
            self.expect_paren_r()?;
            return Ok(inner);
        }
        let name = self.parse_name()?;
        if matches!(self.peek(), Token::Keyword(Keyword::Some | Keyword::Only)) {
            let quant = self.advance();
            let filler = self.parse_primary()?;
            return match quant {
                Token::Keyword(Keyword::Some) => {
                    Ok(ManchesterAst::Some { property: name, filler: Box::new(filler) })
                }
                Token::Keyword(Keyword::Only) => {
                    Ok(ManchesterAst::Only { property: name, filler: Box::new(filler) })
                }
                _ => return Err("internal parser error after some/only keyword".to_string()),
            };
        }
        Ok(ManchesterAst::Class(name))
    }

    fn parse_cardinality(&mut self) -> std::result::Result<ManchesterAst, String> {
        let kind = self.advance();
        let Token::Number(n) = self.advance() else {
            return Err("expected cardinality number".to_string());
        };
        let prop = self.parse_name()?;
        let filler = if matches!(self.peek(), Token::Keyword(Keyword::Some)) {
            self.advance();
            self.parse_primary()?
        } else {
            ManchesterAst::Class("owl:Thing".to_string())
        };
        Ok(match kind {
            Token::Keyword(Keyword::Min) => {
                ManchesterAst::Min { n, property: prop, filler: Box::new(filler) }
            }
            Token::Keyword(Keyword::Max) => {
                ManchesterAst::Max { n, property: prop, filler: Box::new(filler) }
            }
            Token::Keyword(Keyword::Exactly) => {
                ManchesterAst::Exactly { n, property: prop, filler: Box::new(filler) }
            }
            _ => return Err("expected min, max, or exactly".to_string()),
        })
    }

    fn parse_name(&mut self) -> std::result::Result<String, String> {
        match self.advance() {
            Token::Ident(s) => Ok(s),
            Token::Iri(iri) => Ok(format!("<{iri}>")),
            other => Err(format!("expected name, got {other:?}")),
        }
    }

    fn expect_paren_r(&mut self) -> std::result::Result<(), String> {
        if !matches!(self.advance(), Token::RParen) {
            return Err("expected ')'".to_string());
        }
        Ok(())
    }
}

pub(crate) fn class_expression_to_turtle_value(
    expr: &ClassExpression<RcStr>,
    namespaces: &BTreeMap<String, String>,
    indent: usize,
) -> Result<String> {
    let pad = "    ".repeat(indent);
    let inner_pad = "    ".repeat(indent + 1);
    match expr {
        ClassExpression::Class(c) => Ok(iri_to_turtle_term(&c.to_string(), namespaces)),
        ClassExpression::ObjectIntersectionOf(v) if v.len() == 1 => {
            class_expression_to_turtle_value(&v[0], namespaces, indent)
        }
        ClassExpression::ObjectSomeValuesFrom { ope, bce } => {
            let prop = iri_to_turtle_term(&ope_to_iri(ope), namespaces);
            let filler = class_expression_to_turtle_value(bce, namespaces, indent + 1)?;
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Restriction ;").ok();
            writeln!(out, "{inner_pad}owl:onProperty {prop} ;").ok();
            writeln!(out, "{inner_pad}owl:someValuesFrom {filler}").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::ObjectAllValuesFrom { ope, bce } => {
            let prop = iri_to_turtle_term(&ope_to_iri(ope), namespaces);
            let filler = class_expression_to_turtle_value(bce, namespaces, indent + 1)?;
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Restriction ;").ok();
            writeln!(out, "{inner_pad}owl:onProperty {prop} ;").ok();
            writeln!(out, "{inner_pad}owl:allValuesFrom {filler}").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::ObjectMinCardinality { n, ope, bce } => {
            cardinality_turtle("owl:minQualifiedCardinality", *n, ope, bce, namespaces, indent)
        }
        ClassExpression::ObjectMaxCardinality { n, ope, bce } => {
            cardinality_turtle("owl:maxQualifiedCardinality", *n, ope, bce, namespaces, indent)
        }
        ClassExpression::ObjectExactCardinality { n, ope, bce } => {
            cardinality_turtle("owl:qualifiedCardinality", *n, ope, bce, namespaces, indent)
        }
        ClassExpression::ObjectIntersectionOf(v) if v.len() > 1 => {
            let terms: Vec<String> = v
                .iter()
                .map(|e| class_expression_to_turtle_value(e, namespaces, indent + 2))
                .collect::<Result<_>>()?;
            let list = terms.join(" ");
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Class ;").ok();
            writeln!(out, "{inner_pad}owl:intersectionOf ( {list} )").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::ObjectUnionOf(v) if v.len() > 1 => {
            let terms: Vec<String> = v
                .iter()
                .map(|e| class_expression_to_turtle_value(e, namespaces, indent + 2))
                .collect::<Result<_>>()?;
            let list = terms.join(" ");
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Class ;").ok();
            writeln!(out, "{inner_pad}owl:unionOf ( {list} )").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::ObjectIntersectionOf(v) => {
            class_expression_to_turtle_value(&v[0], namespaces, indent)
        }
        ClassExpression::ObjectUnionOf(v) => {
            class_expression_to_turtle_value(&v[0], namespaces, indent)
        }
        other => Err(OwlError::ManchesterInvalid(format!(
            "unsupported expression for Turtle: {other:?}"
        ))),
    }
}

fn cardinality_turtle(
    pred: &str,
    n: u32,
    ope: &ObjectPropertyExpression<RcStr>,
    bce: &ClassExpression<RcStr>,
    namespaces: &BTreeMap<String, String>,
    indent: usize,
) -> Result<String> {
    let pad = "    ".repeat(indent);
    let inner_pad = "    ".repeat(indent + 1);
    let prop = iri_to_turtle_term(&ope_to_iri(ope), namespaces);
    let filler = class_expression_to_turtle_value(bce, namespaces, indent + 1)?;
    let mut out = String::new();
    writeln!(out, "[").ok();
    writeln!(out, "{inner_pad}a owl:Restriction ;").ok();
    writeln!(out, "{inner_pad}owl:onProperty {prop} ;").ok();
    writeln!(out, "{inner_pad}{pred} {n} ;").ok();
    writeln!(out, "{inner_pad}owl:onClass {filler}").ok();
    write!(out, "{pad}]").ok();
    Ok(out)
}

fn ope_to_iri(ope: &ObjectPropertyExpression<RcStr>) -> String {
    match ope {
        ObjectPropertyExpression::ObjectProperty(p) => p.0.as_ref().to_string(),
        ObjectPropertyExpression::InverseObjectProperty(p) => {
            format!("inverse {}", p.0.as_ref())
        }
    }
}

fn iri_to_manchester_term(iri: &str, namespaces: &BTreeMap<String, String>) -> String {
    for (prefix, ns) in namespaces {
        if !prefix.is_empty() && iri.starts_with(ns) {
            let local = &iri[ns.len()..];
            return format!("{prefix}:{local}");
        }
    }
    if iri.starts_with("http://") || iri.starts_with("https://") {
        format!("<{iri}>")
    } else {
        iri.to_string()
    }
}

fn iri_to_turtle_term(iri: &str, namespaces: &BTreeMap<String, String>) -> String {
    if iri == "http://www.w3.org/2002/07/owl#Thing" {
        return "owl:Thing".to_string();
    }
    for (prefix, ns) in namespaces {
        if !prefix.is_empty() && iri.starts_with(ns) {
            let local = &iri[ns.len()..];
            return format!("{prefix}:{local}");
        }
    }
    format!("<{iri}>")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex_ns() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("ex".to_string(), "http://example.org/people#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
            ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ])
    }

    fn clinic_ns() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("ex".to_string(), "http://example.org/clinic#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
        ])
    }

    #[test]
    fn parse_some_values_from() {
        let ns = clinic_ns();
        let out = parse_class_expression("ex:hasRecord some ex:MedicalRecord", &ns).expect("parse");
        assert!(out.normalized.contains("some"));
    }

    #[test]
    fn parse_and_expression() {
        let ns = ex_ns();
        let out = parse_class_expression("ex:Person and ex:Organization", &ns).expect("parse");
        assert!(out.normalized.contains("and"));
    }

    #[test]
    fn turtle_fragment_for_restriction() {
        let ns = clinic_ns();
        let out = parse_class_expression("ex:hasRecord some ex:MedicalRecord", &ns).expect("parse");
        let turtle = class_expression_to_turtle_fragment(&out.expression, "rdfs:subClassOf", &ns)
            .expect("turtle");
        assert!(turtle.contains("owl:Restriction"));
        assert!(turtle.contains("owl:someValuesFrom"));
    }

    #[test]
    fn turtle_intersection_includes_all_operands() {
        let ns = ex_ns();
        let out = parse_class_expression("ex:Person and ex:Organization", &ns).expect("parse");
        let turtle = class_expression_to_turtle_value(&out.expression, &ns, 0).expect("turtle");
        assert!(turtle.contains("ex:Person"));
        assert!(turtle.contains("ex:Organization"));
        assert!(turtle.contains("owl:intersectionOf"));
    }
}
