use std::error::Error;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "filter/grammar.pest"]
pub struct Filter {}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub labels: Vec<String>,
}

pub fn parse_filter(input: &str) -> Result<Vec<Field>, Box<dyn Error>> {
    let mut fields = Vec::new();
    let pairs = Filter::parse(Rule::filter, input)?;
    for pair in pairs {
        let mut name = String::new();
        let mut labels = Vec::new();

        let inner_pairs = pair.into_inner();
        for inner in inner_pairs {
            match inner.as_rule() {
                Rule::name => {
                    // according to clippy, this is a more efficient way of doing
                    // `name = inner.as_str().to_owned()`
                    inner.as_str().clone_into(&mut name);
                }
                Rule::label => {
                    labels.push(inner.as_str().to_owned());
                }
                _ => {}
            }
        }
        if !name.is_empty() {
            fields.push(Field { name, labels });
        }
    }
    Ok(fields)
}
