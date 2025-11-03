use clap::Parser;

// compilers writes this code
// the derive for debubg, auto gens fmt::debug implementation for this struct
// parser, same thing for passing in the arguments we want, defined in a struct
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)] //short = n, long = name, uses the variable name
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
