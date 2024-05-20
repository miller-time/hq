use hcl::{Body, Expression, Identifier, ObjectKey};

pub enum QueryResult {
    Expr(Expression),
    Body(Body),
}

pub fn query(fields: &mut Vec<String>, body: &Body) -> Vec<QueryResult> {
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

fn body_query(field: &str, body: &Body) -> Vec<QueryResult> {
    let mut matches = Vec::new();
    let mut attr_matches = attr_query(field, body);
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

fn block_query(field: &str, body: &Body) -> Vec<QueryResult> {
    let mut matches = Vec::new();
    for block in body.blocks() {
        if block.identifier() == field {
            matches.push(QueryResult::Body(block.body().clone()));
        }
    }
    matches
}

fn result_query(field: &str, query_results: Vec<QueryResult>) -> Vec<QueryResult> {
    let mut matches = Vec::new();
    for query_result in query_results {
        match query_result {
            QueryResult::Expr(expr) => {
                if let Expression::Object(object) = expr {
                    let key = ObjectKey::Identifier(Identifier::new(field).unwrap());
                    if let Some(expr) = object.get(&key) {
                        matches.push(QueryResult::Expr(expr.clone()));
                    }
                }
            }
            QueryResult::Body(body) => {
                let mut body_matches = body_query(field, &body);
                matches.append(&mut body_matches);
            }
        }
    }
    matches
}
