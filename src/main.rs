use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, name = "crypti")]
#[clap(about = "Encoding, decoding, conversion tool", long_about = "None")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(about = "Rotate by (default to ROT13)", arg_required_else_help = true)]
    Rot {
        #[clap(value_parser)]
        string: String,
        #[clap(short, long, value_parser)]
        rotation: Option<u8>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Rot { string, rotation } => {
            let rotation = rotation.unwrap_or(13);

            println!(
                "{}",
                string
                    .chars()
                    .map(|c| match c {
                        'A'..='Z' => {
                            (((c as u8 - 65) + rotation).wrapping_rem_euclid(26) as u8 + 65) as char
                        }
                        'a'..='z' => {
                            (((c as u8 - 97) + rotation).wrapping_rem_euclid(26) as u8 + 97) as char
                        }
                        _ => c as u8 as char,
                    })
                    .collect::<String>()
            )
        }
    }
}
