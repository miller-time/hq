use pest::Parser;
use pest_derive::Parser;

use super::error::FilterError;

#[derive(Parser)]
#[grammar = "filter/grammar.pest"]
pub struct Filter {}

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub labels: Vec<String>,
}

pub fn parse_filter(input: &str) -> Result<Vec<Field>, Box<FilterError<Rule>>> {
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

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn name_filter() -> Result<(), Box<dyn Error>> {
        let input = ".a_name";
        let expected = vec![Field {
            name: String::from("a_name"),
            labels: Vec::new(),
        }];
        let fields = parse_filter(input)?;
        assert_eq!(expected, fields);
        Ok(())
    }

    #[test]
    fn label_filter() -> Result<(), Box<dyn Error>> {
        let input = ".a_name[label=\"a_label\"]";
        let expected = vec![Field {
            name: String::from("a_name"),
            labels: vec![String::from("a_label")],
        }];
        let fields = parse_filter(input)?;
        assert_eq!(expected, fields);
        Ok(())
    }

    #[test]
    fn traversal_filter() -> Result<(), Box<dyn Error>> {
        let input = ".a_name[label=\"a_label\"].another_name[label=\"another_label\"].third_name";
        let expected = vec![
            Field {
                name: String::from("a_name"),
                labels: vec![String::from("a_label")],
            },
            Field {
                name: String::from("another_name"),
                labels: vec![String::from("another_label")],
            },
            Field {
                name: String::from("third_name"),
                labels: Vec::new(),
            },
        ];
        let fields = parse_filter(input)?;
        assert_eq!(expected, fields);
        Ok(())
    }
}
