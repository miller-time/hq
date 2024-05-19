use hcl::{Body, Expression, Identifier, ObjectKey};

pub enum QueryResult {
    Expr(Expression),
    Body(Body),
}

pub fn query(fields: &mut Vec<String>, body: &Body) -> Option<QueryResult> {
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

fn body_query(field: &str, body: &Body) -> Option<QueryResult> {
    if let Some(value) = attr_query(field, body) {
        return Some(value);
    }
    block_query(field, body)
}

fn attr_query(field: &str, body: &Body) -> Option<QueryResult> {
    for attr in body.attributes() {
        if attr.key() == field {
            return Some(QueryResult::Expr(attr.expr().clone()));
        }
    }
    None
}

fn block_query(field: &str, body: &Body) -> Option<QueryResult> {
    for block in body.blocks() {
        if block.identifier() == field {
            return Some(QueryResult::Body(block.body().clone()));
        }
    }
    None
}

fn result_query(field: &str, query_result: Option<QueryResult>) -> Option<QueryResult> {
    if let Some(query_result) = query_result {
        match query_result {
            QueryResult::Expr(expr) => {
                if let Expression::Object(object) = expr {
                    let key = ObjectKey::Identifier(Identifier::new(field).unwrap());
                    if let Some(expr) = object.get(&key) {
                        return Some(QueryResult::Expr(expr.clone()));
                    }
                }
            }
            QueryResult::Body(body) => {
                return body_query(field, &body);
            }
        }
    }
    None
}
