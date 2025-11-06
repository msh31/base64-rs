use clap::{Parser, Subcommand};

// compiler writes this code
// tells the compiler to automatically implement the pparser trait* for this struct
// *a trait defines behavior that types can implement, like interfaces or virtual base classes
#[derive(Parser)]
#[command(name = "base64-rs")]
#[command(about = "Encode and decode base64")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode { text: String },
    Decode { text: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { text } => {
            println!("TODO: encode '{}'", text);
        }
        Commands::Decode { text } => {
            println!("TODO: decode '{}'", text);
        }
    }
}

fn encode() {
    //
}

fn decode() {
    //
}
