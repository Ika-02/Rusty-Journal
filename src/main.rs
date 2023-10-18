mod cli;
use structopt::StructOpt;


fn main() {
    println!("{:#?}", cli::CliOptions::from_args());
}
