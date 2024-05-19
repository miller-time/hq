use std::{
    error::Error,
    io::{self, Read},
};

use clap::Parser;
use hcl::Body;
use hq::{lookup::LookupResult, lookup_field, parse_filter};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(value_name = "filter")]
    filter: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf)?;

    let body: Body = hcl::from_str(&buf)?;

    if let Some(filter) = args.filter {
        let mut fields = parse_filter(&filter)?;
        let lookup_result = lookup_field(&mut fields, &body);
        if let Some(lookup_result) = lookup_result {
            // beware `hcl::to_string`!
            // https://github.com/martinohmann/hcl-rs/issues/344
            let s = match lookup_result {
                LookupResult::Expr(expr) => hcl::format::to_string(&expr)?,
                LookupResult::Body(body) => hcl::format::to_string(&body)?,
            };
            println!("{s}");
        }
    } else {
        println!("HCL from stdin contained:");
        println!(" * {} top-level attribute(s)", body.attributes().count());
        println!(" * {} top-level block(s)", body.blocks().count());
    }
    Ok(())
}
