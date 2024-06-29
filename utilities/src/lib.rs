use std::{error::Error, fs};

pub fn read_test_hcl() -> Result<hcl::Body, Box<dyn Error>> {
    let contents = fs::read_to_string("tests/test.tf")?;
    let body: hcl::Body = hcl::from_str(&contents)?;
    Ok(body)
}

pub fn edit_hcl(contents: &str) -> Result<hcl_edit::structure::Body, Box<dyn Error>> {
    let body: hcl_edit::structure::Body = contents.parse()?;
    Ok(body)
}
