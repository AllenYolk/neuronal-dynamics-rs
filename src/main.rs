use clap::Parser;
use neuronal_dynamics_rs::NeuronType;

#[derive(Parser)]
#[derive(Debug, Clone, Copy)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[arg(value_enum)]
    neuron_type: NeuronType,
}

fn main() {
    let cli = Cli::parse();
    dbg!(cli);

    neuronal_dynamics_rs::run(cli.neuron_type);
}
