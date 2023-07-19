# neuronal-dynamics-rs

`neuronal-dynamics-rs` is a tool written in Rust for simulating and visualizing the dynamics of non-spiking neurons.

## Implementation

The simulations and visualizations are accomplished using the following crates:

* Solving ODEs: [`ode_solvers`](https://crates.io/crates/ode_solvers)
* CLI: [`clap`](https://crates.io/crates/clap)
* Visualization: [`plotters`](https://crates.io/crates/plotters)

## Contents

This crate currently supports the simulation of the following models:

* **Hodgkin-Huxley Model**
    * Standard HH, Reduced Traub-Miles model, Wang-Buzsaki model
* **Morris-Lecar Model**
    * Hopf bifurcation, saddle point limit cycle, saddle point bifurcation

Furthermore, the following visualization methods are provided:

* **Membrane potential -- time**
* **Gating Variables -- time**
* **Phase plane trajectory**

Some examples are given:

* Hodgkin-Huxley standard model, oscillation
* Morris-Lecar model (Hopf bifurcation parameter setting), oscillation
