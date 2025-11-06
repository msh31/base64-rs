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

const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

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

// -> {type} indicates we return as that
fn encode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        // chunk is &[u8], could be 1, 2, or 3 bytes

        if chunk.len() == 3 {
            // extract 4 indices, push 4 chars to result
            // result.push(some_char);
        }
    }
}

fn decode(input: &str) -> String {
    // TODO: implement
}
