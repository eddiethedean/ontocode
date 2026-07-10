use crate::schema::list_sql_tables;
use crate::QueryError;
use ontocore_catalog::OntologyCatalog;
use ontocore_core::{
    limits::MAX_QUERY_BYTES, limits::MAX_SQL_RESULT_ROWS, EntityKind, AXIOM_KIND_DISJOINT_CLASS,
    AXIOM_KIND_DOMAIN, AXIOM_KIND_EQUIVALENT_CLASS, AXIOM_KIND_RANGE, AXIOM_KIND_SUB_CLASS_OF,
};
use serde::Serialize;
use sqlparser::ast::{
    Expr, GroupByExpr, Select, SelectItem, SetExpr, Statement, TableFactor, Value,
};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::collections::{BTreeMap, HashSet};

pub type Result<T> = std::result::Result<T, QueryError>;

type Row = BTreeMap<String, String>;

#[derive(Debug, Clone, Serialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Row>,
    pub truncated: bool,
}

pub fn run_sql(catalog: &OntologyCatalog, sql: &str) -> Result<QueryResult> {
    if sql.len() > MAX_QUERY_BYTES {
        return Err(QueryError::Sql(format!(
            "query exceeds maximum length of {MAX_QUERY_BYTES} bytes"
        )));
    }

    let dialect = GenericDialect {};
    let statements =
        Parser::parse_sql(&dialect, sql).map_err(|e| QueryError::Sql(e.to_string()))?;

    if statements.len() > 1 {
        return Err(QueryError::Sql("only a single SQL statement is supported".to_string()));
    }

    let statement =
        statements.into_iter().next().ok_or_else(|| QueryError::Sql("empty query".to_string()))?;

    match statement {
        Statement::Query(query) => {
            if query.order_by.is_some() {
                return Err(QueryError::Sql("ORDER BY is not supported".to_string()));
            }
            if query.limit.is_some() || query.offset.is_some() {
                return Err(QueryError::Sql("LIMIT and OFFSET are not supported".to_string()));
            }
            let select = match *query.body {
                SetExpr::Select(select) => select,
                _ => return Err(QueryError::Sql("only SELECT queries are supported".to_string())),
            };
            execute_select(catalog, select)
        }
        _ => Err(QueryError::Sql("only SELECT queries are supported".to_string())),
    }
}

fn execute_select(catalog: &OntologyCatalog, select: Box<Select>) -> Result<QueryResult> {
    if select.distinct.is_some() {
        return Err(QueryError::Sql("DISTINCT is not supported".to_string()));
    }
    if group_by_present(&select.group_by) {
        return Err(QueryError::Sql("GROUP BY is not supported".to_string()));
    }
    if select.having.is_some() {
        return Err(QueryError::Sql("HAVING is not supported".to_string()));
    }
    if select.from.len() > 1 {
        return Err(QueryError::Sql("JOIN is not supported".to_string()));
    }
    let table_name = table_name_from_select(&select)?;
    let known_columns = known_columns_for_table(&table_name)?;
    validate_projection(&select.projection, &known_columns)?;
    if let Some(selection) = &select.selection {
        if matches!(selection, Expr::Identifier(_)) {
            return Err(QueryError::Sql(
                "bare column names are not supported in WHERE; use column = 'value'".to_string(),
            ));
        }
        validate_filter(selection, &known_columns)?;
    }

    let mut rows = Vec::new();
    let mut truncated = false;
    for row in table_row_iter(catalog, &table_name)? {
        if let Some(selection) = &select.selection {
            if !evaluate_filter(selection, &row)? {
                continue;
            }
        }
        if rows.len() >= MAX_SQL_RESULT_ROWS {
            truncated = true;
            break;
        }
        rows.push(row);
    }

    let (columns, projected_rows) = project_rows(&select.projection, &rows)?;
    Ok(QueryResult { columns, rows: projected_rows, truncated })
}

