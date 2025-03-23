use hq_rs::{parser::Field, write};

#[test]
fn attr() {
    // filter '.version'
    let fields = vec![Field::new("version")];

    let mut body = utilities::edit_hcl("version = \"test\"").expect("hcl error");

    let value: hcl_edit::expr::Expression = "\"new_value\"".parse().expect("parse error");

    write(fields, &mut body, &value);

    assert_eq!("version = \"new_value\"", body.to_string());
}

#[test]
fn block_attr() {
    // filter '.options.enabled'
    let fields = vec![Field::new("options"), Field::new("enabled")];

    let mut body = utilities::edit_hcl("options { enabled = false }").expect("hcl error");

    let value: hcl_edit::expr::Expression = "true".parse().expect("parse error");

    write(fields, &mut body, &value);

    assert_eq!("options { enabled = true }", body.to_string());
}

#[test]
fn labeled_block_attr() {
    // filter '.module{"cool-module"}.version'
    let fields = vec![
        Field::labeled("module", &["cool-module"]),
        Field::new("version"),
    ];

    let mut body =
        utilities::edit_hcl("module \"cool-module\" { version = \"1.0\" }").expect("hcl error");

    let value: hcl_edit::expr::Expression = "\"2.0\"".parse().expect("parse error");

    write(fields, &mut body, &value);

    assert_eq!(
        "module \"cool-module\" { version = \"2.0\" }",
        body.to_string()
    );
}

#[test]
fn insert() {
    // filter '.options.new_attr'
    let fields = vec![Field::new("options"), Field::new("new_attr")];

    let mut body = utilities::edit_hcl("options { attr = \"value\" }").expect("hcl error");

    let value: hcl_edit::expr::Expression = "\"new_value\"".parse().expect("parse error");

    write(fields, &mut body, &value);

    assert_eq!(
        "options {\n attr = \"value\" \nnew_attr = \"new_value\"\n}",
        body.to_string()
    );
}
