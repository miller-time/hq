use std::error::Error;

use hq_rs::{delete, parser::Field};

#[test]
fn delete_attr() -> Result<(), Box<dyn Error>> {
    // filter '.version'
    let fields = vec![Field::new("version")];

    let mut body = utilities::edit_hcl("version = \"test\"")?;

    delete(fields, &mut body)?;

    assert_eq!("", body.to_string());

    Ok(())
}

#[test]
fn delete_labeled_block() -> Result<(), Box<dyn Error>> {
    // filter '.module{"cool-module"}'
    let fields = vec![Field::labeled("module", &["cool-module"])];

    let mut body = utilities::edit_hcl("module \"cool-module\" { version = \"1.0\" }")?;

    delete(fields, &mut body)?;

    assert_eq!("", body.to_string());
    Ok(())
}

#[test]
fn delete_labeled_block_attr() -> Result<(), Box<dyn Error>> {
    // filter '.module{"cool-module"}.version'
    let fields = vec![
        Field::labeled("module", &["cool-module"]),
        Field::new("version"),
    ];

    let mut body = utilities::edit_hcl("module \"cool-module\" { version = \"1.0\" }")?;

    delete(fields, &mut body)?;

    assert_eq!("module \"cool-module\" {}", body.to_string());
    Ok(())
}

#[test]
fn delete_block() -> Result<(), Box<dyn Error>> {
    // filter '.local'
    let fields = vec![Field::new("local")];

    let mut body = utilities::edit_hcl("local { var = 5 }")?;

    delete(fields, &mut body)?;

    assert_eq!("", body.to_string());
    Ok(())
}

#[test]
fn delete_block_attr() -> Result<(), Box<dyn Error>> {
    // filter '.local.var'
    let fields = vec![Field::new("local"), Field::new("var")];

    let mut body = utilities::edit_hcl("local { var = 5 }")?;

    delete(fields, &mut body)?;

    assert_eq!("local {}", body.to_string());
    Ok(())
}

#[test]
fn delete_from_object() -> Result<(), Box<dyn Error>> {
    // filter '.local.obj.val'
    let fields = vec![Field::new("local"), Field::new("obj"), Field::new("val")];

    let mut body = utilities::edit_hcl("local { obj = { val = 5 } }")?;

    delete(fields, &mut body)?;

    assert_eq!("local { obj = {} }", body.to_string());
    Ok(())
}

#[test]
fn delete_from_nested_object() -> Result<(), Box<dyn Error>> {
    // filter '.local.obj.obj2.val'
    let fields = vec![Field::new("local"), Field::new("obj"), Field::new("obj2"), Field::new("val")];

    let mut body = utilities::edit_hcl("local { obj = { obj2 = { val = 5 } } }")?;

    delete(fields, &mut body)?;

    assert_eq!("local { obj = { obj2 = {} } }", body.to_string());
    Ok(())
}