fn known_columns_for_table(table: &str) -> Result<HashSet<String>> {
    list_sql_tables()
        .into_iter()
        .find(|t| t.name == table)
        .map(|t| t.columns.into_iter().map(|c| c.name).collect())
        .ok_or_else(|| QueryError::Sql(format!("unknown table: {table}")))
}

fn ensure_known_column(known: &HashSet<String>, name: &str) -> Result<()> {
    if known.contains(name) {
        Ok(())
    } else {
        Err(QueryError::Sql(format!("unknown column: {name}")))
    }
}

fn validate_projection(projection: &[SelectItem], known: &HashSet<String>) -> Result<()> {
    if projection.len() == 1 && matches!(projection[0], SelectItem::Wildcard(_)) {
        return Ok(());
    }
    for col in projection_columns(projection)? {
        ensure_known_column(known, &col.source)?;
    }
    Ok(())
}

fn table_name_from_select(select: &Select) -> Result<String> {
    let from =
        select.from.first().ok_or_else(|| QueryError::Sql("missing FROM clause".to_string()))?;

    match &from.relation {
        TableFactor::Table { name, .. } => Ok(name.to_string().to_ascii_lowercase()),
        _ => Err(QueryError::Sql("unsupported table expression".to_string())),
    }
}

fn table_row_iter<'a>(
    catalog: &'a OntologyCatalog,
    table: &str,
) -> Result<Box<dyn Iterator<Item = Row> + 'a>> {
    let data = catalog.data();
    match table {
        "ontologies" => Ok(Box::new(data.documents.iter().map(|doc| {
            let mut row = BTreeMap::new();
            row.insert("id".into(), doc.id.clone());
            row.insert("path".into(), doc.path.display().to_string());
            row.insert("format".into(), doc.format.as_str().to_string());
            row.insert("base_iri".into(), doc.base_iri.clone().unwrap_or_default());
            row.insert("parse_status".into(), doc.parse_status.as_str().to_string());
            row.insert("content_hash".into(), doc.content_hash.clone());
            row.insert("modified_time".into(), doc.modified_time.to_string());
            row
        }))),
        "classes" => entity_row_iter(catalog, EntityKind::Class),
        "object_properties" => entity_row_iter(catalog, EntityKind::ObjectProperty),
        "data_properties" => entity_row_iter(catalog, EntityKind::DataProperty),
        "annotation_properties" => entity_row_iter(catalog, EntityKind::AnnotationProperty),
        "individuals" => entity_row_iter(catalog, EntityKind::Individual),
        "entities" => Ok(Box::new(data.entities.iter().map(entity_to_row))),
        "annotations" => Ok(Box::new(data.annotations.iter().map(|a| {
            let mut row = BTreeMap::new();
            row.insert("subject".into(), a.subject.clone());
            row.insert("predicate".into(), a.predicate.clone());
            row.insert("object".into(), a.object.clone());
            row.insert("ontology_id".into(), a.ontology_id.clone());
            row
        }))),
        "axioms" => Ok(Box::new(data.axioms.iter().map(|a| {
            let mut row = BTreeMap::new();
            row.insert("id".into(), a.id.clone());
            row.insert("ontology_id".into(), a.ontology_id.clone());
            row.insert("subject".into(), a.subject.clone());
            row.insert("predicate".into(), a.predicate.clone());
            row.insert("object".into(), a.object.clone());
            row.insert("axiom_kind".into(), a.axiom_kind.clone());
            row
        }))),
        "namespaces" => Ok(Box::new(data.namespaces.iter().map(|n| {
            let mut row = BTreeMap::new();
            row.insert("prefix".into(), n.prefix.clone());
            row.insert("iri".into(), n.iri.clone());
            row.insert("ontology_id".into(), n.ontology_id.clone());
            row
        }))),
        "imports" => Ok(Box::new(data.imports.iter().map(|i| {
            let mut row = BTreeMap::new();
            row.insert("ontology_id".into(), i.ontology_id.clone());
            row.insert("import_iri".into(), i.import_iri.clone());
            row
        }))),
        "diagnostics" => Ok(Box::new(data.diagnostics.iter().map(|d| {
            let mut row = BTreeMap::new();
            row.insert("code".into(), d.code.as_str().to_string());
            row.insert("severity".into(), d.severity.as_str().to_string());
            row.insert("message".into(), d.message.clone());
            row.insert("file".into(), d.file.display().to_string());
            row.insert("line".into(), d.range.line.map(|l| l.to_string()).unwrap_or_default());
            row.insert("column".into(), d.range.column.map(|c| c.to_string()).unwrap_or_default());
            row.insert("entity_iri".into(), d.entity_iri.clone().unwrap_or_default());
            row
        }))),
        "equivalent_class_axioms" => {
            axiom_kind_rows(catalog, AXIOM_KIND_EQUIVALENT_CLASS, "class_iri", "expression")
        }
        "disjoint_class_axioms" => {
            axiom_kind_rows(catalog, AXIOM_KIND_DISJOINT_CLASS, "class_iri", "disjoint_with")
        }
        "domain_axioms" => axiom_kind_rows(catalog, AXIOM_KIND_DOMAIN, "property_iri", "domain"),
        "range_axioms" => axiom_kind_rows(catalog, AXIOM_KIND_RANGE, "property_iri", "range"),
        "restrictions" => restriction_rows(catalog),
        "properties" => {
            let mut iter: Box<dyn Iterator<Item = Row>> = Box::new(std::iter::empty());
            for kind in [
                EntityKind::ObjectProperty,
                EntityKind::DataProperty,
                EntityKind::AnnotationProperty,
            ] {
                let next = entity_row_iter(catalog, kind)?;
                iter = Box::new(iter.chain(next));
            }
            Ok(iter)
        }
        other => Err(QueryError::Sql(format!("unknown table: {other}"))),
    }
}

