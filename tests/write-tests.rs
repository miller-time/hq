use std::error::Error;

use hq_rs::{parser::Field, write};

#[test]
fn attr() -> Result<(), Box<dyn Error>> {
    // filter '.version'
    let fields = vec![Field::new("version")];
    // hcl:
    // version = "test"
    let mut body = utilities::edit_hcl("version = \"test\"")?;

    let value: hcl_edit::expr::Expression = "\"new_value\"".parse()?;

    write(fields, &mut body, &value)?;

    assert_eq!("version = \"new_value\"", body.to_string());

    Ok(())
}

#[test]
fn block_attr() -> Result<(), Box<dyn Error>> {
    // filter '.options.enabled'
    let fields = vec![Field::new("options"), Field::new("enabled")];
    // hcl:
    // options { enabled = false }
    let mut body = utilities::edit_hcl("options { enabled = false }")?;

    let value: hcl_edit::expr::Expression = "true".parse()?;

    write(fields, &mut body, &value)?;

    assert_eq!("options { enabled = true }", body.to_string());

    Ok(())
}

#[test]
fn labeled_block_attr() -> Result<(), Box<dyn Error>> {
    // filter '.module["cool-module"].version'
    let fields = vec![
        Field::labeled("module", &["cool-module"]),
        Field::new("version"),
    ];
    // hcl:
    // module "cool-module" { version = "1.0" }
    let mut body = utilities::edit_hcl("module \"cool-module\" { version = \"1.0\" }")?;

    let value: hcl_edit::expr::Expression = "\"2.0\"".parse()?;

    write(fields, &mut body, &value)?;

    assert_eq!(
        "module \"cool-module\" { version = \"2.0\" }",
        body.to_string()
    );

    Ok(())
}
