use std::error::Error;

use hq_rs::{parser::Field, query};

#[test]
fn block_attr() -> Result<(), Box<dyn Error>> {
    // filter '.variable.default'
    let mut fields = vec![Field::new("variable"), Field::new("default")];
    let body = utilities::read_test_hcl()?;

    let expected = vec![
        String::from("\"my_default_value\""),
        String::from("\"another_default_value\""),
    ];

    let results: Vec<_> = query(&mut fields, &body)
        .iter()
        .map(|r| r.to_string().unwrap())
        .collect();

    assert_eq!(expected, results);

    Ok(())
}

#[test]
fn labeled_block_attr() -> Result<(), Box<dyn Error>> {
    // filter '.variable[label="my_var"].default'
    let mut fields = vec![
        Field::labeled("variable", &["my_var"]),
        Field::new("default"),
    ];
    let body = utilities::read_test_hcl()?;

    let expected = vec![String::from("\"my_default_value\"")];

    let results: Vec<_> = query(&mut fields, &body)
        .iter()
        .map(|r| r.to_string().unwrap())
        .collect();

    assert_eq!(expected, results);

    Ok(())
}

#[test]
fn block() -> Result<(), Box<dyn Error>> {
    // filter '.data'
    let mut fields = vec![Field::new("data")];
    let body = utilities::read_test_hcl()?;

    let expected = vec![
        String::from("data \"a_data_block\" \"with_some_attrs\" {\n  my_attr = \"my_attr_value\"\n  another_attr = \"another_attr_value\"\n}\n"),
        String::from("data \"another_data_block\" \"with_some_attrs\" {\n  cromulent_attr = \"cromulent_value\"\n}\n"),
    ];

    let results: Vec<_> = query(&mut fields, &body)
        .iter()
        .map(|r| r.to_string().unwrap())
        .collect();

    assert_eq!(expected, results);

    Ok(())
}

#[test]
fn labeled_block() -> Result<(), Box<dyn Error>> {
    // filter '.data[label="another_data_block"]'
    let mut fields = vec![Field::labeled("data", &["another_data_block"])];
    let body = utilities::read_test_hcl()?;

    let expected = vec![String::from("data \"another_data_block\" \"with_some_attrs\" {\n  cromulent_attr = \"cromulent_value\"\n}\n")];

    let results: Vec<_> = query(&mut fields, &body)
        .iter()
        .map(|r| r.to_string().unwrap())
        .collect();

    assert_eq!(expected, results);

    Ok(())
}

#[test]
fn dash_labeled_block() -> Result<(), Box<dyn Error>> {
    // filter '.module[label="cool-module"].version'
    let mut fields = vec![
        Field::labeled("module", &["cool-module"]),
        Field::new("version"),
    ];
    let body = utilities::read_test_hcl()?;

    let expected = vec![String::from("\"1.2.3\"")];

    let results: Vec<_> = query(&mut fields, &body)
        .iter()
        .map(|r| r.to_string().unwrap())
        .collect();

    assert_eq!(expected, results);

    Ok(())
}