fn entity_row_iter(
    catalog: &OntologyCatalog,
    kind: EntityKind,
) -> Result<Box<dyn Iterator<Item = Row> + '_>> {
    Ok(Box::new(catalog.data().entities.iter().filter(move |e| e.kind == kind).map(entity_to_row)))
}

fn axiom_kind_rows<'a>(
    catalog: &'a OntologyCatalog,
    kind: &str,
    col_a: &str,
    col_b: &str,
) -> Result<Box<dyn Iterator<Item = Row> + 'a>> {
    let col_a = col_a.to_string();
    let col_b = col_b.to_string();
    let kind = kind.to_string();
    Ok(Box::new(catalog.data().axioms.iter().filter(move |a| a.axiom_kind == kind).map(move |a| {
        let mut row = BTreeMap::new();
        row.insert(col_a.clone(), a.subject.clone());
        row.insert(col_b.clone(), a.object.clone());
        row
    })))
}

fn restriction_rows<'a>(
    catalog: &'a OntologyCatalog,
) -> Result<Box<dyn Iterator<Item = Row> + 'a>> {
    Ok(Box::new(
        catalog
            .data()
            .axioms
            .iter()
            .filter(|a| a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF && is_restriction_expr(&a.object))
            .map(|a| {
                let (property_iri, restriction_kind) = parse_restriction_header(&a.object);
                let mut row = BTreeMap::new();
                row.insert("class_iri".into(), a.subject.clone());
                row.insert("property_iri".into(), property_iri);
                row.insert("restriction_kind".into(), restriction_kind);
                row.insert("filler".into(), a.object.clone());
                row
            }),
    ))
}

fn is_restriction_expr(expr: &str) -> bool {
    let lower = expr.to_ascii_lowercase();
    lower.contains(" some ")
        || lower.contains(" only ")
        || lower.contains(" value ")
        || lower.contains(" min ")
        || lower.contains(" max ")
        || lower.contains("self")
}

