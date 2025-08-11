# Secret Jesus Solver

The **Secret Jesus Solver** is a simulation and SAT solver for [Secret Jesus](https://secretjes.us). The game is simulated randomly and the solver tries to identify Jesus based on the played prodigies.

## How to Run

1. **Install Rust**: Ensure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/).
2. **Clone the Repository**:
   ```bash
   git clone <repository-url>
   cd secret-jesus-solver
```bash
cargo run --release -- [OPTIONS]
```
- `--players <N>`: Number of players in the simulation (default: 7).
- `--seed <SEED>`: Optional random seed for reproducibility.
- `--verbose`: Print detailed output for each game.

Example:
```bash
cargo run --release -- --players 6 --ssed 15 --verbose
```