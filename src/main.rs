use clap::Parser;
use neuronal_dynamics_rs::NeuronType;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[arg(value_enum)]
    neuron: NeuronType,
}

fn main() {
    let cli = Cli::parse();
    dbg!(cli);
}