fn parse_restriction_header(expr: &str) -> (String, String) {
    let trimmed = expr.trim();
    for kind in ["some", "only", "value", "min", "max", "self"] {
        let needle = format!(" {kind} ");
        if let Some(idx) = trimmed.to_ascii_lowercase().find(&needle) {
            let property = trimmed[..idx].trim().to_string();
            return (property, kind.to_string());
        }
        if trimmed.to_ascii_lowercase().ends_with(kind) && kind == "self" {
            let property =
                trimmed.trim_end_matches("self").trim().trim_end_matches("and").trim().to_string();
            return (property, kind.to_string());
        }
    }
    (String::new(), "complex".to_string())
}

fn entity_to_row(entity: &ontocore_core::Entity) -> Row {
    let mut row = BTreeMap::new();
    row.insert("iri".into(), entity.iri.clone());
    row.insert("short_name".into(), entity.short_name.clone());
    row.insert("kind".into(), entity.kind.as_str().to_string());
    row.insert("ontology_id".into(), entity.ontology_id.clone());
    row.insert("labels".into(), entity.labels.join("; "));
    row.insert("comments".into(), entity.comments.join("; "));
    row.insert("deprecated".into(), entity.deprecated.to_string());
    if let Some(ref obo_id) = entity.obo_id {
        row.insert("obo_id".into(), obo_id.clone());
    }
    row
}

struct ProjectionCol {
    name: String,
    source: String,
}

fn projection_columns(projection: &[SelectItem]) -> Result<Vec<ProjectionCol>> {
    if projection.len() == 1 && matches!(projection[0], SelectItem::Wildcard(_)) {
        return Ok(Vec::new());
    }

    let mut columns = Vec::new();
    for item in projection {
        match item {
            SelectItem::UnnamedExpr(Expr::Identifier(ident)) => {
                let col = ident.value.to_ascii_lowercase();
                columns.push(ProjectionCol { name: col.clone(), source: col });
            }
            SelectItem::ExprWithAlias { expr, alias, .. } => {
                let source = match expr {
                    Expr::Identifier(ident) => ident.value.to_ascii_lowercase(),
                    _ => {
                        return Err(QueryError::Sql(
                            "only simple column projections are supported".to_string(),
                        ));
                    }
                };
                columns.push(ProjectionCol { name: alias.value.to_ascii_lowercase(), source });
            }
            SelectItem::Wildcard(_) => {
                return Err(QueryError::Sql("wildcard projection must be used alone".to_string()));
            }
            _ => {
                return Err(QueryError::Sql(
                    "only simple column projections are supported".to_string(),
                ));
            }
        }
    }
    Ok(columns)
}

fn project_rows(projection: &[SelectItem], rows: &[Row]) -> Result<(Vec<String>, Vec<Row>)> {
    if projection.len() == 1 && matches!(projection[0], SelectItem::Wildcard(_)) {
        let columns = rows.first().map(|r| r.keys().cloned().collect()).unwrap_or_default();
        return Ok((columns, rows.to_vec()));
    }

    let columns_spec = projection_columns(projection)?;
    let columns: Vec<String> = columns_spec.iter().map(|c| c.name.clone()).collect();

    let projected = rows
        .iter()
        .map(|row| {
            let mut out = BTreeMap::new();
            for col in &columns_spec {
                out.insert(col.name.clone(), row.get(&col.source).cloned().unwrap_or_default());
            }
            out
        })
        .collect();

    Ok((columns, projected))
}

fn group_by_present(group_by: &GroupByExpr) -> bool {
    match group_by {
        GroupByExpr::All(_) => true,
        GroupByExpr::Expressions(exprs, _) => !exprs.is_empty(),
    }
}

fn validate_filter(expr: &Expr, known: &HashSet<String>) -> Result<()> {
    match expr {
        Expr::BinaryOp { left, op, right } => {
            use sqlparser::ast::BinaryOperator;
            match op {
                BinaryOperator::Eq
                | BinaryOperator::NotEq
                | BinaryOperator::And
                | BinaryOperator::Or => {
                    validate_filter(left, known)?;
                    validate_filter(right, known)?;
                    Ok(())
                }
                other => Err(QueryError::Sql(format!("unsupported WHERE operator: {other:?}"))),
            }
        }
        Expr::Identifier(ident) => ensure_known_column(known, &ident.value.to_ascii_lowercase()),
        Expr::Value(_) => Ok(()),
        other => Err(QueryError::Sql(format!("unsupported WHERE expression: {other:?}"))),
    }
}

