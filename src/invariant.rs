use clap::Parser;

#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[arg(short, long)]
    size: u32,
    #[arg(short, long)]
    degree: u32,
}

fn main() {
    let cli = Args::parse();

    println!("{cli:#?}")
}