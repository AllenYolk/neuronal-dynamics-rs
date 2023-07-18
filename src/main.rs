use std::process::exit;
use clap::Parser;
use neuronal_dynamics_rs::NeuronType;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[command(subcommand)]
    neuron_type: NeuronType,

    #[arg(short, long)]
    result_folder: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Err(msg) = neuronal_dynamics_rs::run(cli.neuron_type, cli.result_folder) {
        eprintln!("{}", msg);
        exit(-1);
    }
}
