use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(value_name = "filter")]
    filter: Option<String>,
}

fn main() {
    let args = Args::parse();
    if let Some(filter) = args.filter {
        println!("filter: {filter}");
    }
}
