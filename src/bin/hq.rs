use std::{
    error::Error,
    fs,
    io::{self, Read, Write},
};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    // the `Read` options are duplicated here because when no command is given
    // then the `read` command is the default and its options come from the root
    #[arg(value_name = "filter", help = "HCL filter expression")]
    filter: Option<String>,

    #[clap(short, long, value_name = "FILE", help = "HCL file to read from")]
    file: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Read value from HCL (default)")]
    Read {
        #[arg(value_name = "filter", help = "HCL filter expression")]
        filter: Option<String>,

        #[clap(short, long, value_name = "FILE", help = "HCL file to read from")]
        file: Option<String>,
    },
    #[command(about = "Write value into HCL")]
    Write {
        #[arg(value_name = "filter", help = "HCL filter expression")]
        filter: Option<String>,

        #[clap(short, long, value_name = "FILE", help = "HCL file to read from")]
        file: Option<String>,

        #[arg(required = true, help = "Value to write into HCL")]
        value: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        None => {
            read(args.filter, args.file)?;
        }
        Some(Command::Read { filter, file }) => {
            read(filter, file)?;
        }
        Some(Command::Write {
            filter,
            file,
            value,
        }) => {
            write(filter, file, value)?;
        }
    }

    Ok(())
}

fn read_stdin() -> Result<String, Box<dyn Error>> {
    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf)?;
    Ok(buf)
}

fn read(filter: Option<String>, file: Option<String>) -> Result<(), Box<dyn Error>> {
    let contents = match file {
        Some(file) => fs::read_to_string(file)?,
        None => read_stdin()?,
    };
    let body: hcl::Body = hcl::from_str(&contents)?;
    match filter {
        Some(filter) => {
            let mut fields = hq_rs::parse_filter(&filter)?;
            let query_results = hq_rs::query(&mut fields, &body);
            for query_result in query_results {
                let s = query_result.to_string()?;
                print!("{s}");
                io::stdout().flush()?;
                if !s.ends_with('\n') {
                    println!();
                }
            }
        }
        None => {
            println!("HCL from stdin contained:");
            println!(" * {} top-level attribute(s)", body.attributes().count());
            println!(" * {} top-level block(s)", body.blocks().count());
        }
    }
    Ok(())
}

fn write(
    filter: Option<String>,
    file: Option<String>,
    value: String,
) -> Result<(), Box<dyn Error>> {
    let contents = match file {
        Some(file) => fs::read_to_string(file)?,
        None => read_stdin()?,
    };
    let mut body: hcl_edit::structure::Body = contents.parse()?;
    let expr: hcl_edit::expr::Expression = value.parse()?;
    match filter {
        Some(filter) => {
            let fields = hq_rs::parse_filter(&filter)?;
            hq_rs::write(fields, &mut body, &expr)?;
            print!("{body}");
            io::stdout().flush()?;
        }
        None => {
            print!("{expr}");
            io::stdout().flush()?;
        }
    }
    Ok(())
}
