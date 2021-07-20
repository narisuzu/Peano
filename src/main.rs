use clap::Clap;
use peano::Opts;

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
