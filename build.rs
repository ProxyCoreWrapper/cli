use clap::{ValueEnum, CommandFactory};
use clap_complete::{generate_to, Shell};

include!("src/cli.rs");

fn main() -> std::io::Result<()> {
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut Cli::command(), "pcw-cli", "autocompletion")?;
    }

    tonic_prost_build::compile_protos("proto-files/backend.proto")
}