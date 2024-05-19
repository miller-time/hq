use std::{
    error::Error,
    io::{self, Read},
};

use clap::Parser;
use hcl::Body;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(value_name = "filter")]
    filter: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let _args = Args::parse();
    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf)?;
    let body: Body = hcl::from_str(&buf)?;
    println!("HCL from stdin contained:");
    println!(" * {} top-level attribute(s)", body.attributes().count());
    println!(" * {} top-level block(s)", body.blocks().count());
    Ok(())
}
