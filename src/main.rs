use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {}

fn main() {
    let _args = Args::parse();
}
