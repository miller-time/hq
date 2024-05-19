use hcl::{Body, Expression, Identifier, ObjectKey};

pub enum LookupResult {
    Expr(Expression),
    Body(Body),
}

pub fn lookup_field(fields: &mut Vec<String>, body: &Body) -> Option<LookupResult> {
    if fields.is_empty() {
        // our grammar/parser for filters won't allow an empty filter
        unreachable!();
    }

    // take the first field and do a `Body` lookup
    // e.g. `.foo.bar` will start with 'foo'
    let field = fields.remove(0);
    let mut lookup_result = body_lookup(&field, body);

    // iteratively evaluate each subsequent field
    // e.g. having handled 'foo' we move on to 'bar'
    while !fields.is_empty() {
        let field = fields.remove(0);
        lookup_result = result_lookup(&field, lookup_result);
    }

    lookup_result
}

fn body_lookup(field: &str, body: &Body) -> Option<LookupResult> {
    if let Some(value) = attr_lookup(field, body) {
        return Some(value);
    }
    block_lookup(field, body)
}

fn attr_lookup(field: &str, body: &Body) -> Option<LookupResult> {
    for attr in body.attributes() {
        if attr.key() == field {
            return Some(LookupResult::Expr(attr.expr().clone()));
        }
    }
    None
}

fn block_lookup(field: &str, body: &Body) -> Option<LookupResult> {
    for block in body.blocks() {
        if block.identifier() == field {
            return Some(LookupResult::Body(block.body().clone()));
        }
    }
    None
}

fn result_lookup(field: &str, lookup_result: Option<LookupResult>) -> Option<LookupResult> {
    if let Some(lookup_result) = lookup_result {
        match lookup_result {
            LookupResult::Expr(expr) => {
                if let Expression::Object(object) = expr {
                    let key = ObjectKey::Identifier(Identifier::new(field).unwrap());
                    if let Some(expr) = object.get(&key) {
                        return Some(LookupResult::Expr(expr.clone()));
                    }
                }
            }
            LookupResult::Body(body) => {
                return body_lookup(field, &body);
            }
        }
    }
    None
}
