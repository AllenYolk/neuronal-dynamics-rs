use clap::ValueEnum;

#[derive(ValueEnum)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[value(rename_all = "PascalCase")]
pub enum NeuronType {
    HodgkinHuxley,
}
