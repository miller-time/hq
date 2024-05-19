use std::error::Error;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "filter/grammar.pest"]
pub struct Filter {}

pub fn parse_filter(input: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut names = Vec::new();
    let pairs = Filter::parse(Rule::filter, input)?;
    for pair in pairs {
        if let Rule::name = pair.as_rule() {
            names.push(pair.as_str().to_owned());
        }
    }
    Ok(names)
}
