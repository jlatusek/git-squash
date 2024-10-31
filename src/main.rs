use clap::Parser;


/// Squash current branch like GitHub squash merge
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of branch fromt whom squash
    branch_name: String,
}

fn main() {
    let args = Args::parse();
    println!("args: {:?}", args);
}
