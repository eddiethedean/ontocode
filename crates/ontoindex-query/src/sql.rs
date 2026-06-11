use crate::QueryError;
use ontoindex_catalog::OntologyCatalog;
use ontoindex_core::{limits::MAX_QUERY_BYTES, limits::MAX_SQL_RESULT_ROWS, EntityKind};
use serde::Serialize;
use sqlparser::ast::{Expr, Select, SelectItem, SetExpr, Statement, TableFactor, Value};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::collections::BTreeMap;

pub type Result<T> = std::result::Result<T, QueryError>;

type Row = BTreeMap<String, String>;

#[derive(Debug, Clone, Serialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Row>,
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

    let statement =
        statements.into_iter().next().ok_or_else(|| QueryError::Sql("empty query".to_string()))?;

    match statement {
        Statement::Query(query) => {
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
    let table_name = table_name_from_select(&select)?;
    let mut rows = table_rows(catalog, &table_name)?;

    if let Some(selection) = &select.selection {
        rows.retain(|row| evaluate_filter(selection, row));
    }

    let (columns, projected_rows) = project_rows(&select.projection, &rows)?;
    let rows = truncate_rows(projected_rows);
    Ok(QueryResult { columns, rows })
}

fn truncate_rows(mut rows: Vec<Row>) -> Vec<Row> {
    if rows.len() > MAX_SQL_RESULT_ROWS {
        rows.truncate(MAX_SQL_RESULT_ROWS);
    }
    rows
}

fn table_name_from_select(select: &Select) -> Result<String> {
    let from =
        select.from.first().ok_or_else(|| QueryError::Sql("missing FROM clause".to_string()))?;

    match &from.relation {
        TableFactor::Table { name, .. } => Ok(name.to_string().to_ascii_lowercase()),
        _ => Err(QueryError::Sql("unsupported table expression".to_string())),
    }
}

fn table_rows(catalog: &OntologyCatalog, table: &str) -> Result<Vec<Row>> {
    let data = catalog.data();
    match table {
        "ontologies" => Ok(data
            .documents
            .iter()
            .map(|doc| {
                let mut row = BTreeMap::new();
                row.insert("id".into(), doc.id.clone());
                row.insert("path".into(), doc.path.display().to_string());
                row.insert("format".into(), doc.format.as_str().to_string());
                row.insert("base_iri".into(), doc.base_iri.clone().unwrap_or_default());
                row.insert("parse_status".into(), format!("{:?}", doc.parse_status));
                row.insert("content_hash".into(), doc.content_hash.clone());
                row.insert("modified_time".into(), doc.modified_time.to_string());
                row
            })
            .collect()),
        "classes" => entity_rows(catalog, EntityKind::Class),
        "object_properties" => entity_rows(catalog, EntityKind::ObjectProperty),
        "data_properties" => entity_rows(catalog, EntityKind::DataProperty),
        "annotation_properties" => entity_rows(catalog, EntityKind::AnnotationProperty),
        "individuals" => entity_rows(catalog, EntityKind::Individual),
        "entities" => Ok(data.entities.iter().map(entity_to_row).collect()),
        "annotations" => Ok(data
            .annotations
            .iter()
            .map(|a| {
                let mut row = BTreeMap::new();
                row.insert("subject".into(), a.subject.clone());
                row.insert("predicate".into(), a.predicate.clone());
                row.insert("object".into(), a.object.clone());
                row.insert("ontology_id".into(), a.ontology_id.clone());
                row
            })
            .collect()),
        "axioms" => Ok(data
            .axioms
            .iter()
            .map(|a| {
                let mut row = BTreeMap::new();
                row.insert("id".into(), a.id.clone());
                row.insert("ontology_id".into(), a.ontology_id.clone());
                row.insert("subject".into(), a.subject.clone());
                row.insert("predicate".into(), a.predicate.clone());
                row.insert("object".into(), a.object.clone());
                row.insert("axiom_kind".into(), a.axiom_kind.clone());
                row
            })
            .collect()),
        "namespaces" => Ok(data
            .namespaces
            .iter()
            .map(|n| {
                let mut row = BTreeMap::new();
                row.insert("prefix".into(), n.prefix.clone());
                row.insert("iri".into(), n.iri.clone());
                row.insert("ontology_id".into(), n.ontology_id.clone());
                row
            })
            .collect()),
        "imports" => Ok(data
            .imports
            .iter()
            .map(|i| {
                let mut row = BTreeMap::new();
                row.insert("ontology_id".into(), i.ontology_id.clone());
                row.insert("import_iri".into(), i.import_iri.clone());
                row
            })
            .collect()),
        "properties" => {
            let mut rows = entity_rows(catalog, EntityKind::ObjectProperty)?;
            rows.extend(entity_rows(catalog, EntityKind::DataProperty)?);
            rows.extend(entity_rows(catalog, EntityKind::AnnotationProperty)?);
            Ok(rows)
        }
        other => Err(QueryError::Sql(format!("unknown table: {other}"))),
    }
}

fn entity_rows(catalog: &OntologyCatalog, kind: EntityKind) -> Result<Vec<Row>> {
    Ok(catalog.data().entities.iter().filter(|e| e.kind == kind).map(entity_to_row).collect())
}

fn entity_to_row(entity: &ontoindex_core::Entity) -> Row {
    let mut row = BTreeMap::new();
    row.insert("iri".into(), entity.iri.clone());
    row.insert("short_name".into(), entity.short_name.clone());
    row.insert("kind".into(), entity.kind.as_str().to_string());
    row.insert("ontology_id".into(), entity.ontology_id.clone());
    row.insert("labels".into(), entity.labels.join("; "));
    row.insert("comments".into(), entity.comments.join("; "));
    row.insert("deprecated".into(), entity.deprecated.to_string());
    row
}

fn project_rows(projection: &[SelectItem], rows: &[Row]) -> Result<(Vec<String>, Vec<Row>)> {
    if projection.len() == 1 && matches!(projection[0], SelectItem::Wildcard(_)) {
        let columns = rows.first().map(|r| r.keys().cloned().collect()).unwrap_or_default();
        return Ok((columns, rows.to_vec()));
    }

    let mut columns = Vec::new();
    for item in projection {
        match item {
            SelectItem::UnnamedExpr(Expr::Identifier(ident)) => {
                columns.push(ident.value.to_ascii_lowercase());
            }
            SelectItem::ExprWithAlias { alias, .. } => {
                columns.push(alias.value.to_ascii_lowercase());
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

    let projected = rows
        .iter()
        .map(|row| {
            let mut out = BTreeMap::new();
            for col in &columns {
                out.insert(col.clone(), row.get(col).cloned().unwrap_or_default());
            }
            out
        })
        .collect();

    Ok((columns, projected))
}

fn evaluate_filter(expr: &Expr, row: &Row) -> bool {
    match expr {
        Expr::BinaryOp { left, op, right } => {
            use sqlparser::ast::BinaryOperator;
            let left_val = eval_expr(left, row);
            let right_val = eval_expr(right, row);
            match op {
                BinaryOperator::Eq => left_val == right_val,
                BinaryOperator::NotEq => left_val != right_val,
                BinaryOperator::And => evaluate_filter(left, row) && evaluate_filter(right, row),
                BinaryOperator::Or => evaluate_filter(left, row) || evaluate_filter(right, row),
                _ => false,
            }
        }
        Expr::Identifier(ident) => {
            row.get(&ident.value.to_ascii_lowercase()).map(|v| v == "true").unwrap_or(false)
        }
        Expr::Value(Value::Boolean(b)) => *b,
        _ => false,
    }
}

fn eval_expr(expr: &Expr, row: &Row) -> String {
    match expr {
        Expr::Identifier(ident) => {
            row.get(&ident.value.to_ascii_lowercase()).cloned().unwrap_or_default()
        }
        Expr::Value(Value::SingleQuotedString(s) | Value::DoubleQuotedString(s)) => s.clone(),
        Expr::Value(Value::Boolean(b)) => b.to_string(),
        Expr::Value(Value::Number(n, _)) => n.clone(),
        _ => String::new(),
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
    serde_json::to_string_pretty(result).map_err(|e| crate::QueryError::Export(e.to_string()))
}