fn evaluate_filter(expr: &Expr, row: &Row) -> Result<bool> {
    match expr {
        Expr::BinaryOp { left, op, right } => {
            use sqlparser::ast::BinaryOperator;
            match op {
                BinaryOperator::Eq => Ok(eval_expr(left, row)? == eval_expr(right, row)?),
                BinaryOperator::NotEq => Ok(eval_expr(left, row)? != eval_expr(right, row)?),
                BinaryOperator::And => {
                    Ok(evaluate_filter(left, row)? && evaluate_filter(right, row)?)
                }
                BinaryOperator::Or => {
                    Ok(evaluate_filter(left, row)? || evaluate_filter(right, row)?)
                }
                other => Err(QueryError::Sql(format!("unsupported WHERE operator: {other:?}"))),
            }
        }
        Expr::Identifier(ident) => {
            let key = ident.value.to_ascii_lowercase();
            match row.get(&key) {
                Some(v) => Ok(v == "true"),
                None => Err(QueryError::Sql(format!("unknown column: {key}"))),
            }
        }
        Expr::Value(Value::Boolean(b)) => Ok(*b),
        other => Err(QueryError::Sql(format!("unsupported WHERE expression: {other:?}"))),
    }
}

fn eval_expr(expr: &Expr, row: &Row) -> Result<String> {
    match expr {
        Expr::Identifier(ident) => {
            let key = ident.value.to_ascii_lowercase();
            // Optional schema columns (e.g. obo_id) may be absent on a given row.
            // Unknown identifiers are rejected earlier via schema validation.
            Ok(row.get(&key).cloned().unwrap_or_default())
        }
        Expr::Value(Value::SingleQuotedString(s) | Value::DoubleQuotedString(s)) => Ok(s.clone()),
        Expr::Value(Value::Boolean(b)) => Ok(b.to_string()),
        Expr::Value(Value::Number(n, _)) => Ok(n.clone()),
        other => Err(QueryError::Sql(format!("unsupported expression: {other:?}"))),
    }
}

pub fn to_csv(result: &QueryResult) -> Result<String> {
    let mut writer = csv::Writer::from_writer(Vec::new());
    writer.write_record(&result.columns).map_err(|e| crate::QueryError::Export(e.to_string()))?;
    for row in &result.rows {
        let values: Vec<String> =
            result.columns.iter().map(|c| row.get(c).cloned().unwrap_or_default()).collect();
        writer.write_record(&values).map_err(|e| crate::QueryError::Export(e.to_string()))?;
    }
    let bytes = writer.into_inner().map_err(|e| crate::QueryError::Export(e.to_string()))?;
    String::from_utf8(bytes).map_err(|e| crate::QueryError::Export(e.to_string()))
}

