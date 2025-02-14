use std::error::Error;

use hq_rs::{delete, parser::Field};

#[test]
fn delete_attr() -> Result<(), Box<dyn Error>> {
    let fields = vec![Field::new("version")];

    // hcl:
    // version = "test"
    let mut body = utilities::edit_hcl("version = \"test\"")?;

    delete(fields, &mut body)?;

    assert_eq!("", body.to_string());

    Ok(())
}

#[test]
fn delete_labeled_block() -> Result<(), Box<dyn Error>> {
    let fields = vec![Field::labeled("module", &["cool-module"])];

    // hcl:
    // module "cool-module" { version = "1.0" }
    let mut body = utilities::edit_hcl("module \"cool-module\" { version = \"1.0\" }")?;

    delete(fields, &mut body)?;

    assert_eq!("", body.to_string());
    Ok(())
}

#[test]
fn delete_labeled_block_attr() -> Result<(), Box<dyn Error>> {
    let fields = vec![
        Field::labeled("module", &["cool-module"]),
        Field::new("version"),
    ];

    // hcl:
    // module "cool-module" { version = "1.0" }
    let mut body = utilities::edit_hcl("module \"cool-module\" { version = \"1.0\" }")?;

    delete(fields, &mut body)?;

    assert_eq!("module \"cool-module\" {}", body.to_string());
    Ok(())
}

#[test]
fn delete_block() -> Result<(), Box<dyn Error>> {
    let fields = vec![Field::new("local")];

    // hcl:
    // local { var = 5 }
    let mut body = utilities::edit_hcl("local { var = 5 }")?;

    delete(fields, &mut body)?;

    assert_eq!("", body.to_string());
    Ok(())
}

#[test]
fn delete_block_attr() -> Result<(), Box<dyn Error>> {
    let fields = vec![Field::new("local"), Field::new("var")];

    // hcl:
    // local { var = 5 }
    let mut body = utilities::edit_hcl("local { var = 5 }")?;

    delete(fields, &mut body)?;

    assert_eq!("local {}", body.to_string());
    Ok(())
}

#[ignore] // Deletion from an object is not yet implement
#[test]
fn delete_from_object() -> Result<(), Box<dyn Error>> {
    let fields = vec![Field::new("local"), Field::new("obj"), Field::new("val")];

    // hcl:
    // local { obj = { val = 5 } }
    let mut body = utilities::edit_hcl("local { obj = { val = 5 } }")?;

    delete(fields, &mut body)?;

    assert_eq!("local { obj = {} }", body.to_string());
    Ok(())
}
