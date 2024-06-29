use std::{
    error::Error,
    fs,
    io::{self, Read, Write},
};

use clap::{Parser, Subcommand};
use hq_rs::{parse_filter, query, write};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[clap(short, long, value_name = "FILE", help = "HCL file to read from")]
    file: Option<String>,

    #[arg(value_name = "filter", help = "HCL filter expression")]
    filter: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Read value from HCL (default)")]
    Read,
    #[command(about = "Write value into HCL")]
    Write {
        #[arg(required = true, help = "Value to write into HCL")]
        value: String,
    },
}

impl Default for Command {
    fn default() -> Self {
        Self::Read
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Read the HCL from either a file or stdin
    let buf = if let Some(file_path) = args.file {
        fs::read_to_string(file_path)?
    } else {
        let mut stdin = io::stdin();
        let mut buf = String::new();
        stdin.read_to_string(&mut buf)?;
        buf
    };

    match args.command {
        None | Some(Command::Read) => {
            let body: hcl::Body = hcl::from_str(&buf)?;

            if let Some(filter) = args.filter {
                let mut fields = parse_filter(&filter)?;
                let query_results = query(&mut fields, &body);
                for query_result in query_results {
                    let s = query_result.to_string()?;
                    print!("{s}");
                    io::stdout().flush()?;
                }
            } else {
                println!("HCL from stdin contained:");
                println!(" * {} top-level attribute(s)", body.attributes().count());
                println!(" * {} top-level block(s)", body.blocks().count());
            }
        }
        Some(Command::Write { value }) => {
            let mut body: hcl_edit::structure::Body = buf.parse()?;

            let expr: hcl_edit::expr::Expression = value.parse()?;
            if let Some(filter) = args.filter {
                let fields = parse_filter(&filter)?;
                write(fields, &mut body, &expr)?;
                print!("{body}");
                io::stdout().flush()?;
            } else {
                print!("{expr}");
                io::stdout().flush()?;
            }
        }
    }

    Ok(())
}
