//! use the [`hcl-rs`][hcl] crate to query HCL documents

use std::error::Error;

use hcl::{Block, Body, Expression, Identifier, ObjectKey};

use crate::parser::Field;

/// a portion of an HCL document that matched the provided filter
pub enum QueryResult {
    /// an HCL [`Expression`] matched the filter
    Expr(Expression),
    /// an HCL [`Block`] matched the filter
    Block(Block),
}

impl QueryResult {
    pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
        let s = match self {
            // beware `hcl::to_string`!
            // https://github.com/martinohmann/hcl-rs/issues/344
            Self::Expr(expr) => hcl::format::to_string(expr)?,
            Self::Block(block) => hcl::format::to_string(block)?,
        };
        Ok(s)
    }
}

/// given a vector of [`Field`]s return a vector of [`QueryResult`]s
///
/// a result vector with multiple results indicates that multiple entities
/// matched the provided filter
pub fn query(fields: &mut Vec<Field>, body: &Body) -> Vec<QueryResult> {
    if fields.is_empty() {
        // our grammar/parser for filters won't allow an empty filter
        unreachable!();
    }

    // take the first field and do a `Body` query
    // e.g. `.foo.bar` will start with 'foo'
    let field = fields.remove(0);
    let mut query_result = body_query(&field, body);

    // iteratively evaluate each subsequent field
    // e.g. having handled 'foo' we move on to 'bar'
    while !fields.is_empty() {
        let field = fields.remove(0);
        query_result = result_query(&field, query_result);
    }

    query_result
}

fn body_query(field: &Field, body: &Body) -> Vec<QueryResult> {
    let mut matches = Vec::new();
    let mut attr_matches = attr_query(&field.name, body);
    matches.append(&mut attr_matches);
    let mut block_matches = block_query(field, body);
    matches.append(&mut block_matches);
    matches
}

fn attr_query(field: &str, body: &Body) -> Vec<QueryResult> {
    let mut matches = Vec::new();
    for attr in body.attributes() {
        if attr.key() == field {
            matches.push(QueryResult::Expr(attr.expr().clone()));
        }
    }
    matches
}

fn block_query(field: &Field, body: &Body) -> Vec<QueryResult> {
    let mut matches = Vec::new();
    for block in body.blocks() {
        if block.identifier() != field.name {
            continue;
        }
        if field.labels.is_empty() {
            matches.push(QueryResult::Block(block.clone()));
        }
        for filter_label in &field.labels {
            for block_label in block.labels() {
                if block_label.as_str() == filter_label {
                    matches.push(QueryResult::Block(block.clone()));
                }
            }
        }
    }
    matches
}

fn result_query(field: &Field, query_results: Vec<QueryResult>) -> Vec<QueryResult> {
    let mut matches = Vec::new();
    for query_result in query_results {
        match query_result {
            QueryResult::Expr(expr) => {
                if let Expression::Object(object) = expr {
                    // some objects are keyed with an Identifier
                    let key = ObjectKey::Identifier(Identifier::new(&field.name).unwrap());
                    if let Some(expr) = object.get(&key) {
                        matches.push(QueryResult::Expr(expr.clone()));
                    }
                    // some objects are keyed with a String Expression
                    let key = ObjectKey::Expression(Expression::String(field.name.clone()));
                    if let Some(expr) = object.get(&key) {
                        matches.push(QueryResult::Expr(expr.clone()));
                    }
                }
            }
            QueryResult::Block(block) => {
                let mut body_matches = body_query(field, block.body());
                matches.append(&mut body_matches);
            }
        }
    }
    matches
}
