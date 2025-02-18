use hq_rs::{parser::Field, query};

#[test]
fn scalar_attr() {
    // filter '.version'
    let mut fields = vec![Field::new("version")];
    let body = utilities::read_test_hcl().expect("hcl error");

    let expected = vec![String::from("\"test\"")];

    let results: Vec<_> = query(&mut fields, &body)
        .iter()
        .map(|r| r.to_string().unwrap())
        .collect();

    assert_eq!(expected, results);
}

#[test]
fn obj_attr() {
    // filter '.options'
    let mut fields = vec![Field::new("options")];
    let body = utilities::read_test_hcl().expect("hcl error");

    let expected = vec![String::from("{\n  verbose = true\n  debug = false\n}")];

    let results: Vec<_> = query(&mut fields, &body)
        .iter()
        .map(|r| r.to_string().unwrap())
        .collect();

    assert_eq!(expected, results);
}

#[test]
fn block_attr() {
    // filter '.variable.default'
    let mut fields = vec![Field::new("variable"), Field::new("default")];
    let body = utilities::read_test_hcl().expect("hcl error");

    let expected = vec![
        String::from("\"my_default_value\""),
        String::from("\"another_default_value\""),
    ];

    let results: Vec<_> = query(&mut fields, &body)
        .iter()
        .map(|r| r.to_string().unwrap())
        .collect();

    assert_eq!(expected, results);
}

#[test]
fn labeled_block_attr() {
    // filter '.variable{"my_var"}.default'
    let mut fields = vec![
        Field::labeled("variable", &["my_var"]),
        Field::new("default"),
    ];
    let body = utilities::read_test_hcl().expect("hcl error");

    let expected = vec![String::from("\"my_default_value\"")];

    let results: Vec<_> = query(&mut fields, &body)
        .iter()
        .map(|r| r.to_string().unwrap())
        .collect();

    assert_eq!(expected, results);
}

#[test]
fn block() {
    // filter '.data'
    let mut fields = vec![Field::new("data")];
    let body = utilities::read_test_hcl().expect("hcl error");

    let expected = vec![
        String::from("data \"a_data_block\" \"with_some_attrs\" {\n  my_attr = \"my_attr_value\"\n  another_attr = \"another_attr_value\"\n}\n"),
        String::from("data \"another_data_block\" \"with_some_attrs\" {\n  cromulent_attr = \"cromulent_value\"\n}\n"),
    ];

    let results: Vec<_> = query(&mut fields, &body)
        .iter()
        .map(|r| r.to_string().unwrap())
        .collect();

    assert_eq!(expected, results);
}

#[test]
fn labeled_block() {
    // filter '.data{"another_data_block"}'
    let mut fields = vec![Field::labeled("data", &["another_data_block"])];
    let body = utilities::read_test_hcl().expect("hcl error");

    let expected = vec![String::from("data \"another_data_block\" \"with_some_attrs\" {\n  cromulent_attr = \"cromulent_value\"\n}\n")];

    let results: Vec<_> = query(&mut fields, &body)
        .iter()
        .map(|r| r.to_string().unwrap())
        .collect();

    assert_eq!(expected, results);
}

#[test]
fn dash_labeled_block() {
    // filter '.module{"cool-module"}.version'
    let mut fields = vec![
        Field::labeled("module", &["cool-module"]),
        Field::new("version"),
    ];
    let body = utilities::read_test_hcl().expect("hcl error");

    let expected = vec![String::from("\"1.2.3\"")];

    let results: Vec<_> = query(&mut fields, &body)
        .iter()
        .map(|r| r.to_string().unwrap())
        .collect();

    assert_eq!(expected, results);
}
