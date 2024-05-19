use std::error::Error;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "filter/grammar.pest"]
pub struct Filter {}

pub fn parse_filter(input: &str) -> Result<String, Box<dyn Error>> {
    let pairs = Filter::parse(Rule::filter, input)?;
    for pair in pairs {
        if let Rule::text = pair.as_rule() {
            return Ok(pair.as_str().to_owned());
        }
    }
    Err("malformed filter".into())
}
