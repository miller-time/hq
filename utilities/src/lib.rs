use std::{error::Error, fs};

pub fn read_test_hcl() -> Result<hcl::Body, Box<dyn Error>> {
    let contents = fs::read_to_string("tests/test.tf")?;
    let body: hcl::Body = hcl::from_str(&contents)?;
    Ok(body)
}
