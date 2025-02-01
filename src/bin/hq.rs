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
    #[arg(value_name = "FILTER", help = "HCL filter expression")]
    filter: Option<String>,

    #[clap(
        short = 'f',
        long = "file",
        value_name = "FILE",
        help = "HCL file to read from"
    )]
    file: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Read value from HCL (default)")]
    Read {
        #[clap(
            short = 'f',
            long = "file",
            value_name = "FILE",
            help = "HCL file to read from"
        )]
        file: Option<String>,

        #[arg(value_name = "FILTER", help = "HCL filter expression")]
        filter: Option<String>,
    },
    #[command(about = "Write value into HCL")]
    Write {
        #[clap(
            short = 'f',
            long = "file",
            value_name = "FILE",
            help = "HCL file to read from"
        )]
        file: Option<String>,

        #[clap(
            short = 'i',
            long = "inline",
            requires = "file",
            help = "Write to HCL file inline instead of stdout (--file must also be set)"
        )]
        inline: bool,

        #[arg(required = true, help = "HCL write expression (<FILTER>=<VALUE>)")]
        expr: String,
    },
    #[command(about = "Remove a value from HCL")]
    Delete {
        #[clap(
            short = 'f',
            long = "file",
            value_name = "FILE",
            help = "HCL file to read from"
        )]
        file: Option<String>,

        #[clap(
            short = 'i',
            long = "inline",
            requires = "file",
            help = "Modify HCL file inline instead of stdout (--file must also be set"
        )]
        inline: bool,

        #[arg(value_name = "FILTER", help = "HCL filter expression")]
        filter: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        None => {
            read(args.filter, args.file)?;
        }
        Some(Command::Read { file, filter }) => {
            read(file, filter)?;
        }
        Some(Command::Write { file, inline, expr }) => {
            write(file, inline, expr)?;
        }
        Some(Command::Delete {
            file,
            inline,
            filter,
        }) => {
            delete(file, inline, filter)?;
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

fn read(file: Option<String>, filter: Option<String>) -> Result<(), Box<dyn Error>> {
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

fn write(file: Option<String>, inline: bool, expr: String) -> Result<(), Box<dyn Error>> {
    let contents = match file {
        Some(ref file) => fs::read_to_string(file)?,
        None => read_stdin()?,
    };
    let mut body: hcl_edit::structure::Body = contents.parse()?;
    if !expr.contains('=') {
        return Err("write expression should be <FILTER>=<VALUE>".into());
    }
    let parts: Vec<_> = expr.split('=').collect();
    if parts.len() != 2 {
        return Err("write expression should be <FILTER>=<VALUE>".into());
    }
    let filter = parts[0];
    let new_value = parts[1];
    let expr: hcl_edit::expr::Expression = new_value.parse()?;
    let fields = hq_rs::parse_filter(filter)?;
    hq_rs::write(fields, &mut body, &expr)?;

    if inline {
        // When inline is set, write the modified HCL back to the file
        // file cannot be none here since --inline requires --file
        fs::write(file.unwrap(), body.to_string())?;
    } else {
        // Otherwise, write to stdout
        print!("{body}");
        io::stdout().flush()?;
    }

    Ok(())
}

fn delete(file: Option<String>, inline: bool, filter: String) -> Result<(), Box<dyn Error>> {
    let contents = match file {
        Some(ref file) => fs::read_to_string(file)?,
        None => read_stdin()?,
    };
    let mut body: hcl_edit::structure::Body = contents.parse()?;
    let fields = hq_rs::parse_filter(&filter)?;
    hq_rs::delete(fields, &mut body)?;

    if inline {
        // When inline is set, write the modified HCL back to the file
        // file cannot be none here since --inline requires --file
        fs::write(file.unwrap(), body.to_string())?;
    } else {
        // Otherwise, write to stdout
        print!("{body}");
        io::stdout().flush()?;
    }

    Ok(())
}
