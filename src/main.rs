use std::{
    error::Error,
    io::{self, Read},
};

use clap::Parser;
use hcl::{Body, Expression, Identifier, ObjectKey};
use hq::parse_filter;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(value_name = "filter")]
    filter: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf)?;

    let body: Body = hcl::from_str(&buf)?;

    if let Some(filter) = args.filter {
        let mut fields = parse_filter(&filter)?;
        if fields.is_empty() {
            unreachable!();
        }
        let field = fields.remove(0);
        let mut expr = body_lookup(&field, &body);

        while !fields.is_empty() {
            let field = fields.remove(0);
            expr = expr_lookup(&field, &expr);
        }

        if let Some(expr) = expr {
            // beware `hcl::to_string`!
            // https://github.com/martinohmann/hcl-rs/issues/344
            let expr: String = hcl::format::to_string(&expr)?;
            println!("{expr}");
        }
    } else {
        println!("HCL from stdin contained:");
        println!(" * {} top-level attribute(s)", body.attributes().count());
        println!(" * {} top-level block(s)", body.blocks().count());
    }
    Ok(())
}

fn body_lookup(field: &str, body: &Body) -> Option<Expression> {
    for attr in body.attributes() {
        if attr.key() == field {
            return Some(attr.expr().clone());
        }
    }
    None
}

fn expr_lookup(field: &str, expr: &Option<Expression>) -> Option<Expression> {
    if expr.is_none() {
        return None;
    }
    if let Expression::Object(object) = expr.as_ref().unwrap() {
        let key = ObjectKey::Identifier(Identifier::new(field).unwrap());
        if let Some(expr) = object.get(&key) {
            return Some(expr.clone());
        }
    }
    None
}
