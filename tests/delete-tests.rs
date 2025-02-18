use hq_rs::{delete, parser::Field};

#[test]
fn delete_attr() {
    // filter '.version'
    let fields = vec![Field::new("version")];

    let mut body = utilities::edit_hcl("version = \"test\"").expect("hcl error");

    delete(fields, &mut body).expect("delete error");

    assert_eq!("", body.to_string());
}

#[test]
fn delete_labeled_block() {
    // filter '.module{"cool-module"}'
    let fields = vec![Field::labeled("module", &["cool-module"])];

    let mut body =
        utilities::edit_hcl("module \"cool-module\" { version = \"1.0\" }").expect("hcl error");

    delete(fields, &mut body).expect("delete error");

    assert_eq!("", body.to_string());
}

#[test]
fn delete_labeled_block_attr() {
    // filter '.module{"cool-module"}.version'
    let fields = vec![
        Field::labeled("module", &["cool-module"]),
        Field::new("version"),
    ];

    let mut body =
        utilities::edit_hcl("module \"cool-module\" { version = \"1.0\" }").expect("hcl error");

    delete(fields, &mut body).expect("delete error");

    assert_eq!("module \"cool-module\" {}", body.to_string());
}

#[test]
fn delete_block() {
    // filter '.local'
    let fields = vec![Field::new("local")];

    let mut body = utilities::edit_hcl("local { var = 5 }").expect("hcl error");

    delete(fields, &mut body).expect("delete error");

    assert_eq!("", body.to_string());
}

#[test]
fn delete_block_attr() {
    // filter '.local.var'
    let fields = vec![Field::new("local"), Field::new("var")];

    let mut body = utilities::edit_hcl("local { var = 5 }").expect("hcl error");

    delete(fields, &mut body).expect("delete error");

    assert_eq!("local {}", body.to_string());
}

#[test]
fn delete_from_object() {
    // filter '.local.obj.val'
    let fields = vec![Field::new("local"), Field::new("obj"), Field::new("val")];

    let mut body = utilities::edit_hcl("local { obj = { val = 5 } }").expect("hcl error");

    delete(fields, &mut body).expect("delete error");

    assert_eq!("local { obj = {} }", body.to_string());
}

#[test]
fn delete_from_nested_object() {
    // filter '.local.obj.obj2.val'
    let fields = vec![
        Field::new("local"),
        Field::new("obj"),
        Field::new("obj2"),
        Field::new("val"),
    ];

    let mut body =
        utilities::edit_hcl("local { obj = { obj2 = { val = 5 } } }").expect("hcl error");

    delete(fields, &mut body).expect("delete error");

    assert_eq!("local { obj = { obj2 = {} } }", body.to_string());
}
