use std::error::Error;

use hcl::{Attribute, Body, Expression, Identifier, ObjectKey};

use crate::parser::Field;

pub fn write(
    fields: Vec<Field>,
    body: &mut Body,
    value: &Expression,
) -> Result<(), Box<dyn Error>> {
    if fields.is_empty() {
        // our grammar/parser for filters won't allow an empty filter
        unreachable!();
    }

    write_body(fields, body, value)?;

    Ok(())
}

fn write_body(
    mut fields: Vec<Field>,
    body: &mut Body,
    value: &Expression,
) -> Result<bool, Box<dyn Error>> {
    let field = fields.remove(0);

    let mut matched = false;

    for attr in body.attributes_mut() {
        if attr.key() == field.name {
            if fields.is_empty() {
                // we are done!
                attr.expr = value.clone();
                matched = true;
                continue;
            }
            // dive! dive!
            let expr_matched = write_expr(fields.clone(), &mut attr.expr, value);
            if expr_matched {
                matched = true;
            }
        }
    }

    for block in body.blocks_mut() {
        if block.identifier() != field.name {
            continue;
        }
        if field.labels.is_empty() {
            if fields.is_empty() {
                return Err("unable to write expr as block body".into());
            }
            let block_matched = write_body(fields.clone(), &mut block.body, value)?;
            if block_matched {
                matched = true;
            }
        }
        for filter_label in &field.labels {
            for block_label in block.labels.clone() {
                if block_label.as_str() == filter_label {
                    let block_matched = write_body(fields.clone(), &mut block.body, value)?;
                    if block_matched {
                        matched = true;
                    }
                }
            }
        }
    }

    if !matched {
        let new_attr = Attribute::new(field.name, value.clone());
        let new_body = Body::builder().add_attribute(new_attr).build();
        body.extend(new_body);
        matched = true;
    }

    Ok(matched)
}

fn write_expr(mut fields: Vec<Field>, expr: &mut Expression, value: &Expression) -> bool {
    let field = fields.remove(0);
    if let Expression::Object(object) = expr {
        let key = ObjectKey::Identifier(Identifier::new(field.name).unwrap());
        if fields.is_empty() {
            // we are done!
            object.insert(key, value.clone());
            return true;
        }
        if let Some(expr) = object.get_mut(&key) {
            return write_expr(fields.clone(), expr, value);
        }
    }

    false
}
