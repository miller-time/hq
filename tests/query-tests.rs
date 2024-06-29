use std::error::Error;

use hq_rs::{parser::Field, query};

#[test]
fn attr_query_test() -> Result<(), Box<dyn Error>> {
    // filter '.variable'
    let mut fields = vec![Field {
        name: String::from("variable"),
        labels: Vec::new(),
    }];
    let body = utilities::read_test_hcl()?;

    let expected = vec![
        String::from("variable \"my_var\" {\n  default = \"my_default_value\"\n}\n"),
        String::from("variable \"another_var\" {\n  default = \"another_default_value\"\n}\n"),
    ];

    let results: Vec<_> = query(&mut fields, &body)
        .iter()
        .map(|r| r.to_string().unwrap())
        .collect();

    assert_eq!(expected, results);

    Ok(())
}

#[test]
fn label_attr_query_test() -> Result<(), Box<dyn Error>> {
    // filter '.variable[label="my_var"].default'
    let mut fields = vec![
        Field {
            name: String::from("variable"),
            labels: vec![String::from("my_var")],
        },
        Field {
            name: String::from("default"),
            labels: Vec::new(),
        },
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
fn block_query_test() -> Result<(), Box<dyn Error>> {
    // filter '.data'
    let mut fields = vec![Field {
        name: String::from("data"),
        labels: Vec::new(),
    }];
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
fn label_block_query_test() -> Result<(), Box<dyn Error>> {
    // filter '.data[label="another_data_block"]'
    let mut fields = vec![Field {
        name: String::from("data"),
        labels: vec![String::from("another_data_block")],
    }];
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
fn dashed_label_block_query_test() -> Result<(), Box<dyn Error>> {
    // filter '.module[label="cool-module"].version'
    let mut fields = vec![
        Field {
            name: String::from("module"),
            labels: vec![String::from("cool-module")],
        },
        Field {
            name: String::from("version"),
            labels: Vec::new(),
        },
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
