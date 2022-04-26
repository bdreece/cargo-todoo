use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    name = "cargo_todoo",
    version = "0.1",
    author = "Brian Reece",
    about = "A todo comment aggregator for Cargo"
)]
pub struct Args {
    #[clap(short, long, parse(from_occurrences), help = "Enable verbose output")]
    pub verbose: usize,

    #[clap(short, long, help = "Ignore comments matching regex string")]
    pub ignore_regex: Option<String>,

    #[clap(short, long, help = "Skip source files matching regex string")]
    pub skip_regex: Option<String>,

    #[clap(parse(from_os_str), default_value("."), help = "Path to crate root")]
    pub path: PathBuf,
    // TODOO: Add additional CLI flags
}