pub fn to_json(result: &QueryResult) -> Result<String> {
    let rows: Vec<Vec<String>> = result
        .rows
        .iter()
        .map(|row| result.columns.iter().map(|c| row.get(c).cloned().unwrap_or_default()).collect())
        .collect();
    serde_json::to_string_pretty(&serde_json::json!({
        "columns": result.columns,
        "rows": rows,
        "truncated": result.truncated,
    }))
    .map_err(|e| crate::QueryError::Export(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontocore_catalog::OntologyCatalog;
    use ontocore_core::limits::MAX_QUERY_BYTES;
    use std::path::PathBuf;

    fn fixture_catalog() -> OntologyCatalog {
        let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        ontocore_catalog::IndexBuilder::new().workspace(fixtures).build().expect("index fixtures")
    }

    #[test]
    fn where_and_filters_rows() {
        let catalog = fixture_catalog();
        let result = run_sql(
            &catalog,
            "SELECT short_name FROM classes WHERE short_name = 'Person' AND deprecated = 'false'",
        )
        .expect("and filter");
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0].get("short_name").map(String::as_str), Some("Person"));
    }

    #[test]
    fn where_or_filters_rows() {
        let catalog = fixture_catalog();
        let result = run_sql(
            &catalog,
            "SELECT short_name FROM classes WHERE short_name = 'Person' OR short_name = 'Thing'",
        )
        .expect("or filter");
        let names: Vec<_> =
            result.rows.iter().filter_map(|r| r.get("short_name").cloned()).collect();
        assert!(names.contains(&"Person".to_string()));
        assert!(names.contains(&"Thing".to_string()));
    }

    #[test]
    fn where_not_eq_excludes_matching_row() {
        let catalog = fixture_catalog();
        let result =
            run_sql(&catalog, "SELECT short_name FROM classes WHERE short_name != 'Person'")
                .expect("not eq filter");
        assert!(!result
            .rows
            .iter()
            .any(|r| r.get("short_name").map(String::as_str) == Some("Person")));
        assert!(!result.rows.is_empty());
    }

    #[test]
    fn unsupported_like_returns_error() {
        let catalog = fixture_catalog();
        let err = run_sql(&catalog, "SELECT short_name FROM classes WHERE short_name LIKE 'Per%'")
            .unwrap_err();
        assert!(matches!(err, crate::QueryError::Sql(msg) if msg.contains("unsupported WHERE")));
    }

    #[test]
    fn unsupported_limit_returns_error() {
        let catalog = fixture_catalog();
        let err = run_sql(&catalog, "SELECT short_name FROM classes LIMIT 1").unwrap_err();
        assert!(matches!(err, crate::QueryError::Sql(msg) if msg.contains("LIMIT")));
    }

    #[test]
    fn unsupported_having_returns_error() {
        let catalog = fixture_catalog();
        let err = run_sql(&catalog, "SELECT short_name FROM classes HAVING short_name = 'Person'")
            .unwrap_err();
        assert!(matches!(err, crate::QueryError::Sql(msg) if msg.contains("HAVING")));
    }

    #[test]
    fn rejects_oversized_query() {
        let catalog = fixture_catalog();
        let padding = "x".repeat(MAX_QUERY_BYTES);
        let sql = format!("SELECT short_name FROM classes WHERE short_name = '{padding}'");
        let err = run_sql(&catalog, &sql).unwrap_err();
        assert!(matches!(err, crate::QueryError::Sql(msg) if msg.contains("maximum length")));
    }

    #[test]
    fn select_alias_uses_source_column() {
        let catalog = fixture_catalog();
        let result =
            run_sql(&catalog, "SELECT short_name AS name FROM classes WHERE short_name = 'Person'")
                .expect("alias projection");
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0].get("name").map(String::as_str), Some("Person"));
    }

    #[test]
    fn unknown_select_column_returns_error() {
        let catalog = fixture_catalog();
        let err = run_sql(&catalog, "SELECT nonexistent FROM classes").unwrap_err();
        assert!(matches!(err, crate::QueryError::Sql(msg) if msg.contains("unknown column")));
    }

    #[test]
    fn unknown_where_column_returns_error() {
        let catalog = fixture_catalog();
        let err = run_sql(&catalog, "SELECT short_name FROM classes WHERE nonexistent = ''")
            .unwrap_err();
        assert!(matches!(err, crate::QueryError::Sql(msg) if msg.contains("unknown column")));
    }

    #[test]
    fn optional_obo_id_column_is_allowed() {
        let catalog = fixture_catalog();
        let result = run_sql(&catalog, "SELECT obo_id FROM classes").expect("obo_id projection");
        assert!(!result.rows.is_empty());
        assert!(result.rows.iter().all(|r| r.contains_key("obo_id")));
    }
}
